use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::{trades};

#[derive(Debug, Queryable, Identifiable)]
pub struct Trade {
    pub id: Uuid,
    pub account_id: Uuid,
    pub order_id: Uuid,
    pub instrument_id: Uuid,
    pub price: f32,
    pub exec_qty: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "trades"]
pub struct NewTrade {
    pub id: Uuid,
    pub account_id: Uuid,
    pub order_id: Uuid,
    pub instrument_id: Uuid,
    pub price: f32,
    pub exec_qty: i32
}
