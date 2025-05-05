-- Add up migration script here
-- Add optional updated_at (TIMESTAMPTZ)
ALTER TABLE dogs
ADD COLUMN updated_at TIMESTAMPTZ;

-- Add created_at as a non-null TIMESTAMPTZ
ALTER TABLE dogs
ADD COLUMN created_at TIMESTAMPTZ NOT NULL DEFAULT now();


-- Add created_at as a non-null TIMESTAMPTZ
ALTER TABLE dogs
ADD COLUMN sex Sex NOT NULL DEFAULT 'Other';

CREATE TRIGGER update_updated_at
BEFORE UPDATE
ON dogs
FOR EACH ROW 
EXECUTE PROCEDURE update_updated_at_column();
