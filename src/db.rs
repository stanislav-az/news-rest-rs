use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenv::dotenv;
use std::env;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

// this function is only for testing, use connection pool instead
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let error_msg = format!("Error connecting to db at {}", &database_url);
    PgConnection::establish(&database_url).expect(&error_msg)
}

pub fn init_connection_pool() -> ConnectionPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
