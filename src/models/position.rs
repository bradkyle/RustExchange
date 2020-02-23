
use chrono::NaiveDateTime; //TODO change to utc
use uuid::Uuid;

use crate::schema::{orders, trades};

#[derive(Debug, Queryable, Identifiable)]
pub struct Position {
    pub id: Uuid,
    pub account_id: Uuid,
    pub instrument_id: Uuid,
    pub side: String,
    pub init_margin_req: f64,
    pub maint_margin_req: f64,
    pub leverage: f64,
    pub open_order_buy_qty: f64,
    pub open_order_buy_cost: f64,
    pub open_order_buy_premium: f64,
    pub open_order_buy_qty: f64,
    pub open_order_buy_cost: f64,
    pub open_order_buy_premium: f64,
    pub opening_qty: i64,
    pub avg_entry_price: f64,
    pub current_qty: i64,
    pub realized_pnl: i64,
    pub realized_gross_pnl: i64,
    pub unrealized_pnl: i64,
    pub init_margin: i64,
    pub maint_margin: i64,
    pub liquidation_price: f64,
    pub bankrupt_price: f64,
    pub break_even_price: f64,
    pub margin_call_price: f64,
    pub last_price: f64,
    pub mark_price: f64,
    pub last_value: f64,
    pub opened_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "positions"]
pub struct NewPosition {
    pub id: Uuid,
    pub account_id: Uuid,
    pub instrument_id: Uuid,
    pub side: String,
    pub init_margin_req: f64,
    pub maint_margin_req: f64,
    pub leverage: f64,
    pub open_order_buy_qty: f64,
    pub open_order_buy_cost: f64,
    pub open_order_buy_premium: f64,
    pub open_order_buy_qty: f64,
    pub open_order_buy_cost: f64,
    pub open_order_buy_premium: f64,
    pub opening_qty: i64,
    pub avg_entry_price: f64,
    pub current_qty: i64,
    pub realized_pnl: i64,
    pub realized_gross_pnl: i64,
    pub unrealized_pnl: i64,
    pub init_margin: i64,
    pub maint_margin: i64,
    pub liquidation_price: f64,
    pub bankrupt_price: f64,
    pub break_even_price: f64,
    pub margin_call_price: f64,
    pub last_price: f64,
    pub mark_price: f64,
    pub last_value: f64,
}

#[derive(Debug, AsChangeset)]
#[table_name = "positions"]
pub struct UpdatePosition {
    pub leverage: f64,
    pub open_order_buy_qty: f64,
    pub open_order_buy_cost: f64,
    pub open_order_buy_premium: f64,
    pub open_order_buy_qty: f64,
    pub open_order_buy_cost: f64,
    pub open_order_buy_premium: f64,
    pub opening_qty: i64,
    pub avg_entry_price: f64,
    pub current_qty: i64,
    pub realized_pnl: i64,
    pub realized_gross_pnl: i64,
    pub unrealized_pnl: i64,
    pub init_margin: i64,
    pub maint_margin: i64,
    pub liquidation_price: f64,
    pub bankrupt_price: f64,
    pub break_even_price: f64,
    pub margin_call_price: f64,
    pub last_price: f64,
    pub mark_price: f64,
    pub last_value: f64,
}
