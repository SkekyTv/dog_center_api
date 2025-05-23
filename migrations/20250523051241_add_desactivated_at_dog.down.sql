-- Add down migration script here
-- Remove optional weight
ALTER TABLE dogs
DROP COLUMN desactivated_at;
