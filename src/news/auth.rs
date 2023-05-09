use axum::http::StatusCode;
use axum_auth::AuthBasic;
use base64::{engine::general_purpose, Engine as _};
use diesel::prelude::*;
use diesel::result::Error as DieselErr;
use dotenv::dotenv;
use std::error::Error as StdErr;
use std::{env, fmt};
use tracing::{info, info_span, warn};

use super::handlers::Error;
use super::models::User;
use crate::{db::establish_connection, schema::users::dsl as users_dsl};
use crate::{schema::users, services::pbkdf2};

pub fn load_salt() -> pbkdf2::Salt {
    dotenv().ok();
    let salt_base64 = env::var("SALT_16_BYTES_BASE_64").expect("SALT_16_BYTES_BASE_64 must be set");
    let salt_arr = general_purpose::STANDARD.decode(salt_base64).unwrap();
    let mut salt = [0u8; 16];
    salt.copy_from_slice(&salt_arr[..16]);

    salt
}

#[derive(Debug, PartialEq)]
pub enum AuthenticationError {
    NoPasswordProvided,
    UserNotFound,
    WrongUsernameOrPassword,
    DatabaseError(DieselErr),
}

impl AuthenticationError {
    pub fn into_error(self) -> Error {
        match self {
            AuthenticationError::NoPasswordProvided => (
                StatusCode::BAD_REQUEST,
                "Bad request: no password provided".to_string(),
            ),
            AuthenticationError::UserNotFound => {
                (StatusCode::FORBIDDEN, "Wrong username".to_string())
            }
            AuthenticationError::WrongUsernameOrPassword => (
                StatusCode::FORBIDDEN,
                "Wrong username or password".to_string(),
            ),
            AuthenticationError::DatabaseError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
            }
        }
    }
}

pub fn authenticate(AuthBasic((login, password)): AuthBasic) -> Result<User, AuthenticationError> {
    let span = info_span!("Authentication");
    // TODO if this function ever becomes async entering the span will not work correctly
    let _enter = span.enter();

    let password = password.ok_or(AuthenticationError::NoPasswordProvided)?;
    let mut conn = establish_connection();

    let user: Option<User> = users::table
        .filter(users_dsl::login.eq(&login))
        .get_result(&mut conn)
        .optional()
        .map_err(AuthenticationError::DatabaseError)?;

    match user {
        None => {
            warn!("User login {} not found", login);
            Err(AuthenticationError::UserNotFound)
        }
        Some(user) => {
            info!("User id {} wants too authenticate", user.id);
            let salt = load_salt();
            let mut credential = [0u8; pbkdf2::CREDENTIAL_LEN];
            credential.copy_from_slice(&user.password[..pbkdf2::CREDENTIAL_LEN]);

            pbkdf2::verify_password(&salt, &login, &password, &credential).map_err(|_| {
                warn!("User id {} password verification failed", user.id);
                AuthenticationError::WrongUsernameOrPassword
            })?;
            info!("User id {} password verification success", user.id);

            Ok(user)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct NotAuthorized;

impl fmt::Display for NotAuthorized {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not Authorized")
    }
}

impl StdErr for NotAuthorized {}

pub fn authorize_admin(user: &User) -> Result<(), NotAuthorized> {
    if user.is_admin {
        Ok(())
    } else {
        Err(NotAuthorized)
    }
}

pub fn authorize_author(user: &User) -> Result<(), NotAuthorized> {
    if user.is_author {
        Ok(())
    } else {
        Err(NotAuthorized)
    }
}

pub fn authorize_self(user_id: i32, user: &User) -> Result<(), NotAuthorized> {
    if user.id == user_id {
        Ok(())
    } else {
        Err(NotAuthorized)
    }
}

pub fn authorize_self_or_admin(user_id: i32, user: &User) -> Result<(), NotAuthorized> {
    if user.id == user_id || user.is_admin {
        Ok(())
    } else {
        Err(NotAuthorized)
    }
}
