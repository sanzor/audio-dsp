-- Publish must be able to tell whether the currently-saved binary still
-- corresponds to the currently-saved source (a source-only save can leave a
-- previously-attached binary in place while source_code moves on). Snapshot
-- the exact source text a save's attached resource was compiled from,
-- alongside the other bucket-2 snapshot fields (name/description/ports/params)
-- copied at attach time. NULL means no binary has ever been attached, same as
-- wasm_bytecode.
ALTER TABLE transform_saved_state ADD COLUMN wasm_source_code TEXT;
