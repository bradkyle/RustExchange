use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::{orders};

// TODO implement stop orders

#[derive(Debug, Queryable, Identifiable)]
pub struct Order {
    pub id: Uuid,
    pub user_id: Uuid,
    pub account_id: Uuid,
    pub instrument_id: Uuid,
    pub side: String,
    pub status: String,
    pub initial_qty: i32,
    pub leaves_qty: i32,
    pub price: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "orders"]
pub struct NewOrder {
    pub id: Uuid,
    pub user_id: Uuid,
    pub instrument_id: Uuid,
    pub side: String,
    pub initial_qty: i32,
    pub leaves_qty: i32,
    pub price: f32,
}

#[derive(Debug, AsChangeset)]
#[table_name = "orders"]
pub struct UpdateOrder {
    pub side: String,
    pub leaves_qty: i32,
    pub price: f32,
}
