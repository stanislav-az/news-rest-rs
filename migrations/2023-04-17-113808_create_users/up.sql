CREATE TABLE users(
  id serial PRIMARY KEY,
  name varchar NOT NULL,
  login varchar NOT NULL,
  password varchar NOT NULL,
  creation_timestamp timestamptz NOT NULL DEFAULT current_timestamp,
  is_admin boolean NOT NULL DEFAULT FALSE,
  is_author boolean NOT NULL DEFAULT FALSE
)
