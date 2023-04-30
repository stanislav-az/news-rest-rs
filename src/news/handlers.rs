pub mod categories;
pub mod stories;
pub mod tags;
pub mod users;

use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

pub use categories::*;
pub use stories::*;
pub use tags::*;
pub use users::*;

use super::config::{load_default_limit, ConfiguredPagination};

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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pagination {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl Pagination {
    pub fn configure(self) -> ConfiguredPagination {
        let default_limit = load_default_limit();
        ConfiguredPagination {
            offset: self.offset.unwrap_or(0),
            limit: self.limit.unwrap_or(default_limit),
        }
    }
}
