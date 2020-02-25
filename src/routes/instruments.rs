use crate::auth::Auth;
use crate::db;
use crate::db::instruments::{FindInstruments};
use crate::models::instrument::{InstrumentJson};
use crate::errors::{Errors, FieldValidator};
use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

/// return multiple instruments, ordered by most recent first
#[get("/instruments?<params..>")]
pub fn get_instruments(params: Form<FindInstruments>, auth: Option<Auth>, conn: db::Conn) -> JsonValue {
    let instruments = db::instruments::find(&conn, &params);
    let instruments_json: Vec<InstrumentJson> = instruments.0.into_iter().map(|x| x.to_json()).rev().collect();
    json!({ 
        "instruments": instruments_json, 
        "instrumentCount": instruments.1 
    })
}

#[get("/instruments/<symbol>")]
pub fn get_instrument(symbol: String, auth: Option<Auth>, conn: db::Conn) -> Option<JsonValue> {
    db::instruments::find_one(&conn, &symbol).map(|instrument| json!({ "instrument": instrument.to_json()}))
}