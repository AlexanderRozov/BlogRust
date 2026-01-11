-- Add avatar column to users table
ALTER TABLE users ADD COLUMN IF NOT EXISTS avatar TEXT;

-- Set default avatar for existing users (можно использовать Gravatar или другой сервис)
UPDATE users SET avatar = 'https://www.gravatar.com/avatar/00000000000000000000000000000000?d=identicon' WHERE avatar IS NULL;

