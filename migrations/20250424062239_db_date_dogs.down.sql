-- Add down migration script here
-- Remove trigger
DROP TRIGGER IF EXISTS update_updated_at ON dogs;

-- Remove added columns
ALTER TABLE dogs DROP COLUMN IF EXISTS updated_at;
ALTER TABLE dogs DROP COLUMN IF EXISTS created_at;
ALTER TABLE dogs DROP COLUMN IF EXISTS sex;
