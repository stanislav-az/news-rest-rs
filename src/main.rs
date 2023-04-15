use axum::http;
use axum::routing;
use news_rest_rs::news::handlers;

#[tokio::main]
async fn main() {
    // Build our application with a single route.
    let app = axum::Router::new()
        .fallback(fallback_handler)
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
