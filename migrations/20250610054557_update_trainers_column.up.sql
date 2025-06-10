-- Add up migration script here
-- 1. Rename email column to contact_email
ALTER TABLE trainers RENAME COLUMN email TO contact_email;

-- 2. Make contact_email nullable (struct has Option<String>)
ALTER TABLE trainers ALTER COLUMN contact_email DROP NOT NULL;

-- 3. Remove pdw column (not present in current struct)
ALTER TABLE trainers DROP COLUMN pdw;
