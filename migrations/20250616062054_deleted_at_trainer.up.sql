-- Add up migration script here
-- Add deleted_at column for soft delete functionality
ALTER TABLE trainers ADD COLUMN deleted_at timestamptz DEFAULT NULL;
