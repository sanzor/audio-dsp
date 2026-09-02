-- Seeded transforms are the development catalog baseline. They are visible
-- in every workspace without creating a grant for each newly-created
-- workspace; creator-published transforms remain private by default.
BEGIN;

ALTER TABLE transform
  ADD COLUMN is_default BOOLEAN NOT NULL DEFAULT false;

CREATE INDEX idx_transform_default ON transform (transform_id)
  WHERE is_default = true;

COMMIT;
