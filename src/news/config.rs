use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfiguredPagination {
    pub offset: i64,
    pub limit: i64,
}

pub fn load_default_limit() -> i64 {
    dotenv().ok();
    let limit = env::var("DEFAULT_PAGINATION_LIMIT").expect("DEFAULT_PAGINATION_LIMIT must be set");

    limit
        .parse()
        .expect("DEFAULT_PAGINATION_LIMIT should be integer")
}
