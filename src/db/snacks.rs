use crate::db::OffsetLimit;
use crate::models::snack::{snack, snackJson};
use crate::models::user::User;
use crate::schema::snacks;
use crate::schema::favorites;
use crate::schema::follows;
use crate::schema::users;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use slug;

const SUFFIX_LEN: usize = 6;
const DEFAULT_LIMIT: i64 = 20;

#[derive(Insertable)]
#[table_name = "snacks"]
struct Newsnack<'a> {
    title: &'a str,
    description: &'a str,
    body: &'a str,
    slug: &'a str,
    author: i32,
    tag_list: &'a Vec<String>,
}

pub fn create(
    conn: &PgConnection,
    author: i32,
    title: &str,
    description: &str,
    body: &str,
    tag_list: &Vec<String>,
) -> snackJson {
    let new_snack = &Newsnack {
        title,
        description,
        body,
        author,
        tag_list,
        slug: &slugify(title),
    };

    let author = users::table
        .find(author)
        .get_result::<User>(conn)
        .expect("Error loading author");

    diesel::insert_into(snacks::table)
        .values(new_snack)
        .get_result::<snack>(conn)
        .expect("Error creating snack")
        .attach(author, false)
}

fn slugify(title: &str) -> String {
    if cfg!(feature = "random-suffix") {
        format!("{}-{}", slug::slugify(title), generate_suffix(SUFFIX_LEN))
    } else {
        slug::slugify(title)
    }
}

fn generate_suffix(len: usize) -> String {
    let mut rng = thread_rng();
    (0..len).map(|_| rng.sample(Alphanumeric)).collect()
}

#[derive(FromForm, Default)]
pub struct Findsnacks {
    tag: Option<String>,
    author: Option<String>,
    /// favorited by user
    favorited: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub fn find(conn: &PgConnection, params: &Findsnacks, user_id: Option<i32>) -> (Vec<snackJson>, i64) {
    let mut query = snacks::table
        .inner_join(users::table)
        .left_join(
            favorites::table.on(snacks::id
                .eq(favorites::snack)
                .and(favorites::user.eq(user_id.unwrap_or(0)))), // TODO: refactor
        )
        .select((
            snacks::all_columns,
            users::all_columns,
            favorites::user.nullable().is_not_null(),
        ))
        .into_boxed();
    if let Some(ref author) = params.author {
        query = query.filter(users::username.eq(author))
    }
    if let Some(ref tag) = params.tag {
        query = query.or_filter(snacks::tag_list.contains(vec![tag]))
    }
    if let Some(ref favorited) = params.favorited {
        let result = users::table
            .select(users::id)
            .filter(users::username.eq(favorited))
            .get_result::<i32>(conn);
        match result {
            Ok(id) => {
                query = query.filter(diesel::dsl::sql(&format!(
                    "snacks.id IN (SELECT favorites.snack FROM favorites WHERE favorites.user = {})",
                    id
                )));
            }
            Err(err) => match err {
                diesel::result::Error::NotFound => return (vec![], 0),
                _ => panic!("Cannot load favorited user: {}", err),
            },
        }
    }

    query
        .offset_and_limit(
            params.offset.unwrap_or(0),
            params.limit.unwrap_or(DEFAULT_LIMIT),
        )
        .load_and_count::<(snack, User, bool)>(conn)
        .map(|(res, count)| {
            (
                res.into_iter()
                    .map(|(snack, author, favorited)| snack.attach(author, favorited))
                    .collect(),
                count,
            )
        })
        .expect("Cannot load snacks")
}

pub fn find_one(conn: &PgConnection, slug: &str, user_id: Option<i32>) -> Option<snackJson> {
    let snack = snacks::table
        .filter(snacks::slug.eq(slug))
        .first::<snack>(conn)
        .map_err(|err| eprintln!("snacks::find_one: {}", err))
        .ok()?;

    let favorited = user_id
        .map(|id| is_favorite(conn, &snack, id))
        .unwrap_or(false);

    Some(populate(conn, snack, favorited))
}

#[derive(FromForm, Default)]
pub struct Feedsnacks {
    limit: Option<i64>,
    offset: Option<i64>,
}

// select * from snacks where author in (select followed from follows where follower = 7);
pub fn feed(conn: &PgConnection, params: &Feedsnacks, user_id: i32) -> Vec<snackJson> {
    snacks::table
        .filter(
            snacks::author.eq_any(
                follows::table
                    .select(follows::followed)
                    .filter(follows::follower.eq(user_id)),
            ),
        )
        .inner_join(users::table)
        .left_join(
            favorites::table.on(snacks::id
                .eq(favorites::snack)
                .and(favorites::user.eq(user_id))),
        )
        .select((
            snacks::all_columns,
            users::all_columns,
            favorites::user.nullable().is_not_null(),
        ))
        .limit(params.limit.unwrap_or(DEFAULT_LIMIT))
        .offset(params.offset.unwrap_or(0))
        .load::<(snack, User, bool)>(conn)
        .expect("Cannot load feed")
        .into_iter()
        .map(|(snack, author, favorited)| snack.attach(author, favorited))
        .collect()
}

pub fn favorite(conn: &PgConnection, slug: &str, user_id: i32) -> Option<snackJson> {
    conn.transaction::<_, diesel::result::Error, _>(|| {
        let snack = diesel::update(snacks::table.filter(snacks::slug.eq(slug)))
            .set(snacks::favorites_count.eq(snacks::favorites_count + 1))
            .get_result::<snack>(conn)?;

        diesel::insert_into(favorites::table)
            .values((
                favorites::user.eq(user_id),
                favorites::snack.eq(snack.id),
            ))
            .execute(conn)?;

        Ok(populate(conn, snack, true))
    })
    .map_err(|err| eprintln!("snacks::favorite: {}", err))
    .ok()
}

pub fn unfavorite(conn: &PgConnection, slug: &str, user_id: i32) -> Option<snackJson> {
    conn.transaction::<_, diesel::result::Error, _>(|| {
        let snack = diesel::update(snacks::table.filter(snacks::slug.eq(slug)))
            .set(snacks::favorites_count.eq(snacks::favorites_count - 1))
            .get_result::<snack>(conn)?;

        diesel::delete(favorites::table.find((user_id, snack.id))).execute(conn)?;

        Ok(populate(conn, snack, false))
    })
    .map_err(|err| eprintln!("snacks::unfavorite: {}", err))
    .ok()
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "snacks"]
pub struct UpdatesnackData {
    title: Option<String>,
    description: Option<String>,
    body: Option<String>,
    #[serde(skip)]
    slug: Option<String>,
    #[serde(rename = "tagList")]
    tag_list: Vec<String>,
}

pub fn update(
    conn: &PgConnection,
    slug: &str,
    user_id: i32,
    mut data: UpdatesnackData,
) -> Option<snackJson> {
    if let Some(ref title) = data.title {
        data.slug = Some(slugify(&title));
    }
    // TODO: check for not_found
    let snack = diesel::update(snacks::table.filter(snacks::slug.eq(slug)))
        .set(&data)
        .get_result(conn)
        .expect("Error loading snack");

    let favorited = is_favorite(conn, &snack, user_id);
    Some(populate(conn, snack, favorited))
}

pub fn delete(conn: &PgConnection, slug: &str, user_id: i32) {
    let result = diesel::delete(
        snacks::table.filter(snacks::slug.eq(slug).and(snacks::author.eq(user_id))),
    )
    .execute(conn);
    if let Err(err) = result {
        eprintln!("snacks::delete: {}", err);
    }
}

fn is_favorite(conn: &PgConnection, snack: &snack, user_id: i32) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(favorites::table.find((user_id, snack.id))))
        .get_result(conn)
        .expect("Error loading favorited")
}

fn populate(conn: &PgConnection, snack: snack, favorited: bool) -> snackJson {
    let author = users::table
        .find(snack.author)
        .get_result::<User>(conn)
        .expect("Error loading author");

    snack.attach(author, favorited)
}

pub fn tags(conn: &PgConnection) -> Vec<String> {
    snacks::table
        .select(diesel::dsl::sql("distinct unnest(tag_list)"))
        .load::<String>(conn)
        .expect("Cannot load tags")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_suffix() {
        for len in 3..9 {
            assert_eq!(generate_suffix(len).len(), len);
        }
    }
}
