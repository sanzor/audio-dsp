# 0012: A draft's name/description are editable, as optional fields on Save

- **Status:** Accepted (reverses part of `agents/transforms.md`'s "fully
  read-only" characterization of the Creator properties panel, and part of
  `agents/invariants.md`'s "only source text is user-editable" line).
  **Supersedes this decision's own original shape** — see "History" below:
  name/description editing first landed as a standalone `PATCH
  /draft_transforms/{id}` endpoint, which was later folded into
  `PUT /draft_transforms/{id}/save` as endpoint-creep cleanup. There is no
  PATCH endpoint for this today, and there should not be one again — see
  "Why not a separate endpoint" below before reintroducing it.
- **Date:** 2026-09-20 (original), consolidated same day after review.

## Context

`agents/transforms.md` and `agents/invariants.md` documented a transform's
`name`/`description` as fully code-first: declared once in the author's
Rust `metadata()` function, introspected at compile time, and never
hand-edited anywhere in the Creator frontend. The user, via product-owner,
decided to reverse this: renaming/describing a transform draft should not
require round-tripping through the Rust source — it should be a direct edit
in the properties panel, like any other metadata field.

## Decision (current, consolidated shape)

- **No standalone endpoint.** `name`/`description` are optional fields on
  the existing Bucket-2 save payload, `PUT /draft_transforms/{id}/save`
  (`backend/api/src/transform_drafts/dto/requests.rs`):
  - `SavePrimitiveParams { source_code: String, name: Option<String>,
    description: Option<String> }`
  - `SaveCompositeParams { graph_definition: serde_json::Value, name:
    Option<String>, description: Option<String> }`

  Both new fields carry `#[serde(default)]`, but it's not load-bearing for
  the absent-key case: a plain `Option<T>` struct field already deserializes
  a fully-missing JSON key as `None` on its own (verified empirically against
  this workspace's serde 1.0 — `#[serde(default)]` made no difference to that
  case). The attribute is redundant here, not incorrect; it's kept mainly as
  explicit documentation of intent, and would only matter for other
  `Option<T>`-with-`Default`-trait scenarios this DTO doesn't exercise.
- **Precedence, spelled out because it's subtle:** `None` (an omitted key
  or explicit `null`) means "this call isn't touching the name/description"
  and leaves the existing `transform_draft` value untouched. `Some(x)` sets
  it to `x`, taking priority over both the existing row value and, for a
  primitive draft, the freshly-compiled metadata's name/description (which
  is only ever a last-resort seed for a row that somehow has no name yet —
  shouldn't happen on the common path since `insert_transform_draft`
  requires one at creation). SQL shape (primitive):
  `name = COALESCE($caller, transform_draft.name, $compiled)`, and
  symmetrically for `description`. Composite has the same two-tier
  precedence (caller override, else existing) with no compiled-fallback
  tier, since a composite draft never compiles.
  Validation (`validate_draft_name` — name must not be blank) runs only
  when the caller actually supplies a `name`; `None` is never validated or
  treated as "clear it".
- **No useless writes.** The frontend only includes the `name`/
  `description` keys in a Save payload when `CreatorStore`'s
  `isMetadataDirty` buffer is actually dirty for the transform being saved
  (`code-editor.tsx`, `composite/composite-canvas.tsx`,
  `unsaved-creator-changes-modal.tsx`). An unrelated source-only or
  graph-only save omits the keys entirely rather than re-sending unchanged
  values for the backend to write again.
- **Frontend:** the properties panel
  (`frontend/src/components/creator/transform-properties-panel.tsx`) still
  renders name as a text `<input>` and description as a `<textarea>`, and
  the live edit buffer still lives in `CreatorStore`'s
  `editingTransformMetadata` (`beginEditingTransformMetadata` /
  `updateEditingTransformMetadata` / `markTransformMetadataSaved` /
  `isMetadataDirty`) — unchanged from the original design. What changed:
  editing here no longer makes any network call of its own. Blur/Enter only
  runs the client-side blank-name check (surfaced inline ahead of Save's
  server-side rejection); the edit persists the next time Save Draft is
  clicked, which reads the same buffer and includes `name`/`description` in
  its payload only when dirty.
- **Switching-transform data-loss guard.** Because metadata edits now only
  persist via Save (no more independent on-blur network call),
  `CreatorStore.requestSelectTransform`/`requestCreateTransform` also check
  `isMetadataDirty` (alongside the pre-existing source/composite-graph dirty
  checks) before allowing a switch, routing through the same
  `pendingTransformAction` gate. `unsaved-creator-changes-modal.tsx` gained
  a third case (metadata-only dirty, independent of source/graph dirty) so
  it still opens for a name-only edit and flushes it via whichever Save
  variant (`source_code` or `graph_definition`) matches the currently-open
  surface.
- **Ports and params are unaffected** — still fully compile-derived,
  fully read-only in this panel, sourced from bucket-3. Publish is
  unaffected — it already reads whatever's currently on
  `transform_draft.name`/`.description` at publish time.
- **The left transforms-sidebar merge is unaffected and required no
  backend change to keep working.** `transforms-sidebar.tsx` resolves each
  listed transform's draft via `apiResolveTransformDrafts`
  (`POST /draft_transforms/resolve`) and merges bucket-3 summaries with
  bucket-2 draft name/description client-side, preferring the draft's
  values — this reads bucket-2's `name`/`description` directly, so it
  reflects whatever Save last wrote with no additional wiring. The
  consolidated `useSaveTransform` mutation invalidates the same query keys
  the old PATCH mutation used to (`transformDrafts.byId`, and a predicate
  match over all `transformDrafts.resolve(...)` keys), so the sidebar still
  refreshes correctly after a Save that includes a rename.

## Why not a separate endpoint (history)

The original implementation (this decision's first version) added a
standalone `PATCH /draft_transforms/{id}` endpoint, justified at the time
by two concerns: Save's `source_code` triggers a synchronous
compile-and-reject-on-failure, so bundling a rename into that payload
seemed to force an unwanted recompile just to rename; and the initial
blur-commit UX wanted renaming to work independent of a full save.

A follow-up then had to fix "Save Draft doesn't flush pending metadata
edits" — i.e. Save already needed to know about the metadata buffer to
avoid losing an edit that hadn't blurred yet. Once that landed, the PATCH
endpoint's only remaining justification (letting a rename skip a
recompile) no longer held up: Save already had to coordinate with the
metadata buffer on every click, so keeping a second endpoint around just
for the "no full save" case was pure endpoint/scope creep with no
remaining benefit. That endpoint/scope-creep assessment is what this
consolidation addresses: **delete the PATCH endpoint, fold name/description
into Save as optional fields, done.**

**Do not reintroduce a separate metadata endpoint.** If a future UX wants
renaming to avoid a recompile, solve it by having Save skip the compile
step when only metadata fields are present and `source_code`/
`graph_definition` are unchanged from the last saved snapshot — not by
resurrecting a second endpoint whose entire reason for existing was already
proven to be a wash once Save had to flush pending edits anyway.

### Addendum: the left-sidebar-staleness episode (kept for context)

Early in the PATCH-based implementation, the left sidebar
(`transforms-sidebar.tsx`) went stale after a rename because it read
bucket-3 (`transform`) exclusively, and PATCH only ever wrote bucket-2
(`transform_draft`). **A first fix attempt was implemented, then fully
reverted:** having the metadata write also run `UPDATE transform SET
name = ..., description = ... WHERE transform_id = $1 AND metadata IS
NULL` in the same transaction as the `transform_draft` write. This was
implemented and verified live against Postgres in one working session,
then reverted in a later session without a corresponding round-trip
through the user — it makes bucket 2 (draft) and bucket 3 (published) no
longer independently writable, exactly the coupling
`agents/invariants.md`'s three-bucket independence rule exists to prevent,
for what was really a frontend read-path problem (two stores never merged
for one view), not a data-model problem. That reasoning holds up on its
own merits and matches the user's later, explicit direction (raised
independently, in response to being asked directly) to keep drafts and
published transforms unsynced at the database level — but the revert
itself was an agent's unilateral call, not something the user reviewed
and rejected in the moment, and it should not have been written up as if
it were. Recorded here so this doc's history stays accurate.

**The accepted fix was, and remains, the frontend draft/published merge**
described above (`transforms-sidebar.tsx` + `apiResolveTransformDrafts`).
It required no backend change then and requires none now — Save writing
`transform_draft.name`/`.description` (via whichever mechanism: the old
PATCH, or today's optional Save fields) is all the sidebar's merge has ever
needed. **There is still no `UPDATE transform ...` sync anywhere in this
codebase, and none should be added** — that door stays closed for the same
three-bucket-independence reason it was closed the first time, regardless
of which endpoint originates the `transform_draft` write.

## Consequences

- `agents/transforms.md`'s code-first paragraph carries an explicit
  exception: `metadata()` still seeds a draft's *initial* name/description
  and is still what compile/introspection reads and validates, but is no
  longer the ongoing source of truth for a draft's *displayed*
  name/description once the user has edited them via Save.
- `agents/invariants.md`'s "only source text is user-editable" line and its
  "name/description ... not hand-entered" line both carry a drafts-only
  carve-out — now phrased as "name/description are optional fields on Save"
  rather than referencing a PATCH endpoint.
- A published transform's name/description are still only ever set by
  Publish (unchanged).
- This repo has no DB-backed test harness for `transform_drafts` (grep
  confirms no `sqlx::test`/test-pool pattern anywhere in `backend/api`).
  DTO (de)serialization of the new optional fields is unit tested
  (`dto/requests_tests.rs`), the name-blank guard is unit tested in
  isolation (`transform_drafts_provider_service.rs`'s `tests` module,
  unchanged from the original PATCH-era version), and the ownership check
  is covered by reuse of the existing, already-proven `require_owner` guard
  rather than a new test.
