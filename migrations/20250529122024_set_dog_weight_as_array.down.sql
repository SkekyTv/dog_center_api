-- Add down migration script here
ALTER TABLE dogs
ALTER COLUMN weight TYPE int USING weight[1],
ALTER COLUMN weight SET DEFAULT NULL;

UPDATE dogs
SET weight = weight[1]
WHERE weight IS NOT NULL;
