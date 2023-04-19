pub mod stories;
pub mod users;

use axum::http::StatusCode;
pub use stories::*;
pub use users::*;

pub type Error = (StatusCode, String);

pub type Response = Result<StatusCode, Error>;

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> Error
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
