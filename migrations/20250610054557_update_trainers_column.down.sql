-- Add down migration script here
-- 1. Add back pdw column (adjust data type as needed)
ALTER TABLE trainers ADD COLUMN pdw VARCHAR(255);

-- 2. Make contact_email NOT NULL again
ALTER TABLE trainers ALTER COLUMN contact_email SET NOT NULL;

-- 3. Rename contact_email column back to email
ALTER TABLE trainers RENAME COLUMN contact_email TO email;
