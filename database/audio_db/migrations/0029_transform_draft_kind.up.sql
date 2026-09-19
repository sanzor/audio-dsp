-- transform_draft never got a `kind` column, unlike `transform` (added in
-- 0018). The Rust data layer (PostgresTransformDraftsDataProvider) has
-- always read/written transform_draft.kind (DRAFT_ROW_COLUMNS, the INSERT in
-- insert_transform_draft) as if it existed, which fails every draft
-- creation/read with "column kind does not exist" -- this was a gap left by
-- 0026's flattening pass, not an intentional omission. Backfill from the
-- owning transform row via transform_id, matching 0018's kind CHECK.

BEGIN;

ALTER TABLE transform_draft
  ADD COLUMN kind TEXT CHECK (kind IN ('primitive', 'composite'));

UPDATE transform_draft td
SET kind = t.kind
FROM transform t
WHERE t.transform_id = td.transform_id;

ALTER TABLE transform_draft ALTER COLUMN kind SET NOT NULL;

COMMIT;
