use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// TODO use connection pool:
// https://docs.rs/r2d2/latest/r2d2/
// https://blog.logrocket.com/create-backend-api-with-rust-postgres/
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let error_msg = format!("Error connecting to db at {}", &database_url);
    PgConnection::establish(&database_url).expect(&error_msg)
}
