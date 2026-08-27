BEGIN;

ALTER TABLE transform_draft
  ALTER COLUMN source_code SET DEFAULT '',
  DROP COLUMN metadata;

UPDATE transform_draft SET source_code = '' WHERE source_code IS NULL;
ALTER TABLE transform_draft ALTER COLUMN source_code SET NOT NULL;

ALTER TABLE transform
  DROP COLUMN IF EXISTS source_code,
  DROP COLUMN IF EXISTS wasm_bytecode,
  DROP COLUMN IF EXISTS metadata;

COMMIT;
