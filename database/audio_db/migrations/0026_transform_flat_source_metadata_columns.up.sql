-- The Rust data layer (PostgresTransformsDataProvider, in
-- transforms_data_provider_service.rs) has always modeled `transform`/
-- `transform_draft` with flat source_code/wasm_bytecode/metadata columns
-- directly on the row -- diverging from this migration history's earlier
-- normalized model (separate transform_port/transform_param/transform_binary
-- tables joined via get_transform_definition()). Per the decision to treat
-- the Rust code as the source of truth rather than reconcile against the
-- older normalized migrations, this brings the schema in line with what the
-- data layer has always assumed.
--
-- All three columns are nullable with no default -- NULL means "nothing
-- written yet" for a primitive's source_code/wasm_bytecode, or "not
-- applicable" for a composite's source_code (a composite has no source of
-- its own; its authored wiring graph lives nested inside metadata's JSON
-- instead, alongside the same derived ports/params any transform's metadata
-- carries -- see domain::db::db_transform::DbTransform's doc comment).
--
-- get_transform_definition() is left untouched -- it already reads through
-- transform_port/transform_param/transform_draft.source_code, none of which
-- PostgresTransformsDataProvider calls into; reconciling that pre-existing
-- drift is out of scope here.

BEGIN;

ALTER TABLE transform
  ADD COLUMN IF NOT EXISTS source_code TEXT,
  ADD COLUMN IF NOT EXISTS wasm_bytecode BYTEA,
  ADD COLUMN IF NOT EXISTS metadata TEXT;

ALTER TABLE transform_draft
  ADD COLUMN IF NOT EXISTS metadata TEXT;

-- transform_draft.source_code already exists (NOT NULL DEFAULT '', from
-- migration 0014's three-bucket model) -- relax the empty-string sentinel to
-- a real NULL, consistent with the other two columns. Safe no-ops if it's
-- already nullable/default-less.
ALTER TABLE transform_draft
  ALTER COLUMN source_code DROP NOT NULL,
  ALTER COLUMN source_code DROP DEFAULT;

COMMIT;
