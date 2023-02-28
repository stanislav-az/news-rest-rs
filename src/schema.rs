use diesel::prelude::*;

table! {
  stories (id) {
    id -> Int4,
    title -> Text,
    content -> Text,
  }
}
