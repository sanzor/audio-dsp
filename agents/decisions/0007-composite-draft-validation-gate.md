# 0007: Composite draft validation gate — un-gate Save, add explicit validate/mount action

- **Status:** Superseded by 0008 (item 4 only — Save/Validate/is_validated flag implementation in items 1-3,5-7 unaffected)
- **Date:** 2026-08-05

## Context

The composite canvas's Save button (`save_composite_draft`, `backend/api/src/transforms/transforms_provider_service.rs:111-119`) runs full `composite_validator::validate_composite_graph` and rejects the whole save if the graph is invalid — e.g. a freshly-dropped Input node with no edge yet. This blocks the basic workflow of dragging in nodes and wiring them up incrementally: a creator can't preserve work-in-progress without first making the graph structurally complete.

User's product decision: stop conflating "persist the current draft" with "assert this graph is valid" — the same split the compile/save/publish model already makes for primitives (`agents/transforms.md`), just not yet made for a composite's own validation step.

Investigated whether a new gate would have a real runtime consumer to attach to, before assuming scope:
- `composite_validator.rs::build_node_ports` rejects any leaf whose `kind != "primitive"` — **composite-of-composite does not exist today**, so "can this composite be picked as a leaf in another composite" has no code path to gate.
- `agents/decisions/0006-composite-feedback-cycle-support.md` confirms a *published* composite can't be placed inside an Editor graph either (no wasm binary for `kind = "composite"`, no flatten/inline logic anywhere).

So today, nothing reads a new flag. Per explicit user instruction, this is still being built defensively — shaped so a future consumer (composite-of-composite, Editor consumption of composites) can read it without a schema/API rework — rather than implemented as a purely cosmetic status dot.

**Distinct from `agents/decisions/0005-composite-node-inspector.md`'s per-node enable/disable.** 0005 shipped an ephemeral, frontend-only, session-scoped toggle for excluding individual *leaf nodes* from a composite's preview/save/publish compile. This decision is unrelated: it's a persisted, backend-owned, whole-*composite-draft*-scoped flag answering "does the last-saved graph pass validation," not "which nodes are temporarily excluded right now." Both loosely use "enable/disable" vocabulary — keep the two concepts and code paths separate; don't merge or rename toward each other.

## Decision

1. **Un-gate Save.** `save_composite_draft` no longer calls `composite_validator::validate_composite_graph`. It persists `graph_definition` as-is, always succeeds structurally (normal DB-error cases aside), and **no longer writes `ports`** — the `transform_draft.ports` column is left untouched by Save (whatever it already held: empty for a brand-new draft, or the last successfully-derived set from a prior validate/publish).
2. **New explicit validate action** (`POST /transforms/{id}/validate` or equivalent — naming/routing left to `creator-agent`). Runs the exact `validate_composite_graph` call Save used to run, against the just-persisted `graph_definition`. On success: writes the derived `ports` (same derivation as today) and sets a new `transform_draft.is_validated` boolean to `true`. On failure: leaves `ports`/`is_validated` untouched, returns the validation error for the UI to surface (same error shape Save's rejection used to produce).
3. **Staleness.** Any subsequent Save (i.e. any graph mutation) must flip `is_validated` back to `false` until the validate action is re-run successfully — same "stale until re-verified" pattern already used for `attachableResourceId` in the primitive code editor (`code-editor.tsx`), not a new concept. Concretely: `save_composite_draft`'s write should set `is_validated = false` unconditionally alongside its `graph_definition` update.
4. **Publish stays independent.** `publish_transform`'s composite branch keeps re-running `validate_composite_graph` from scratch, exactly as today (`transforms_provider_service.rs:132-138`, the existing "a leaf may have been unpublished since save" comment). It does **not** check `is_validated` as a precondition — validate/mount and Publish are two independently-gated checkpoints that happen to run the same validation logic, not a dependency chain. This was an explicit user call, overriding the natural-seeming alternative of requiring `is_validated == true` before Publish is even attempted.
5. **Flag name: `is_validated`**, stored on `transform_draft` (not `transform` — it must reset on every edit, unlike `transform.published`). Chosen specifically to avoid colliding in meaning with the existing `DbTransform.published` field; names the flag after what the action actually verified, not after a hypothetical future runtime effect ("mounted"/"active" were considered and rejected for this reason).
6. **Defensive shaping for future consumers (no new consumer implemented now):**
   - `is_validated` is returned wherever `transform_draft`/`DbTransformDraft`/`DbTransformDefinition` are already returned — no new fetch path needed for a future reader.
   - The natural future extension point, if/when composite-of-composite ships, is `LeafTransformInfo`/`get_leaf_transform_infos` (`transforms_data_provider.rs:119`, already the single place `composite_validator.rs` asks "is this referenced transform usable as a leaf") — noted here as the intended seam, not implemented as part of this change. `composite_validator.rs`'s existing `kind != "primitive"` rejection stays exactly as-is; nothing here loosens it.
7. **Primitives are unaffected.** No analogous new action for primitive transforms — Compile already serves this role for primitives (a primitive can't be attached/published without a successful compile ticket); this was confirmed with the user rather than assumed.

## Consequences

- `agents/transforms.md`: needs a new subsection (or an amendment to the existing "Bucket 2 — Save" composite notes) documenting that composite drafts now have a fourth, independently-writable sub-state (`is_validated`) sitting between Save and Publish, and that Save no longer validates or derives ports. **`creator-agent` must make this doc update in the same change** per `agents/ownership.md`'s escalation rule (this is a change to the compile/save/publish-adjacent model that doc exists to describe).
- `agents/invariants.md`: the "three buckets... must stay independently writable" line and the composite-specific validation notes should be checked/extended to mention the new validate step — `creator-agent` to assess and update if a rule is genuinely new or broken.
- No change to `agents/architecture.md`'s Creator/Editor boundary — this stays entirely inside Creator's existing ownership of the transforms module and composite canvas.
- New migration required: `transform_draft` gains `is_validated BOOLEAN NOT NULL DEFAULT false`.
- Hand-off: `creator-agent` — backend (`transforms_provider_service.rs`, `data_provider/transforms_data_provider.rs` + `_service.rs`, new migration, the `composite_validator.rs` call-site moving from Save to the new validate action — `composite_validator.rs` itself needs no logic changes) and frontend (`composite-canvas.tsx`, `CompositeCanvasStore`, a new mutations hook alongside `useSaveCompositeTransform`/`usePublishTransform`).
