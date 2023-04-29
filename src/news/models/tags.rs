use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use super::Story;
use crate::schema::*;

#[derive(Debug, PartialEq, Eq, Insertable, Deserialize, Serialize)]
#[diesel(table_name = tags)]
pub struct NewTag {
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Queryable, Selectable, Identifiable, Deserialize, Serialize)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, PartialEq, Eq, Identifiable, Selectable, Queryable, Associations, Insertable)]
#[diesel(belongs_to(Tag))]
#[diesel(belongs_to(Story))]
#[diesel(table_name = tags_stories)]
#[diesel(primary_key(tag_id, story_id))]
pub struct TagStory {
    pub tag_id: i32,
    pub story_id: i32,
}
