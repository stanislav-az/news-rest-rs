// @generated automatically by Diesel CLI.

diesel::table! {
    stories (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        is_published -> Bool,
    }
}
