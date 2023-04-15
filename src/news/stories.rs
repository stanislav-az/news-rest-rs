use diesel::{Insertable, Queryable, Identifiable};
use serde::{Serialize, Deserialize};
use crate::schema::*;

#[derive(Debug, PartialEq, Eq, Insertable, Serialize, Deserialize)]
#[diesel(table_name = stories)]
pub struct NewStory {
    pub title: String,
    pub content: String,
}

#[derive(Debug, PartialEq, Eq, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = stories)]
pub struct Story {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub is_published: bool,
}
