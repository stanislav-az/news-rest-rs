CREATE TABLE stories(
  id serial PRIMARY KEY,
  title varchar NOT NULL,
  content text NOT NULL,
  is_published boolean NOT NULL DEFAULT FALSE
)
