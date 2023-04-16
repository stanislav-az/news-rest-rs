use super::internal_error;
use super::Error;
use super::Response;
use crate::db::establish_connection;
use crate::news::models::NewStory;
use crate::news::models::Story;
use crate::schema::stories;
use crate::schema::stories::dsl::*;
use axum::extract::Path;
use axum::http;
use axum::Json;
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;

pub async fn get_stories() -> Result<Json<Vec<Story>>, Error> {
    let mut conn = establish_connection();

    let news = stories::table
        .filter(is_published.eq(true))
        .order(stories::columns::id.asc())
        .load::<Story>(&mut conn)
        .map_err(internal_error)?;

    Ok(news.into())
}

pub async fn create_story(story: Json<NewStory>) -> Response {
    let story: NewStory = story.0;
    let mut conn = establish_connection();

    let _res = insert_into(stories::table)
        .values(&story)
        .execute(&mut conn)
        .map_err(internal_error)?;

    Ok(http::StatusCode::CREATED)
}

pub async fn publish_story(Path(id_selector): Path<i32>) -> Response {
    let mut conn = establish_connection();

    let num_rows = update(stories::table.find(id_selector))
        .set(is_published.eq(true))
        .execute(&mut conn)
        .map_err(internal_error)?;

    if let 0 = num_rows {
        return Ok(http::StatusCode::NOT_FOUND);
    }

    Ok(http::StatusCode::NO_CONTENT)
}

pub async fn delete_story(Path(id_selector): Path<i32>) -> Response {
    let mut conn = establish_connection();

    let num_rows = delete(stories::table.find(id_selector))
        .execute(&mut conn)
        .map_err(internal_error)?;

    if let 0 = num_rows {
        return Ok(http::StatusCode::NOT_FOUND);
    }

    Ok(http::StatusCode::NO_CONTENT)
}
