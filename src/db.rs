use diesel::prelude::*;

// TODO use config file
// CREATE USER news_rs_admin PASSWORD '000';
// CREATE DATABASE news_rs OWNER news_rs_admin;
const DATABASE_URL: &'static str = "postgres://news_rs_admin:0000@localhost/news_rs";

// TODO use connection pool:
// https://docs.rs/r2d2/latest/r2d2/
// https://blog.logrocket.com/create-backend-api-with-rust-postgres/
pub fn establish_connection() -> PgConnection {
    let error_msg = format!("Error connecting to db at {}", DATABASE_URL);
    PgConnection::establish(DATABASE_URL).expect(&error_msg)
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
