table! {
    articles (id) {
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
    comments (id) {
        id -> Int4,
        body -> Text,
        article -> Int4,
        author -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    favorites (user, article) {
        user -> Int4,
        article -> Int4,
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
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        hash -> Text,
    }
}

joinable!(articles -> users (author));
joinable!(comments -> articles (article));
joinable!(comments -> users (author));
joinable!(favorites -> articles (article));
joinable!(favorites -> users (user));
joinable!(orders -> instruments (instrumentid));
joinable!(orders -> users (userid));

allow_tables_to_appear_in_same_query!(
    articles,
    comments,
    favorites,
    follows,
    instruments,
    orders,
    users,
);
