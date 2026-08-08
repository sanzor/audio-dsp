# 0008: Publish requires a validated composite draft

- **Status:** Supersedes 0007 (item 4 only)
- **Date:** 2026-08-05

## Context

`agents/decisions/0007-composite-draft-validation-gate.md` un-gated Save (it no longer validates the graph) and introduced an explicit Validate action that persists `transform_draft.is_validated`. Item 4 of that decision made an explicit product call: Publish's composite branch would keep re-running `composite_validator::validate_composite_graph` from scratch, independent of `is_validated` — a creator could publish without ever clicking Validate.

After being shown the tradeoff, the user has now explicitly reversed that specific call. This decision only revisits item 4. **Items 1, 2, 3, 5, 6, and 7 of 0007 are unaffected and remain the current, correct behavior**: Save still persists unconditionally with no validation and unconditionally resets `is_validated = false` (item 1, 3); Validate still runs `validate_composite_graph` and writes `ports`/`is_validated` on success (item 2); the flag is still named/stored/shaped exactly as 0007 specified (items 5, 6); primitives are still unaffected, Compile still plays the equivalent role for them (item 7). Nobody should read this doc as reopening any of that — only "does Publish check `is_validated` first" changed.

## Decision

`publish_transform`'s composite branch (`backend/api/src/transforms/transforms_provider_service.rs`) now gates on `transform_draft.is_validated` before doing anything else:

1. Load the draft and its `graph_definition` exactly as before (unchanged: "nothing has been saved for this composite yet" if there's no graph at all).
2. **New early exit:** if `draft.is_validated == false`, return `ServiceError::Validation("composite draft must be validated before publishing")` immediately — before the leaf lookup or `validate_composite_graph` ever run. This is a distinct, clearer message from the generic validator error, so a creator who hasn't clicked Validate gets a different signal than one whose graph is actually broken.
3. If `draft.is_validated == true`, proceed with the **existing, unmodified** re-validation: `leaf_transform_ids` → `get_leaf_transform_infos` → `composite_validator::validate_composite_graph` → derive `ports` → `publish_composite_transform`. This logic is not weakened, removed, or short-circuited — it still exists specifically because a leaf transform referenced by the graph may have been unpublished or deleted since the last Save or the last Validate (the existing comment at the top of this block stays verbatim; it's still accurate, it now explains why re-validation runs *after* the gate rather than why there's no gate at all).

Both checks now coexist: `is_validated` must be `true` to even attempt publish, and the from-scratch re-validation must independently succeed too. Neither replaces the other. `publish_transform(id)`'s signature is unchanged.

Save's staleness behavior (0007 item 3 — any graph edit flips `is_validated` back to `false`) is exactly what makes this gate meaningful and non-stale: a creator cannot Validate once, then edit the graph without saving, and have Publish believe the edited state was checked — any Save before Publish already reset the flag.

## Consequences

- `agents/transforms.md`: the "Composite draft validation" section (specifically the "Independent of Publish, on purpose" bullet and the "No consumer reads it yet" bullet, which is now false — Publish reads it) and the "Bucket 3 — Publish" section's "Composite publish independently re-validates from scratch, unconditionally" bullet both need to describe gate-then-revalidate instead of independent-of-`is_validated`. Updated in this change.
- `agents/invariants.md`: the data-integrity bullet stating Publish "must never check `is_validated` as a publish precondition" is now wrong and is corrected in this change to describe the gate, with a reference to this doc.
- No change to `agents/architecture.md`'s Creator/Editor boundary — stays entirely inside Creator's existing ownership of the transforms module and composite canvas, same as 0007.
- No schema change — `transform_draft.is_validated` already exists from 0007's migration; this only changes what reads it.
- No frontend change — `publishMutation.error?.message`'s existing generic error surfacing on `composite-canvas.tsx` (the button `title` and the truncated-but-full-text error span) already renders any `ServiceError::Validation` string cleanly, verified against this exact code path with the pre-existing "nothing has been saved for this composite yet" message as precedent. The new message travels the identical route: `ServiceError::Validation` → `map_service_error`'s `BadRequest().body(msg)` (`transforms_controller.rs`) → `http.ts`'s `response.text()` → `Error.message` → `publishMutation.error`.
- Hand-off: `creator-agent` — backend only (`transforms_provider_service.rs`'s composite branch of `publish_transform`), plus the test harness in `transforms_provider_service_tests.rs`.
