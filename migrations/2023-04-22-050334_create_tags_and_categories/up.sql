CREATE TABLE tags(
  id serial PRIMARY KEY,
  name varchar UNIQUE NOT NULL
);

CREATE TABLE tags_stories(
  tag_id integer,
  story_id integer,
  PRIMARY KEY (tag_id, story_id),
  FOREIGN KEY (tag_id) REFERENCES tags (id) ON DELETE CASCADE,
  FOREIGN KEY (story_id) REFERENCES stories (id) ON DELETE CASCADE
);

CREATE TABLE categories(
  id serial PRIMARY KEY,
  name varchar UNIQUE NOT NULL,
  parent_id integer DEFAULT NULL,
  CHECK (parent_id <> id),
  FOREIGN KEY (parent_id) REFERENCES categories (id) ON DELETE SET NULL
);
