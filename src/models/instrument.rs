use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Queryable)]
pub struct Instrument {
    pub id: i32,
    pub symbol: String,
    pub margin_asset: String,
    pub underlying_asset: String,
    pub maker_fee: f32,
    pub taker_fee: f32,
    pub routing_fee: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentJson {
    pub id: i32,
    pub symbol: String,
    pub margin_asset: String,
    pub underlying_asset: String,
    pub maker_fee: f32,
    pub taker_fee: f32,
    pub routing_fee: f32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Instrument {
    pub fn to_json(self) -> InstrumentJson {
        InstrumentJson {
            id: self.id,
            symbol: self.symbol,
            margin_asset: self.margin_asset,
            underlying_asset: self.underlying_asset,
            maker_fee: self.maker_fee,
            taker_fee: self.taker_fee,
            routing_fee: self.routing_fee,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}