
table! {
    users (id) {
        id -> Uuid,
        email -> Varchar,
        password -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    instruments (id) {
        id -> Uuid,
        symbol -> Varchar,
        margin_asset -> Varchar,
        underlying_asset -> Varchar,
        maker_fee -> Float,
        taker_fee -> Float,
        routing_fee -> Float,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    users,
    instruments
);
