use crate::db::OffsetLimit;
use crate::models::order::{Order, OrderJson};
use crate::models::user::User;
use crate::schema::orders;
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
#[table_name = "orders"]
struct NewOrder<'a> {
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
) -> OrderJson {
    let new_order = &NewOrder {
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

    diesel::insert_into(orders::table)
        .values(new_order)
        .get_result::<Order>(conn)
        .expect("Error creating order")
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
pub struct FindOrders {
    tag: Option<String>,
    author: Option<String>,
    /// favorited by user
    favorited: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub fn find(conn: &PgConnection, params: &FindOrders, user_id: Option<i32>) -> (Vec<OrderJson>, i64) {
    let mut query = orders::table
        .inner_join(users::table)
        .left_join(
            favorites::table.on(orders::id
                .eq(favorites::order)
                .and(favorites::user.eq(user_id.unwrap_or(0)))), // TODO: refactor
        )
        .select((
            orders::all_columns,
            users::all_columns,
            favorites::user.nullable().is_not_null(),
        ))
        .into_boxed();
    if let Some(ref author) = params.author {
        query = query.filter(users::username.eq(author))
    }
    if let Some(ref tag) = params.tag {
        query = query.or_filter(orders::tag_list.contains(vec![tag]))
    }
    if let Some(ref favorited) = params.favorited {
        let result = users::table
            .select(users::id)
            .filter(users::username.eq(favorited))
            .get_result::<i32>(conn);
        match result {
            Ok(id) => {
                query = query.filter(diesel::dsl::sql(&format!(
                    "orders.id IN (SELECT favorites.order FROM favorites WHERE favorites.user = {})",
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
        .load_and_count::<(Order, User, bool)>(conn)
        .map(|(res, count)| {
            (
                res.into_iter()
                    .map(|(order, author, favorited)| order.attach(author, favorited))
                    .collect(),
                count,
            )
        })
        .expect("Cannot load orders")
}

pub fn find_one(conn: &PgConnection, slug: &str, user_id: Option<i32>) -> Option<OrderJson> {
    let order = orders::table
        .filter(orders::slug.eq(slug))
        .first::<Order>(conn)
        .map_err(|err| eprintln!("orders::find_one: {}", err))
        .ok()?;

    let favorited = user_id
        .map(|id| is_favorite(conn, &order, id))
        .unwrap_or(false);

    Some(populate(conn, order, favorited))
}

#[derive(FromForm, Default)]
pub struct FeedOrders {
    limit: Option<i64>,
    offset: Option<i64>,
}

// select * from orders where author in (select followed from follows where follower = 7);
pub fn feed(conn: &PgConnection, params: &FeedOrders, user_id: i32) -> Vec<OrderJson> {
    orders::table
        .filter(
            orders::author.eq_any(
                follows::table
                    .select(follows::followed)
                    .filter(follows::follower.eq(user_id)),
            ),
        )
        .inner_join(users::table)
        .left_join(
            favorites::table.on(orders::id
                .eq(favorites::order)
                .and(favorites::user.eq(user_id))),
        )
        .select((
            orders::all_columns,
            users::all_columns,
            favorites::user.nullable().is_not_null(),
        ))
        .limit(params.limit.unwrap_or(DEFAULT_LIMIT))
        .offset(params.offset.unwrap_or(0))
        .load::<(Order, User, bool)>(conn)
        .expect("Cannot load feed")
        .into_iter()
        .map(|(order, author, favorited)| order.attach(author, favorited))
        .collect()
}

pub fn favorite(conn: &PgConnection, slug: &str, user_id: i32) -> Option<OrderJson> {
    conn.transaction::<_, diesel::result::Error, _>(|| {
        let order = diesel::update(orders::table.filter(orders::slug.eq(slug)))
            .set(orders::favorites_count.eq(orders::favorites_count + 1))
            .get_result::<Order>(conn)?;

        diesel::insert_into(favorites::table)
            .values((
                favorites::user.eq(user_id),
                favorites::order.eq(order.id),
            ))
            .execute(conn)?;

        Ok(populate(conn, order, true))
    })
    .map_err(|err| eprintln!("orders::favorite: {}", err))
    .ok()
}

pub fn unfavorite(conn: &PgConnection, slug: &str, user_id: i32) -> Option<OrderJson> {
    conn.transaction::<_, diesel::result::Error, _>(|| {
        let order = diesel::update(orders::table.filter(orders::slug.eq(slug)))
            .set(orders::favorites_count.eq(orders::favorites_count - 1))
            .get_result::<Order>(conn)?;

        diesel::delete(favorites::table.find((user_id, order.id))).execute(conn)?;

        Ok(populate(conn, order, false))
    })
    .map_err(|err| eprintln!("orders::unfavorite: {}", err))
    .ok()
}

#[derive(Deserialize, AsChangeset, Default, Clone)]
#[table_name = "orders"]
pub struct UpdateOrderData {
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
    mut data: UpdateOrderData,
) -> Option<OrderJson> {
    if let Some(ref title) = data.title {
        data.slug = Some(slugify(&title));
    }
    // TODO: check for not_found
    let order = diesel::update(orders::table.filter(orders::slug.eq(slug)))
        .set(&data)
        .get_result(conn)
        .expect("Error loading order");

    let favorited = is_favorite(conn, &order, user_id);
    Some(populate(conn, order, favorited))
}

pub fn delete(conn: &PgConnection, slug: &str, user_id: i32) {
    let result = diesel::delete(
        orders::table.filter(orders::slug.eq(slug).and(orders::author.eq(user_id))),
    )
    .execute(conn);
    if let Err(err) = result {
        eprintln!("orders::delete: {}", err);
    }
}

fn is_favorite(conn: &PgConnection, order: &Order, user_id: i32) -> bool {
    use diesel::dsl::exists;
    use diesel::select;

    select(exists(favorites::table.find((user_id, order.id))))
        .get_result(conn)
        .expect("Error loading favorited")
}

fn populate(conn: &PgConnection, order: Order, favorited: bool) -> OrderJson {
    let author = users::table
        .find(order.author)
        .get_result::<User>(conn)
        .expect("Error loading author");

    order.attach(author, favorited)
}

pub fn tags(conn: &PgConnection) -> Vec<String> {
    orders::table
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
