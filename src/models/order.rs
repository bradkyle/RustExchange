use crate::config::DATE_FORMAT;
use crate::models::user::User;
use crate::models::instrument::Instrument;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Queryable)]
pub struct Order {
    pub id : i32,
    pub userid : i32,
    pub instrumentid : i32,
    pub side : String,
    pub ord_status : String,
    pub ord_type : String,
    pub exec_inst : String,
    pub time_in_force : String,
    pub initial_qty : i32,
    pub leaves_qty : i32,
    pub price : f32,
    pub created_at : DateTime<Utc>,
    pub updated_at : DateTime<Utc>,
}

impl Order {
    pub fn attach(self, owner: User, instrument: Instrument) -> OrderJson {
        OrderJson {
            id : self.id,
            userid : owner.id,
            instrumentid : instrument.id, // TODO add extra data
            side : self.side,
            ord_status : self.ord_status,
            ord_type : self.ord_type,
            exec_inst : self.exec_inst,
            time_in_force : self.time_in_force,
            initial_qty : self.initial_qty,
            leaves_qty : self.leaves_qty,
            price : self.price,
            created_at : self.created_at,
            updated_at : self.updated_at,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderJson {
    id : i32,
    userid : i32,
    instrumentid : i32,
    side : String,
    ord_status : String,
    ord_type : String,
    exec_inst : String,
    time_in_force : String,
    initial_qty : i32,
    leaves_qty : i32,
    price : f32,
    created_at : DateTime<Utc>,
    updated_at : DateTime<Utc>,
}
