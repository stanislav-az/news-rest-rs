ALTER TABLE stories
  ADD COLUMN creation_timestamp timestamptz NOT NULL DEFAULT current_timestamp,
  ADD COLUMN user_id integer NOT NULL,
  ADD COLUMN category_id integer,
  ADD CONSTRAINT story_author_fkey FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE,
  ADD CONSTRAINT story_category_fkey FOREIGN KEY (category_id) REFERENCES categories (id) ON DELETE SET NULL;
