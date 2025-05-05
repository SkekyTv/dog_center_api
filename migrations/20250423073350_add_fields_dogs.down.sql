-- migrate:down
-- Remove optional image URL
ALTER TABLE dogs
DROP COLUMN img_url;

-- Remove races array
ALTER TABLE dogs
DROP COLUMN races;

-- Remove optional birthdate
ALTER TABLE dogs
DROP COLUMN birthdate;
