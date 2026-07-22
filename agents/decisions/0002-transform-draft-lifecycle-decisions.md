# 0002: Transform draft lifecycle — republish, storage, publish gate, deletion

- **Status:** Accepted
- **Date:** 2026-07-22

## Context

`features/transform_drafts.md` (the original rough requirement for the compile/save/publish flow) was refined via a joint `business-analyst` + `product-owner` pass. The three-bucket model it describes was already designed in detail in `agents/transforms.md` and found to be substantially implemented (backend compiles clean, frontend typechecks clean) — this decision does not change that model's shape, it settles four points the original draft and `agents/transforms.md` left open:

1. What happens when Publish is hit on a transform that's already published (no version pin exists anywhere — editor graphs reference a transform by bare ID inside `graph_state` JSONB).
2. Whether "different in db and storage" (draft's wording) requires real object-storage separation, given all three buckets are currently Postgres BYTEA columns.
3. Whether Publish should be blocked when the saved binary (`transform_saved_state.wasm_bytecode`) no longer corresponds to the saved source (`transform_saved_state.source_code`) — possible today because a source-only save doesn't touch a previously-attached binary.
4. Whether a creator can delete/abandon a transform that was created but never published — no such path exists today.

Full BA/PO findings are not reproduced here; this file records only what was decided. See conversation history for the full gap analysis if needed.

## Decision

1. **Republish overwrites in place.** No versioning, no compatibility gate. This is an accepted risk, not an oversight: editor graphs currently have no version pin, so a republish that changes a transform's ports/params can silently affect other users' existing graphs. Revisit if this causes real incidents — a versioned catalog would be a separate, larger decision.
2. **Storage separation stays table-level, not storage-tier-level.** The draft's "different in db and storage" is satisfied by the existing separate tables (`transform_resources`, `transform_saved_state`, `transform_binaries`). No object-storage/blob-store backend is required.
3. **Publish is blocked on source/binary mismatch.** `POST /transforms/{id}/publish` must fail validation if `transform_saved_state.wasm_bytecode` is not null and its recorded source does not match `transform_saved_state.source_code` byte-for-byte. This closes the gap where a live transform's advertised source could diverge from what its published binary actually does.
4. **Deletion is allowed only for never-published drafts.** A transform with no row ever written to `transform_binaries` (never published) can be deleted outright, cascading to its `transform_saved_state` and `transform_resources` rows. A transform that has been published at least once cannot be deleted through this path (no live-consumer-breaking delete).

Deferred, not decided now (flagged, revisit if they become real problems): save history/versioning beyond single-row overwrite, and locking/warning for concurrent multi-tab edits of the same draft — both remain last-write-wins as currently implemented.

## Consequences

- `agents/transforms.md`: bucket 3 ("Publish") section updated to state the source/binary consistency gate and the in-place-republish behavior explicitly.
- `agents/invariants.md`: data-integrity section gains the publish consistency-gate rule.
- `features/transform_drafts.md`: rewritten from a rough draft into a settled requirement reflecting all four decisions above.
- Not yet implemented in code: the publish-time mismatch check (backend validation) and the delete-draft endpoint/UI. Both are scoped, ready to hand to `creator-agent`.
