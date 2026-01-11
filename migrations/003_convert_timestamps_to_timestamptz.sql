-- Convert all TIMESTAMP columns to TIMESTAMPTZ to match Rust chrono::DateTime<chrono::Utc>
-- This migration converts existing TIMESTAMP columns to TIMESTAMPTZ

-- Convert users.created_at
ALTER TABLE users 
ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';

-- Convert posts.published_at
ALTER TABLE posts 
ALTER COLUMN published_at TYPE TIMESTAMPTZ USING published_at AT TIME ZONE 'UTC';

-- Convert posts.created_at
ALTER TABLE posts 
ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';

-- Convert posts.updated_at
ALTER TABLE posts 
ALTER COLUMN updated_at TYPE TIMESTAMPTZ USING updated_at AT TIME ZONE 'UTC';

-- Convert comments.created_at
ALTER TABLE comments 
ALTER COLUMN created_at TYPE TIMESTAMPTZ USING created_at AT TIME ZONE 'UTC';

