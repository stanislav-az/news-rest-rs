CREATE TABLE users(
  id serial PRIMARY KEY,
  name varchar NOT NULL,
  login varchar UNIQUE NOT NULL,
  password bytea NOT NULL,
  creation_timestamp timestamptz NOT NULL DEFAULT current_timestamp,
  is_admin boolean NOT NULL DEFAULT FALSE,
  is_author boolean NOT NULL DEFAULT FALSE
)
