use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
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
use crate::db::ConnectionPool;
use crate::news::auth::authenticate;
use crate::news::auth::authorize_admin;
use crate::news::models::nest_category;
use crate::news::models::Category;
use crate::news::models::CategoryNested;
use crate::news::models::NewCategory;
use crate::news::models::UpdatableCategory;
use crate::schema::categories;

pub async fn get_categories(
    State(pool): State<ConnectionPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<CategoryNested>>, Error> {
    let pagination = pagination.configure();
    let offset = pagination.offset.try_into().map_err(bad_request)?;
    let limit = pagination.limit.try_into().map_err(bad_request)?;

    let mut conn = pool.get().map_err(internal_error)?;

    let entries: Vec<Category> = categories::table
        .order(categories::columns::id.asc())
        .load::<Category>(&mut conn)
        .map_err(internal_error)?;

    let dict: HashMap<i32, &Category> = entries.iter().map(|c| (c.id, c)).collect();

    let cats: Vec<CategoryNested> = entries
        .iter()
        .skip(offset)
        .take(limit)
        .map(|c| nest_category(c, &dict))
        .collect();

    Ok(cats.into())
}

pub async fn create_category(
    State(pool): State<ConnectionPool>,
    claims: AuthBasic,
    category: Json<NewCategory>,
) -> Response {
    let mut conn = pool.get().map_err(internal_error)?;

    let actor = authenticate(claims, &mut conn).map_err(|e| e.into_error())?;
    authorize_admin(&actor).map_err(forbidden)?;

    let category: NewCategory = category.0;

    let _res = insert_into(categories::table)
        .values(&category)
        .execute(&mut conn)
        .map_err(internal_error)?;

    Ok(http::StatusCode::CREATED)
}

pub async fn update_category(
    State(pool): State<ConnectionPool>,
    claims: AuthBasic,
    Path(id_selector): Path<i32>,
    updated_category: Json<UpdatableCategory>,
) -> Response {
    let mut conn = pool.get().map_err(internal_error)?;

    let actor = authenticate(claims, &mut conn).map_err(|e| e.into_error())?;
    authorize_admin(&actor).map_err(forbidden)?;

    let updated_category: UpdatableCategory = updated_category.0;

    let num_rows = update(categories::table.find(id_selector))
        .set(&updated_category)
        .execute(&mut conn)
        .map_err(internal_error)?;

    if let 0 = num_rows {
        return Ok(http::StatusCode::NOT_FOUND);
    }

    Ok(http::StatusCode::NO_CONTENT)
}

pub async fn delete_category(
    State(pool): State<ConnectionPool>,
    claims: AuthBasic,
    Path(id_selector): Path<i32>,
) -> Response {
    let mut conn = pool.get().map_err(internal_error)?;

    let actor = authenticate(claims, &mut conn).map_err(|e| e.into_error())?;
    authorize_admin(&actor).map_err(forbidden)?;

    let num_rows = delete(categories::table.find(id_selector))
        .execute(&mut conn)
        .map_err(internal_error)?;

    if let 0 = num_rows {
        return Ok(http::StatusCode::NOT_FOUND);
    }

    Ok(http::StatusCode::NO_CONTENT)
}
