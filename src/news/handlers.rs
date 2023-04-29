pub mod categories;
pub mod stories;
pub mod tags;
pub mod users;

use axum::http::StatusCode;
pub use categories::*;
pub use stories::*;
pub use tags::*;
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

fn bad_request<E>(err: E) -> Error
where
    E: std::error::Error,
{
    (StatusCode::BAD_REQUEST, err.to_string())
}

fn forbidden<E>(err: E) -> Error
where
    E: std::error::Error,
{
    (StatusCode::FORBIDDEN, err.to_string())
}
