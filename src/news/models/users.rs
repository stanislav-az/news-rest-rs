use chrono::{DateTime, Utc};
use diesel::{AsChangeset, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use std::error::Error as StdErr;
use std::fmt;

use crate::services::pbkdf2;
use crate::{news::auth, schema::*};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSerializer {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    pub creation_timestamp: Option<DateTime<Utc>>,
    pub is_admin: Option<bool>,
    pub is_author: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserSerializerError(String);

impl fmt::Display for UserSerializerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Bad request: {}", self.0)
    }
}

impl StdErr for UserSerializerError {}

impl UserSerializer {
    pub fn from_user(user: User) -> Self {
        UserSerializer {
            id: Some(user.id),
            name: Some(user.name),
            login: Some(user.login),
            password: None,
            creation_timestamp: Some(user.creation_timestamp),
            is_admin: Some(user.is_admin),
            is_author: Some(user.is_author),
        }
    }

    pub fn try_into_updatable_user(
        self,
        login: &str,
    ) -> Result<UpdatableUser, UserSerializerError> {
        if self.id.is_some()
            || self.login.is_some()
            || self.creation_timestamp.is_some()
            || self.is_admin.is_some()
            || self.is_author.is_some()
        {
            let msg = String::from(
                "You can't modify certain user fields like id, login, is_admin, is_author, and creation_timestamp",
            );
            Err(UserSerializerError(msg))
        } else {
            let password = self.password.map(|raw_pw| {
                let salt = auth::load_salt();
                let digest = pbkdf2::hash_with_salt(&salt, login, &raw_pw);
                Vec::from(digest)
            });

            Ok(UpdatableUser {
                name: self.name,
                password,
            })
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewUserSerializer {
    pub name: String,
    pub login: String,
    pub password: String,
    pub is_admin: Option<bool>,
    pub is_author: Option<bool>,
}

impl NewUserSerializer {
    pub fn into_new_user(self) -> NewUser {
        let salt = auth::load_salt();

        let password = pbkdf2::hash_with_salt(&salt, &self.login, &self.password);
        NewUser {
            name: self.name,
            login: self.login,
            password: Vec::from(password),
            is_admin: self.is_admin,
            is_author: self.is_author,
        }
    }
}

#[derive(Debug, PartialEq, Eq, AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdatableUser {
    pub name: Option<String>,
    pub password: Option<Vec<u8>>,
}

#[derive(Debug, PartialEq, Eq, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub login: String,
    pub password: Vec<u8>,
    pub is_admin: Option<bool>,
    pub is_author: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Queryable, Identifiable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub login: String,
    pub password: Vec<u8>,
    pub creation_timestamp: DateTime<Utc>,
    pub is_admin: bool,
    pub is_author: bool,
}
