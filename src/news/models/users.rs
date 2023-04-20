use crate::schema::*;
use crate::services::pbkdf2;
use base64::{engine::general_purpose, Engine as _};
use chrono::{DateTime, Utc};
use diesel::{Identifiable, Insertable, Queryable};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

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
pub enum UserSerializerError {
    BadRequest(String),
}

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

    pub fn validate_deserialized_user(&self) -> Result<(), UserSerializerError> {
        if self.id.is_some() || self.login.is_some() || self.creation_timestamp.is_some() {
            let msg = String::from(
                "You can't modify certain user fields like id, login and creation_timestamp",
            );
            Err(UserSerializerError::BadRequest(msg))
        } else {
            Ok(())
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
        dotenv().ok();
        let salt_base64 =
            env::var("SALT_16_BYTES_BASE_64").expect("SALT_16_BYTES_BASE_64 must be set");
        let salt_arr = general_purpose::STANDARD.decode(salt_base64).unwrap();
        let mut salt = [0u8; 16];
        salt.copy_from_slice(&salt_arr[..16]);

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

#[derive(Debug, PartialEq, Eq, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub login: String,
    pub password: Vec<u8>,
    pub is_admin: Option<bool>,
    pub is_author: Option<bool>,
}

#[derive(Debug, PartialEq, Eq, Queryable, Identifiable, Serialize, Deserialize)]
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
