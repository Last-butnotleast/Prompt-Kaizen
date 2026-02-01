-- Add test scenario fields to feedbacks table
ALTER TABLE feedbacks
ADD COLUMN test_input TEXT,
ADD COLUMN test_actual_output TEXT,
ADD COLUMN test_expected_output TEXT;