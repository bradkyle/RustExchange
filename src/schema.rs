
table! {
    orders (id, account_id, instrument_id, position_id) {
        id -> Uuid,
        account_id -> Uuid,
        instrument_id -> Uuid,
        position_id -> Uuid,
        side -> Varchar, // TODO convert to enum
        initial_qty -> Integer,
        leaves_qty -> Integer,
        price -> Float,
        created_at -> Timestamp, // TODO change to UTC
        updated_at -> Timestamp,
    }
}

table! {
    instrumets (id) {
        id -> Uuid,
        symbol -> Varchar,
        maker_fee -> Float,
        taker_fee -> Float,
        routing_fee -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    positions (id, account_id, instrument_id) {
        id -> Uuid,
        account_id -> Uuid,
        instrument_id -> Uuid,
        side -> Varchar,
        init_margin_req -> Float,
        maint_margin_req -> Float,
        leverage -> Float,
        open_order_buy_qty -> Float,
        open_order_buy_cost -> Float,
        open_order_buy_premium -> Float,
        open_order_buy_qty -> Float,
        open_order_buy_cost -> Float,
        open_order_buy_premium -> Float,
        opening_qty -> Integer,
        avg_entry_price -> Float,
        current_qty -> Integer,
        realized_pnl -> Integer,
        realized_gross_pnl -> Integer,
        unrealized_pnl -> Integer,
        init_margin -> Integer,
        maint_margin -> Integer,
        liquidation_price -> Float,
        bankrupt_price -> Float,
        break_even_price -> Float,
        margin_call_price -> Float,
        last_price -> Float,
        mark_price -> Float,
        last_value -> Float,
        opened_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    trades (id, account_id, order_id, instrument_id) {
        id -> Uuid,
        account_id -> Uuid,
        order_id -> Uuid,
        instrument_id -> Uuid,
        exec_qty -> Integer,
        price -> Float,
        created_at -> Timestamp,
    }
}

table! {
    accounts (id, user_id, follower_id) {
        id -> Uuid,
        user_id -> Uuid,
        intrument_id -> Uuid,
        amount -> Integer,
        prev_realized_pnl -> Integer,
        prev_unrealized_pnl -> Integer,
        init_margin -> Integer,
        maint_margin -> Integer,
        realized_pnl -> Integer,
        unrealized_pnl -> Integer,
        wallet_balance -> Integer,
        margin_balance -> Integer,
        margin_used_pcnt -> Integer,
        excess_margin -> Integer,
        excess_margin_pcnt -> Integer,
        available_margin -> Integer,
        withdrawable_margin -> Integer,
        commission -> FLoat,
        opened_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

// TODO update
joinable!(article_tags -> articles (article_id));
joinable!(articles -> users (author_id));
joinable!(comments -> articles (article_id));
joinable!(comments -> users (user_id));
joinable!(favorite_articles -> articles (article_id));
joinable!(favorite_articles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    orders,
    instruments,
    positions,
    trades,
    accounts,
    users,
);
