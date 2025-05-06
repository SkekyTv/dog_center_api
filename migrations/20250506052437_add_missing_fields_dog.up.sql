-- Add up migration script here
-- Add optional birthdate (TIMESTAMPTZ)
ALTER TABLE dogs
ADD COLUMN weight int default null;

-- Add optional icad_id
ALTER TABLE dogs
ADD COLUMN icad_id TEXT default null;

