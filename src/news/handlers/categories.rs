use axum::extract::Path;
use axum::extract::Query;
use axum::http;
use axum::Json;
use axum_auth::AuthBasic;
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use std::collections::HashMap;

use super::bad_request;
use super::forbidden;
use super::internal_error;
use super::Error;
use super::Pagination;
use super::Response;
use crate::db::establish_connection;
use crate::news::auth::authenticate;
use crate::news::auth::authorize_admin;
use crate::news::models::nest;
use crate::news::models::Category;
use crate::news::models::CategoryNested;
use crate::news::models::NewCategory;
use crate::news::models::UpdatableCategory;
use crate::schema::categories;

pub async fn get_categories(
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<CategoryNested>>, Error> {
    let pagination = pagination.configure();
    let offset = pagination.offset.try_into().map_err(bad_request)?;
    let limit = pagination.limit.try_into().map_err(bad_request)?;
    let mut conn = establish_connection();

    let entries: HashMap<i32, Category> = categories::table
        .order(categories::columns::id.asc())
        .load::<Category>(&mut conn)
        .map_err(internal_error)?
        .into_iter()
        .map(|c| (c.id, c))
        .collect();

    let cats: Vec<CategoryNested> = entries
        .values()
        .skip(offset)
        .take(limit)
        .map(|c| nest(c, &entries))
        .collect();

    Ok(cats.into())
}

pub async fn create_category(claims: AuthBasic, category: Json<NewCategory>) -> Response {
    let actor = authenticate(claims).map_err(|e| e.into_error())?;
    authorize_admin(&actor).map_err(forbidden)?;

    let category: NewCategory = category.0;
    let mut conn = establish_connection();

    let _res = insert_into(categories::table)
        .values(&category)
        .execute(&mut conn)
        .map_err(internal_error)?;

    Ok(http::StatusCode::CREATED)
}

pub async fn update_category(
    claims: AuthBasic,
    Path(id_selector): Path<i32>,
    updated_category: Json<UpdatableCategory>,
) -> Response {
    let actor = authenticate(claims).map_err(|e| e.into_error())?;
    authorize_admin(&actor).map_err(forbidden)?;

    let updated_category: UpdatableCategory = updated_category.0;

    let mut conn = establish_connection();

    let num_rows = update(categories::table.find(id_selector))
        .set(&updated_category)
        .execute(&mut conn)
        .map_err(internal_error)?;

    if let 0 = num_rows {
        return Ok(http::StatusCode::NOT_FOUND);
    }

    Ok(http::StatusCode::NO_CONTENT)
}

pub async fn delete_category(claims: AuthBasic, Path(id_selector): Path<i32>) -> Response {
    let actor = authenticate(claims).map_err(|e| e.into_error())?;
    authorize_admin(&actor).map_err(forbidden)?;

    let mut conn = establish_connection();

    let num_rows = delete(categories::table.find(id_selector))
        .execute(&mut conn)
        .map_err(internal_error)?;

    if let 0 = num_rows {
        return Ok(http::StatusCode::NOT_FOUND);
    }

    Ok(http::StatusCode::NO_CONTENT)
}
