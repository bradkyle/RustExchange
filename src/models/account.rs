use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::{articles, favorite_articles};

#[derive(Debug, Queryable, Identifiable)]
pub struct Acccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub intrument_id: Uuid,
    pub amount: i64,
    pub prev_realized_pnl: i64,
    pub prev_unrealized_pnl: i64,
    pub init_margin: i64,
    pub maint_margin: i64,
    pub realized_pnl: i64,
    pub unrealized_pnl: i64,
    pub wallet_balance: i64,
    pub margin_balance: i64,
    pub margin_used_pcnt: i64,
    pub excess_margin: i64,
    pub excess_margin_pcnt: i64,
    pub available_margin: i64,
    pub withdrawable_margin: i64,
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
    pub amount: i64,
    pub prev_realized_pnl: i64,
    pub prev_unrealized_pnl: i64,
    pub init_margin: i64,
    pub maint_margin: i64,
    pub realized_pnl: i64,
    pub unrealized_pnl: i64,
    pub wallet_balance: i64,
    pub margin_balance: i64,
    pub margin_used_pcnt: i64,
    pub excess_margin: i64,
    pub excess_margin_pcnt: i64,
    pub available_margin: i64,
    pub withdrawable_margin: i64,
    pub commission: f32,
}

#[derive(Debug, AsChangeset)]
#[table_name = "accounts"]
pub struct AccountChange {
    pub amount: i64,
    pub prev_realized_pnl: i64,
    pub prev_unrealized_pnl: i64,
    pub init_margin: i64,
    pub maint_margin: i64,
    pub realized_pnl: i64,
    pub unrealized_pnl: i64,
    pub wallet_balance: i64,
    pub margin_balance: i64,
    pub margin_used_pcnt: i64,
    pub excess_margin: i64,
    pub excess_margin_pcnt: i64,
    pub available_margin: i64,
    pub withdrawable_margin: i64,
    pub commission: f32,
}
