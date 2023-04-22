use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable, AsChangeset};
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewStorySerializer {
    pub title: String,
    pub content: String,
    pub category_id: Option<i32>,
}

impl NewStorySerializer {
    pub fn into_new_story(self, author_id: i32) -> NewStory {
        NewStory {
            title: self.title,
            content: self.content,
            user_id: author_id,
            category_id: self.category_id,
        }
    }
}

#[derive(Debug, PartialEq, Eq, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = stories)]
pub struct UpdatableStory {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Insertable)]
#[diesel(table_name = stories)]
pub struct NewStory {
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub category_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = stories)]
pub struct Story {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub creation_timestamp: DateTime<Utc>,
    pub user_id: i32,
    pub category_id: Option<i32>,
}
