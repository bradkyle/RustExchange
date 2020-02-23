use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::{instruments};

#[derive(Debug, Queryable, Identifiable)]
pub struct Instrument {
    pub id: Uuid,
    pub symbol: String,
    pub maker_fee: f32,
    pub taker_fee: f32,
    pub routing_fee: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "instruments"]
pub struct NewInstrument {
    pub id: Uuid,
    pub symbol: String,
    pub maker_fee: f32,
    pub taker_fee: f32,
    pub routing_fee: f32,
}

#[derive(Debug, AsChangeset)]
#[table_name = "instruments"]
pub struct InstrumentChange {
    pub maker_fee: f32,
    pub taker_fee: f32,
    pub routing_fee: f32,
}
