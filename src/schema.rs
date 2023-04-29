// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        name -> Varchar,
        parent_id -> Nullable<Int4>,
    }
}

diesel::table! {
    stories (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        is_published -> Bool,
        creation_timestamp -> Timestamptz,
        user_id -> Int4,
        category_id -> Nullable<Int4>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    tags_stories (tag_id, story_id) {
        tag_id -> Int4,
        story_id -> Int4,
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

diesel::joinable!(stories -> categories (category_id));
diesel::joinable!(stories -> users (user_id));
diesel::joinable!(tags_stories -> stories (story_id));
diesel::joinable!(tags_stories -> tags (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    stories,
    tags,
    tags_stories,
    users,
);
