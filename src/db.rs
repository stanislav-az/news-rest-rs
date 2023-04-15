use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

// TODO use connection pool:
// https://docs.rs/r2d2/latest/r2d2/
// https://blog.logrocket.com/create-backend-api-with-rust-postgres/
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    // CREATE USER news_rs_admin PASSWORD 'your_password';
    // CREATE DATABASE news_rs OWNER news_rs_admin;
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let error_msg = format!("Error connecting to db at {}", &database_url);
    PgConnection::establish(&database_url).expect(&error_msg)
}

// TODO create diesel migrations
// psql -U news_rs_admin -d news_rs -p 5432 -h 127.0.0.1
// CREATE TABLE stories(
//   id serial PRIMARY KEY,
//   title text NOT NULL,
//   content text
// );
pub fn run_migrations() {

}
