use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::{orders, trades};

// TODO implement stop orders

#[derive(Debug, Queryable, Identifiable)]
pub struct Order {
    pub id: Uuid,
    pub account_id: Uuid,
    pub instrument_id: Uuid,
    pub side: String,
    pub initial_qty: i64,
    pub leaves_qty: i64,
    pub price: f64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "orders"]
pub struct NewOrder {
    pub id: Uuid,
    pub account_id: Uuid,
    pub instrument_id: Uuid,
    pub side: String,
    pub initial_qty: i64,
    pub price: f64,
}

#[derive(Debug, AsChangeset)]
#[table_name = "orders"]
pub struct OrderAmend {
    pub side: String,
    pub leaves_qty: i64,
    pub price: f64,
}

#[derive(Debug, Insertable)]
#[table_name = "trades"]
pub struct NewTrade {
    pub id: Uuid,
    pub account_id: Uuid,
    pub order_id: Uuid,
    pub instrument_id: Uuid,
    pub price: f64,
    pub exec_qty: i64
}
