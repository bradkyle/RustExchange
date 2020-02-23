use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::{accounts};

#[derive(Debug, Queryable, Identifiable)]
pub struct Account {
    pub id: Uuid,
    pub user_id: Uuid,
    pub intrument_id: Uuid,
    pub amount: i32,
    pub prev_realized_pnl: i32,
    pub prev_unrealized_pnl: i32,
    pub init_margin: i32,
    pub maint_margin: i32,
    pub realized_pnl: i32,
    pub unrealized_pnl: i32,
    pub wallet_balance: i32,
    pub margin_balance: i32,
    pub margin_used_pcnt: i32,
    pub excess_margin: i32,
    pub excess_margin_pcnt: i32,
    pub available_margin: i32,
    pub withdrawable_margin: i32,
    pub commission: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "accounts"]
pub struct NewAccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub intrument_id: Uuid,
    pub amount: i32,
    pub prev_realized_pnl: i32,
    pub prev_unrealized_pnl: i32,
    pub init_margin: i32,
    pub maint_margin: i32,
    pub realized_pnl: i32,
    pub unrealized_pnl: i32,
    pub wallet_balance: i32,
    pub margin_balance: i32,
    pub margin_used_pcnt: i32,
    pub excess_margin: i32,
    pub excess_margin_pcnt: i32,
    pub available_margin: i32,
    pub withdrawable_margin: i32,
    pub commission: f32,
}

#[derive(Debug, AsChangeset)]
#[table_name = "accounts"]
pub struct AccountChange {
    pub amount: i32,
    pub prev_realized_pnl: i32,
    pub prev_unrealized_pnl: i32,
    pub init_margin: i32,
    pub maint_margin: i32,
    pub realized_pnl: i32,
    pub unrealized_pnl: i32,
    pub wallet_balance: i32,
    pub margin_balance: i32,
    pub margin_used_pcnt: i32,
    pub excess_margin: i32,
    pub excess_margin_pcnt: i32,
    pub available_margin: i32,
    pub withdrawable_margin: i32,
    pub commission: f32,
}
