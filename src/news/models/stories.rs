use std::collections::HashMap;

use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use super::{nest_category, Category, CategoryNested, Tag, User, UserSerializer};
use crate::schema::*;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewStorySerializer {
    pub title: String,
    pub content: String,
    pub category_id: Option<i32>,
    #[serde(default = "Vec::new")]
    pub tags: Vec<i32>,
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UpdatableStorySerializer {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category_id: Option<i32>,
    #[serde(default = "Vec::new")]
    pub tags: Vec<i32>,
}

impl UpdatableStorySerializer {
    pub fn into_updatable_story(self) -> UpdatableStory {
        UpdatableStory {
            title: self.title,
            content: self.content,
            category_id: self.category_id,
        }
    }
}

#[derive(Debug, PartialEq, Eq, AsChangeset)]
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

#[derive(
    Debug, PartialEq, Eq, Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize,
)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Category))]
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StoryNested {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    pub creation_timestamp: DateTime<Utc>,
    pub user: UserSerializer,
    pub category: Option<CategoryNested>,
    pub tags: Vec<Tag>,
}

impl StoryNested {
    pub fn from_tuple(
        t: (Story, User, Option<Category>, Vec<Tag>),
        categories_dict: &HashMap<i32, &Category>,
    ) -> StoryNested {
        let story = t.0;
        let user = t.1;
        let category = t.2.map(|c| nest_category(&c, categories_dict));
        let tags = t.3;

        StoryNested {
            id: story.id,
            title: story.title,
            content: story.content,
            is_published: story.is_published,
            creation_timestamp: story.creation_timestamp,
            user: UserSerializer::from_user(user),
            category,
            tags,
        }
    }
}
