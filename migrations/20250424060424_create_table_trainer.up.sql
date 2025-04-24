-- Add up migration script here
DO $$ BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'sex') THEN
        CREATE TYPE sex AS ENUM ('F', 'M', 'Other');
    END IF;
END $$;
-- stored function to update date on UPDATE
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
NEW.updated_at = now();
RETURN NEW;
END;
$$ language 'plpgsql';

-- create table
create table trainers (
  id UUID PRIMARY KEY,
  email text NOT NULL UNIQUE,
  pdw text NOT NULL,
  name text NOT NULL,
  sex sex NOT NULL,
  phone_number text UNIQUE,
  img_url text,
  birthdate timestamptz,
  created_at timestamptz NOT NULL DEFAULT now(),
  updated_at timestamptz
);

-- add TRIGGER
CREATE TRIGGER update_modified_time
BEFORE UPDATE
ON trainers
FOR EACH ROW 
EXECUTE PROCEDURE update_updated_at_column();
