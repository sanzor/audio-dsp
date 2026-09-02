BEGIN;

DROP INDEX IF EXISTS idx_transform_default;
ALTER TABLE transform DROP COLUMN is_default;

COMMIT;
