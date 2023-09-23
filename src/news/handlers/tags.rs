use axum::extract::Path;
use axum::extract::Query;
use axum::extract::State;
use axum::http;
use axum::Json;
use axum_auth::AuthBasic;
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;

use super::forbidden;
use super::internal_error;
use super::Error;
use super::Pagination;
use super::Response;
use crate::db::ConnectionPool;
use crate::news::auth::authenticate;
use crate::news::auth::authorize_admin;
use crate::news::auth::authorize_author;
use crate::news::models::NewTag;
use crate::news::models::Tag;
use crate::schema::tags;

pub async fn get_tags(
    State(pool): State<ConnectionPool>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<Vec<Tag>>, Error> {
    let mut conn = pool.get().map_err(internal_error)?;

    let pagination = pagination.configure();

    let tags: Vec<Tag> = tags::table
        .order(tags::columns::id.asc())
        .offset(pagination.offset)
        .limit(pagination.limit)
        .load::<Tag>(&mut conn)
        .map_err(internal_error)?;

    Ok(tags.into())
}

pub async fn create_tag(
    State(pool): State<ConnectionPool>,
    claims: AuthBasic,
    tag: Json<NewTag>,
) -> Response {
    let mut conn = pool.get().map_err(internal_error)?;

    let actor = authenticate(claims, &mut conn).map_err(|e| e.into_error())?;
    authorize_author(&actor).map_err(forbidden)?;

    let tag: NewTag = tag.0;

    let _res = insert_into(tags::table)
        .values(&tag)
        .execute(&mut conn)
        .map_err(internal_error)?;

    Ok(http::StatusCode::CREATED)
}

pub async fn delete_tag(
    State(pool): State<ConnectionPool>,
    claims: AuthBasic,
    Path(id_selector): Path<i32>,
) -> Response {
    let mut conn = pool.get().map_err(internal_error)?;

    let actor = authenticate(claims, &mut conn).map_err(|e| e.into_error())?;
    authorize_admin(&actor).map_err(forbidden)?;

    let num_rows = delete(tags::table.find(id_selector))
        .execute(&mut conn)
        .map_err(internal_error)?;

    if let 0 = num_rows {
        return Ok(http::StatusCode::NOT_FOUND);
    }

    Ok(http::StatusCode::NO_CONTENT)
}
