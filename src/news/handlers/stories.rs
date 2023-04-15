use axum::http;
use axum::Json;
use diesel::insert_into;
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::news::models::NewStory;
use crate::news::models::Story;
use crate::schema::stories;
use crate::schema::stories::dsl::*;

pub async fn get_stories() -> Json<Vec<Story>> {
    let mut conn = establish_connection();

    let news = stories::table
        // .filter(is_published.eq(true))
        .order(stories::columns::id.asc())
        .load::<Story>(&mut conn)
        .unwrap();

    news.into()
}

pub async fn create_story(story: Json<NewStory>) -> impl axum::response::IntoResponse {
    let story: NewStory = story.0;
    let mut conn = establish_connection();

    let _res = insert_into(stories::table)
        .values(&story)
        .execute(&mut conn);

    http::StatusCode::CREATED
}
