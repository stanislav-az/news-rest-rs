use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

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
