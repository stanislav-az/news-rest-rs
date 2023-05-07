use axum::http;
use axum::routing;
use news_rest_rs::news::handlers;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .fallback(fallback_handler)
        .route("/api/tags/:id", routing::delete(handlers::delete_tag))
        .route("/api/tags", routing::post(handlers::create_tag))
        .route("/api/tags", routing::get(handlers::get_tags))
        .route("/api/categories/:id", routing::delete(handlers::delete_category).patch(handlers::update_category))
        .route("/api/categories", routing::post(handlers::create_category))
        .route("/api/categories", routing::get(handlers::get_categories))
        .route("/api/users/:id", routing::patch(handlers::update_user))
        .route("/api/users/:id", routing::delete(handlers::delete_user))
        .route("/api/users", routing::post(handlers::create_user))
        .route("/api/users", routing::get(handlers::get_users))
        .route("/api/stories/publish/:id", routing::patch(handlers::publish_story))
        .route("/api/stories/search/:search_query", routing::get(handlers::search_stories))
        .route("/api/stories/:id", routing::delete(handlers::delete_story).get(handlers::get_story).patch(handlers::update_story))
        .route("/api/stories", routing::post(handlers::create_story))
        .route("/api/stories", routing::get(handlers::get_stories))
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
