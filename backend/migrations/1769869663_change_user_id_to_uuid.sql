-- Step 1: Remove the index temporarily
DROP INDEX IF EXISTS idx_prompts_user_id;

-- Step 2: Delete any rows with 'system' as user_id (or convert to a valid UUID)
-- Option A: Delete rows with 'system'
DELETE FROM prompts WHERE user_id = 'system';

-- Option B: If you want to keep existing data, use a placeholder UUID instead
-- UPDATE prompts SET user_id = '00000000-0000-0000-0000-000000000000' WHERE user_id = 'system';

-- Step 3: Change column type from TEXT to UUID
ALTER TABLE prompts
ALTER COLUMN user_id TYPE UUID
    USING user_id::UUID;

-- Step 4: Recreate the index
CREATE INDEX idx_prompts_user_id ON prompts(user_id);

-- Step 5: Add foreign key constraint if you have a users table
-- Uncomment if you have a users table in Supabase
-- ALTER TABLE prompts
--     ADD CONSTRAINT fk_prompts_user_id
--     FOREIGN KEY (user_id)
--     REFERENCES auth.users(id)
--     ON DELETE CASCADE;