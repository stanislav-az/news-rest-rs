use axum::http;
use axum::routing;
use dotenv::dotenv;
use std::env;
use std::fs::File;
use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::DefaultOnFailure;
use tower_http::trace::DefaultOnRequest;
use tower_http::trace::DefaultOnResponse;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use news_rest_rs::news::handlers;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let log_to_stdout: bool = env::var("LOG_TO_STDOUT")
        .expect("LOG_TO_STDOUT must be set")
        .parse()
        .expect("LOG_TO_STDOUT should be boolean: true/false");
    let log_to_stdout = if log_to_stdout {
        Some(tracing_subscriber::fmt::layer())
    } else {
        None
    };
    let log_to_file = env::var("LOG_TO_FILE").ok();
    let log_to_file = log_to_file.map_or(None, |path| {
        if !path.is_empty() {
            let f = File::options()
                .append(true)
                .create(true)
                .open(&path)
                .expect(&format!("Could not open log file on path {}", &path));
            Some(tracing_subscriber::fmt::layer().json().with_writer(f))
        } else {
            None
        }
    });

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                "news_rest_rs=trace,tower_http=trace,axum::rejection=trace".into()
            }),
        )
        .with(log_to_stdout)
        .with(log_to_file)
        .init();

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
        .route("/", routing::get(|| async { "Hello, World!" }))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_failure(DefaultOnFailure::new().level(Level::WARN)),
        );

    info!("Starting axum server at http://localhost:3000. To quit hit ctrl+c.");

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
