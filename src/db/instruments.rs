use actix::prelude::*;
use blob_uuid::to_blob;
use diesel::prelude::*;
use slug::slugify;
use uuid::Uuid;

use super::{DbExecutor, PooledConn};
use crate::app::instruments::{
    InstrumentListResponse, InstrumentResponse, InstrumentResponseInner, CreateInstrumentOuter, CancelInstrument,
    GetInstrument, GetInstruments
};
use crate::app::profiles::ProfileResponseInner;
use crate::models::{
    Instrument, InstrumentAmend, NewTrade, NewInstrument, User,
};
use crate::prelude::*;
use crate::utils::CustomDateTime;

// message handler implementations ↓

impl Message for CreateInstrumentOuter {
    type Result = Result<InstrumentResponse>;
}

// Implement request handlers
impl Handler<CreateInstrumentOuter> for DbExecutor {
    type Result = Result<InstrumentResponse>;

    fn handle(&mut self, msg: CreateInstrumentOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::instruments;

        let conn = &self.0.get()?;

        let author = msg.auth.user;

        // Generating the Uuid here since it will help make a unique slug
        // This is for when some instruments may have similar titles such that they generate the same slug
        let new_instrument_id = Uuid::new_v4();
        let slug = generate_slug(&new_instrument_id, &msg.instrument.title);

        let new_instrument = NewInstrument {
            id: new_instrument_id,
            author_id: author.id,
            slug,
            title: msg.instrument.title,
            description: msg.instrument.description,
            body: msg.instrument.body,
        };
        let instrument = diesel::insert_into(instruments::table)
            .values(&new_instrument)
            .get_result::<Instrument>(conn)?;

        let _ = replace_tags(instrument.id, msg.instrument.tag_list, conn)?;

        get_instrument_response(instrument.slug, Some(instrument.author_id), conn)
    }
}

impl Message for GetInstrument {
    type Result = Result<InstrumentResponse>;
}

impl Handler<GetInstrument> for DbExecutor {
    type Result = Result<InstrumentResponse>;

    fn handle(&mut self, msg: GetInstrument, _: &mut Self::Context) -> Self::Result {
        let conn = &self.0.get()?;

        match msg.auth {
            Some(auth) => get_instrument_response(msg.slug, Some(auth.user.id), conn),
            None => get_instrument_response(msg.slug, None, conn),
        }
    }
}

impl Message for AmendInstrumentOuter {
    type Result = Result<InstrumentResponse>;
}

impl Handler<AmendInstrumentOuter> for DbExecutor {
    type Result = Result<InstrumentResponse>;

    fn handle(&mut self, msg: AmendInstrumentOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::instruments;

        let conn = &self.0.get()?;

        let instrument = instruments::table
            .filter(instruments::slug.eq(msg.slug))
            .get_result::<Instrument>(conn)?;

        if msg.auth.user.id != instrument.author_id {
            return Err(Error::Forbidden(json!({
                "error": "user is not the owner of instrument in question",
            })));
        }

        let slug = match &msg.instrument.title {
            Some(title) => Some(generate_slug(&instrument.id, &title)),
            None => None,
        };

        let instrument_change = InstrumentChange {
            slug,
            title: msg.instrument.title,
            description: msg.instrument.description,
            body: msg.instrument.body,
        };

        let instrument = diesel::update(instruments::table.find(instrument.id))
            .set(&instrument_change)
            .get_result::<Instrument>(conn)?;

        let _ = match msg.instrument.tag_list {
            Some(tags) => {
                let inserted_tags = replace_tags(instrument.id, tags, conn)?;
                inserted_tags
                    .iter()
                    .map(|instrument_tag| instrument_tag.tag_name.to_owned())
                    .collect::<Vec<String>>()
            }
            None => select_tags_on_instrument(instrument.id, conn)?,
        };

        get_instrument_response(instrument.slug, Some(instrument.author_id), conn)
    }
}

impl Message for CancelInstrument {
    type Result = Result<()>;
}

impl Handler<CancelInstrument> for DbExecutor {
    type Result = Result<()>;

    fn handle(&mut self, msg: CancelInstrument, _: &mut Self::Context) -> Self::Result {
        use crate::schema::instruments;

        let conn = &self.0.get()?;

        let instrument = instruments::table
            .filter(instruments::slug.eq(msg.slug))
            .get_result::<Instrument>(conn)?;

        if msg.auth.user.id != instrument.author_id {
            return Err(Error::Forbidden(json!({
                "error": "user is not the author of instrument in question",
            })));
        }

        delete_tags(instrument.id, conn)?;

        delete_favorites(instrument.id, conn)?;

        match diesel::delete(instruments::table.filter(instruments::id.eq(instrument.id))).execute(conn) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

impl Message for GetInstruments {
    type Result = Result<InstrumentListResponse>;
}

impl Handler<GetInstruments> for DbExecutor {
    type Result = Result<InstrumentListResponse>;

    fn handle(&mut self, msg: GetInstruments, _: &mut Self::Context) -> Self::Result {
        use crate::schema::{instruments, users};

        let conn = &self.0.get()?;

        let mut query = instruments::table.into_boxed();

        if let Some(ref author_name) = msg.params.author {
            let instruments_by_author = instruments::table
                .inner_join(users::table)
                .filter(users::username.eq(author_name))
                .select(instruments::id)
                .load::<Uuid>(conn)?;

            query = query.filter(instruments::id.eq_any(instruments_by_author));
        }

        if let Some(ref username_favorited_by) = msg.params.favorited {
            use crate::schema::favorite_instruments;

            let favorite_instrument_ids: Vec<Uuid> = favorite_instruments::table
                .inner_join(users::table)
                .filter(users::username.eq(username_favorited_by))
                .select(favorite_instruments::instrument_id)
                .load::<Uuid>(conn)?;

            query = query.filter(instruments::id.eq_any(favorite_instrument_ids));
        }

        if let Some(ref tag) = msg.params.tag {
            use crate::schema::instrument_tags;

            let tagged_instrument_ids: Vec<Uuid> = instrument_tags::table
                .filter(instrument_tags::tag_name.eq(tag))
                .select(instrument_tags::instrument_id)
                .load::<Uuid>(conn)?;

            query = query.filter(instruments::id.eq_any(tagged_instrument_ids));
        }

        let limit = std::cmp::min(msg.params.limit.unwrap_or(20), 100) as i64;
        let offset = msg.params.offset.unwrap_or(0) as i64;

        let matched_instruments = query
            .instrument(instruments::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load::<Instrument>(conn)?;

        match msg.auth {
            Some(auth) => get_instrument_list_response(matched_instruments, Some(auth.user.id), conn),
            None => get_instrument_list_response(matched_instruments, None, conn),
        }
    }
}

// local helper methods ↓

fn generate_slug(uuid: &Uuid, title: &str) -> String {
    format!("{}-{}", to_blob(uuid), slugify(title))
}

// This will reduce the amount of boilerplate when an InstrumentResponse is needed
fn get_instrument_response(
    slug: String,
    user_id: Option<Uuid>,
    conn: &PooledConn,
) -> Result<InstrumentResponse> {
    use crate::schema::{instruments, users};

    let (instrument, author) = instruments::table
        .inner_join(users::table)
        .filter(instruments::slug.eq(slug))
        .get_result::<(Instrument, User)>(conn)?;

    let (favorited, following) = match user_id {
        Some(user_id) => get_favorited_and_following(instrument.id, author.id, user_id, conn)?,
        None => (false, false),
    };

    let favorites_count = get_favorites_count(instrument.id, conn)?;

    let tags = select_tags_on_instrument(instrument.id, conn)?;

    Ok(InstrumentResponse {
        instrument: InstrumentResponseInner {
            slug: instrument.slug,
            title: instrument.title,
            description: instrument.description,
            body: instrument.body,
            tag_list: tags,
            created_at: CustomDateTime(instrument.created_at),
            updated_at: CustomDateTime(instrument.updated_at),
            favorited,
            favorites_count,
            author: ProfileResponseInner {
                username: author.username,
                bio: author.bio,
                image: author.image,
                following,
            },
        },
    })
}

fn get_instrument_list_response(
    instruments: Vec<Instrument>,
    user_id: Option<Uuid>,
    conn: &PooledConn,
) -> Result<InstrumentListResponse> {
    let instrument_list = instruments
        .iter()
        .map(
            |instrument| match get_instrument_response(instrument.slug.to_owned(), user_id, conn) {
                Ok(response) => Ok(response.instrument),
                Err(e) => Err(e),
            },
        )
        .collect::<Result<Vec<InstrumentResponseInner>>>()?;

    Ok(InstrumentListResponse {
        instruments_count: instrument_list.len(),
        instruments: instrument_list,
    })
}
