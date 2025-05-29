-- Add up migration script here
ALTER TABLE dogs
ALTER COLUMN weight TYPE int[] USING ARRAY[weight],
ALTER COLUMN weight SET DEFAULT '{}';

UPDATE dogs
SET weight = ARRAY[weight]
WHERE weight IS NOT NULL;
