
use chrono::NaiveDateTime; //TODO change to utc
use uuid::Uuid;

use crate::schema::{positions};

#[derive(Debug, Queryable, Identifiable)]
pub struct Position {
    pub id: Uuid,
    pub account_id: Uuid,
    pub instrument_id: Uuid,
    pub side: String,
    pub init_margin_req: f32,
    pub maint_margin_req: f32,
    pub leverage: f32,
    pub open_order_buy_qty: f32,
    pub open_order_buy_cost: f32,
    pub open_order_buy_premium: f32,
    pub open_order_sell_qty: f32,
    pub open_order_sell_cost: f32,
    pub open_order_sell_premium: f32,
    pub opening_qty: i32,
    pub avg_entry_price: f32,
    pub current_qty: i32,
    pub realized_pnl: i32,
    pub realized_gross_pnl: i32,
    pub unrealized_pnl: i32,
    pub init_margin: i32,
    pub maint_margin: i32,
    pub liquidation_price: f32,
    pub bankrupt_price: f32,
    pub break_even_price: f32,
    pub margin_call_price: f32,
    pub last_price: f32,
    pub mark_price: f32,
    pub last_value: f32,
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
    pub init_margin_req: f32,
    pub maint_margin_req: f32,
    pub leverage: f32,
    pub open_order_buy_qty: f32,
    pub open_order_buy_cost: f32,
    pub open_order_buy_premium: f32,
    pub open_order_sell_qty: f32,
    pub open_order_sell_cost: f32,
    pub open_order_sell_premium: f32,
    pub opening_qty: i32,
    pub avg_entry_price: f32,
    pub current_qty: i32,
    pub realized_pnl: i32,
    pub realized_gross_pnl: i32,
    pub unrealized_pnl: i32,
    pub init_margin: i32,
    pub maint_margin: i32,
    pub liquidation_price: f32,
    pub bankrupt_price: f32,
    pub break_even_price: f32,
    pub margin_call_price: f32,
    pub last_price: f32,
    pub mark_price: f32,
    pub last_value: f32,
}

#[derive(Debug, AsChangeset)]
#[table_name = "positions"]
pub struct UpdatePosition {
    pub leverage: f32,
    pub open_order_buy_qty: f32,
    pub open_order_buy_cost: f32,
    pub open_order_buy_premium: f32,
    pub open_order_sell_qty: f32,
    pub open_order_sell_cost: f32,
    pub open_order_sell_premium: f32,
    pub opening_qty: i32,
    pub avg_entry_price: f32,
    pub current_qty: i32,
    pub realized_pnl: i32,
    pub realized_gross_pnl: i32,
    pub unrealized_pnl: i32,
    pub init_margin: i32,
    pub maint_margin: i32,
    pub liquidation_price: f32,
    pub bankrupt_price: f32,
    pub break_even_price: f32,
    pub margin_call_price: f32,
    pub last_price: f32,
    pub mark_price: f32,
    pub last_value: f32,
}
