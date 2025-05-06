-- Add down migration script here
-- Remove optional weight
ALTER TABLE dogs
DROP COLUMN weight;

-- Remove optional icad_id 
ALTER TABLE dogs
DROP COLUMN icad_id;
