table! {
    comments (id) {
        id -> Int4,
        body -> Text,
        snack -> Int4,
        author -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    favorites (user, snack) {
        user -> Int4,
        snack -> Int4,
    }
}

table! {
    follows (follower, followed) {
        follower -> Int4,
        followed -> Int4,
    }
}

table! {
    instruments (id) {
        id -> Int4,
        symbol -> Text,
        margin_asset -> Text,
        underlying_asset -> Text,
        maker_fee -> Float4,
        taker_fee -> Float4,
        routing_fee -> Float4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    orders (id) {
        id -> Int4,
        userid -> Int4,
        instrumentid -> Int4,
        side -> Text,
        ord_status -> Text,
        ord_type -> Text,
        exec_inst -> Text,
        time_in_force -> Text,
        initial_qty -> Int4,
        leaves_qty -> Int4,
        price -> Float4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    snacks (id) {
        id -> Int4,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        author -> Int4,
        tag_list -> Array<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        favorites_count -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        hash -> Text,
    }
}

joinable!(comments -> snacks (snack));
joinable!(comments -> users (author));
joinable!(favorites -> snacks (snack));
joinable!(favorites -> users (user));
joinable!(orders -> instruments (instrumentid));
joinable!(orders -> users (userid));
joinable!(snacks -> users (author));

allow_tables_to_appear_in_same_query!(
    comments,
    favorites,
    follows,
    instruments,
    orders,
    snacks,
    users,
);
