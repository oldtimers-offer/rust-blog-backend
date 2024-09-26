// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        created_at -> Timestamp,
        author_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
        is_author -> Bool,
    }
}

diesel::joinable!(posts -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(posts, users,);
