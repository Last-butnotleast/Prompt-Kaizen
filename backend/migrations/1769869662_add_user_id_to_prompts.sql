-- Add user_id column to prompts table
ALTER TABLE prompts ADD COLUMN user_id TEXT NOT NULL DEFAULT 'system';

-- Create index for faster user-based queries
CREATE INDEX idx_prompts_user_id ON prompts(user_id);

-- Remove default after migration (for new inserts to require user_id)
ALTER TABLE prompts ALTER COLUMN user_id DROP DEFAULT;