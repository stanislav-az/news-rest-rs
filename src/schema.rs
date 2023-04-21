// @generated automatically by Diesel CLI.

diesel::table! {
    stories (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        is_published -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        login -> Varchar,
        password -> Bytea,
        creation_timestamp -> Timestamptz,
        is_admin -> Bool,
        is_author -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    stories,
    users,
);
