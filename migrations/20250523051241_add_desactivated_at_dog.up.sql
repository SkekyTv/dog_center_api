-- Add up migration script here
-- Add optional birthdate (TIMESTAMPTZ)
ALTER TABLE dogs
ADD COLUMN desactivated_at TIMESTAMPTZ default null;
