ALTER TABLE stories
  DROP CONSTRAINT story_author_fkey,
  DROP CONSTRAINT story_category_fkey,
  DROP COLUMN creation_timestamp,
  DROP COLUMN user_id,
  DROP COLUMN category_id;
