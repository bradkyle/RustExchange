use crate::db::OffsetLimit;
use crate::models::instrument::{Instrument};
use crate::schema::instruments;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

const SUFFIX_LEN: usize = 6;
const DEFAULT_LIMIT: i64 = 20; //TODO make global

#[derive(FromForm, Default)]
pub struct FindInstruments {
    margin_asset: Option<String>,
    underlying_asset: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

// Finds all instruments that match the given params else 
// returns all instruments 
pub fn find(conn: &PgConnection, params: &FindInstruments) -> (Vec<Instrument>, i64) {

    let mut query = instruments::table
        .select(instruments::all_columns)
        .into_boxed();

    if let Some(ref margin_asset) = params.margin_asset {
        query = query.filter(instruments::margin_asset.eq(margin_asset))
    }

    if let Some(ref underlying_asset) = params.underlying_asset {
        query = query.filter(instruments::underlying_asset.eq(underlying_asset))
    }

    query
        .offset_and_limit(
            params.offset.unwrap_or(0),
            params.limit.unwrap_or(DEFAULT_LIMIT),
        )
        .load_and_count::<Instrument>(conn)
        .map(|(res, count)| {
            (
                res,
                count,
            )
        })
        .expect("Cannot load instruments")
}

pub fn find_one(conn: &PgConnection, symbol: &str) -> Option<Instrument> {
    let instrument = instruments::table
        .filter(instruments::symbol.eq(symbol))
        .first::<Instrument>(conn)
        .map_err(|err| eprintln!("instruments::find_one: {}", err))
        .ok()?;

    Some(instrument)
}


#[cfg(test)]
mod tests {
    

}
