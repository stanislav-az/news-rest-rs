use std::collections::HashMap;

use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::*;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryNested {
    pub id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<CategoryNested>>,
}

pub fn nest(origin: &Category, flat_dict: &HashMap<i32, Category>) -> CategoryNested {
    let parent = origin.parent_id.map(|pid| {
        let flat_parent = &flat_dict[&pid];
        Box::new(nest(flat_parent, flat_dict))
    });

    CategoryNested {
        id: origin.id,
        name: origin.name.clone(),
        parent,
    }
}

#[derive(Debug, PartialEq, Eq, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = categories)]
pub struct UpdatableCategory {
    pub name: Option<String>,
    pub parent_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Insertable, Serialize, Deserialize)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub name: String,
    pub parent_id: Option<i32>,
}

#[derive(Debug, PartialEq, Eq, Queryable, Identifiable)]
#[diesel(table_name = categories)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
}
