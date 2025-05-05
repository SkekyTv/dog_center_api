-- Add down migration script here
DROP TRIGGER IF EXISTS update_modified_time ON trainers;
DROP FUNCTION IF EXISTS update_updated_at_column;
DROP TABLE IF EXISTS trainers;
DO $$ BEGIN
    IF EXISTS (SELECT 1 FROM pg_type WHERE typname = 'sex') THEN
        DROP TYPE sex;
    END IF;
END $$;
