-- Add down migration script here
-- Remove deleted_at column
ALTER TABLE trainers DROP COLUMN deleted_at;
