-- migrate:up
-- Add optional birthdate (TIMESTAMPTZ)
ALTER TABLE dogs
ADD COLUMN birthdate TIMESTAMPTZ;

-- Add races as a non-null array of text
-- Default to empty array for existing rows
ALTER TABLE dogs
ADD COLUMN races TEXT[] NOT NULL DEFAULT '{}';

-- Add optional image URL
ALTER TABLE dogs
ADD COLUMN img_url TEXT;

