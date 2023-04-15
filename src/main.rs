use axum::http;
use axum::routing;
use axum::Json;
use diesel::insert_into;
use diesel::prelude::*;
use news_rest_rs::db::establish_connection;
use news_rest_rs::news::stories::NewStory;
use news_rest_rs::news::stories::Story;
use news_rest_rs::schema::stories;
use news_rest_rs::schema::stories::dsl::*;

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new()
        .fallback(fallback_handler)
        .route("/api/stories", routing::post(create_story))
        .route("/api/stories", routing::get(get_stories))
        .route("/", routing::get(|| async { "Hello, World!" }));

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// axum handler for any request that fails to match the router routes.
/// This implementation returns HTTP status code Not Found (404).
pub async fn fallback_handler(uri: http::Uri) -> impl axum::response::IntoResponse {
    (http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

// TODO spawn a tokio task for db query or use diesel-async ?
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
