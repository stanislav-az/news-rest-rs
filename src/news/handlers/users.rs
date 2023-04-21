use axum::extract::Path;
use axum::http;
use axum::Json;
use axum_auth::AuthBasic;
use diesel::delete;
use diesel::dsl::not;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;

use super::bad_request;
use super::forbidden;
use super::internal_error;
use super::Error;
use super::Response;
use crate::db::establish_connection;
use crate::news::auth::authenticate;
use crate::news::auth::authorize_admin;
use crate::news::auth::authorize_self;
use crate::news::auth::authorize_self_or_admin;
use crate::news::models::NewUser;
use crate::news::models::NewUserSerializer;
use crate::news::models::User;
use crate::news::models::UserSerializer;
use crate::schema::users;

pub async fn get_users() -> Result<Json<Vec<UserSerializer>>, Error> {
    let mut conn = establish_connection();

    let news: Vec<User> = users::table
        .order(users::columns::id.asc())
        .load::<User>(&mut conn)
        .map_err(internal_error)?;

    let news: Vec<UserSerializer> = news.into_iter().map(UserSerializer::from_user).collect();

    Ok(news.into())
}

pub async fn create_user(claims: AuthBasic, user: Json<NewUserSerializer>) -> Response {
    let actor = authenticate(claims).map_err(|e| e.into_error())?;
    authorize_admin(&actor).map_err(forbidden)?;

    let user: NewUserSerializer = user.0;
    let user: NewUser = user.into_new_user();
    let mut conn = establish_connection();

    let _res = insert_into(users::table)
        .values(&user)
        .execute(&mut conn)
        .map_err(internal_error)?;

    Ok(http::StatusCode::CREATED)
}

pub async fn update_user(
    claims: AuthBasic,
    Path(id_selector): Path<i32>,
    updated_user: Json<UserSerializer>,
) -> Response {
    let actor = authenticate(claims).map_err(|e| e.into_error())?;
    authorize_self(id_selector, &actor).map_err(forbidden)?;

    let mut conn = establish_connection();

    let updated_user: UserSerializer = updated_user.0;

    let user: Option<User> = users::table
        .find(id_selector)
        .get_result(&mut conn)
        .optional()
        .map_err(internal_error)?;

    match user {
        None => return Ok(http::StatusCode::NOT_FOUND),
        Some(user) => {
            let updated_user = updated_user
                .try_into_updatable_user(&user.login)
                .map_err(bad_request)?;

            update(&user)
                .set(updated_user)
                .execute(&mut conn)
                .map_err(internal_error)?;
        }
    }

    Ok(http::StatusCode::NO_CONTENT)
}

pub async fn delete_user(claims: AuthBasic, Path(id_selector): Path<i32>) -> Response {
    let actor = authenticate(claims).map_err(|e| e.into_error())?;
    authorize_self_or_admin(id_selector, &actor).map_err(forbidden)?;

    let mut conn = establish_connection();

    let num_rows = delete(
        users::table
            .filter(users::id.eq(id_selector))
            .filter(not(users::login.eq("admin"))), // admin user is untouchable
    )
    .execute(&mut conn)
    .map_err(internal_error)?;

    if let 0 = num_rows {
        return Ok(http::StatusCode::NOT_FOUND);
    }

    Ok(http::StatusCode::NO_CONTENT)
}
