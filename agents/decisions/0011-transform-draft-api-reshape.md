# 0011: Transform-draft API reshape; is_validated removed; Creator frontend brought in line

- **Status:** Accepted (supersedes 0007, 0008; frontend follow-up to 0010)
- **Date:** 2026-09-09

## Context

The backend's transform-draft controller (`backend/api/src/controllers/transform_drafts_controller.rs`,
mounted at `/draft_transforms`) was reshaped independently of the Creator
frontend, which was left with several dead/stale call sites pointed at
routes, request bodies, and response shapes that no longer exist. This
decision records what changed backend-side and the corresponding frontend
catch-up (`creator-agent`'s pass), in one place, since `agents/transforms.md`
and decisions 0007/0008/0010 described a contract that drifted out of sync
with the actual code.

## Decision

### Route/body reshape

- `save-primitive`/`save-composite` (two routes) are now one unified
  `PUT /draft_transforms/{id}/save`, body `SaveDraftParams` (untagged enum):
  a primitive draft accepts `{source_code}` only (no `wasm_base64` field —
  dropped entirely, not just optional), a composite draft accepts
  `{graph_definition}`. The draft's persisted `kind` picks the variant.
- `validate-source-code` renamed to `validate-source`
  (`POST /draft_transforms/{id}/validate-source`, body `{source_code}`) —
  unchanged in behavior (a fast compile-only check, doesn't save).
- New standalone `POST /draft_transforms/{id}/validate-graph`, body
  `{graph_json: string}` (may include unsaved canvas edits — unlike the old
  is_validated model, this validates whatever graph the caller hands it, not
  "whatever's persisted"). Returns `{ports: PortMetadataJson[]}` and
  persists nothing.
- `publish` split into `publish-primitive` / `publish-composite` — same
  underlying `publish(id)` call, just two routes with kind-specific 400
  messages instead of one route branching internally.
- `TransformDraftDto` (create/save response) and `TransformDto`
  (publish/get response) both dropped their structured `ports`/`params`
  fields — everything the old `ports`/`params`/`graph_definition` triple
  carried now lives only inside `metadata_json` (a JSON string), parsed
  client-side. `PortMetadataJson`'s field is `order`, not `port_order`, and
  there's no `port_id` — ports are derived at compile/validate time, never
  persisted with an id.
- `apiDeleteTransform` (frontend) was hitting `DELETE /transforms/{id}` —
  the *published*-transform controller's delete, a different thing from
  what the call site actually wanted. Switched to
  `DELETE /draft_transforms/{id}`, matching the "delete draft, never-published
  only, 409 otherwise" semantics `TransformController.handleDeleteTransform`
  already documented.

### `is_validated` removed entirely (reverses 0007/0008)

`transform_draft.is_validated` is no longer read or written anywhere in the
Rust backend (confirmed via a full-repo grep — zero hits). This silently
reversed 0007 (which introduced the flag) and 0008 (which made Publish gate
on it) with no decision doc recorded at the time; this entry is that
missing record. The migration 0022 DB column is left in place (schema
change is a separate, more consequential call, same reasoning 0010 used for
leaving the ticket/resource tables in place) but nothing reads or writes it.
Composite Publish no longer gates on any persisted validate flag; it still
independently re-runs `composite_validator::validate_composite_graph` from
the saved graph before publishing (that re-validation was never contingent
on `is_validated` in the first place — see 0007/0008's own text — so this
doesn't remove a safety check, only the pre-check gate in front of it).

### Composite graph wire format: leaf tag is now the transform's own kind

Independently of the above (found while doing this pass, not previously
documented anywhere): the composite graph's `Node` enum
(`backend/api/src/transform_drafts/graph_validator/node.rs`) tags a leaf
node `"primitive"` or `"composite"` (matching the referenced transform's
own kind — a mismatch fails validation) instead of the old generic `"leaf"`
tag. `Node::Composite` existing at all means a composite can (per the
backend type, at least) reference another *published composite* as a leaf,
which the frontend's wiring model didn't previously distinguish. The
frontend's wire-format type (`CompositeGraphDefinition.ts`) and the one
serialize/deserialize boundary that touches it
(`CompositeCanvasStore.ts`'s `toGraphDefinition`/`beginEditingCompositeGraph`)
were updated to match; the canvas's own internal `CanvasNode` model keeps a
single `"leaf"` tag regardless of which kind, since nothing on the canvas
side needs to distinguish them today.

### Ticket-based compile's frontend remnants removed (finishes 0010's frontend follow-up)

0010 removed the backend's ticket/worker pipeline and noted the frontend
still needed a matching update. Removed entirely: `TicketService.ts`,
`hooks/tickets/{mutations,queries}.ts`, the `tickets` query-key entry, the
Compile button and its ticket-poll loop in `code-editor.tsx`, and
`CreatorStore.ts`'s `activeTicketByTransform`/`compiledDraftByTransform`
state. In their place: a lightweight "Check" button wired to
`validate-source` (reuses the existing output-tab diagnostics panel; doesn't
persist on success), since Save's own inline 400 diagnostics alone were
judged not quite enough replacement value to skip it.

**"Try it" preview for primitives has no data source anymore and is
disabled outright, not left silently broken.** The old flow fed a
just-compiled ticket's `wasm_base64` straight into the preview worklet;
`TransformDraftDto` carries no wasm bytes at all (only `has_binary: bool`),
and no route exists to fetch a draft's own not-yet-published binary.
`usePrimitivePlaybackControls` now always reports `canStartPlayback: false`
with a `disabledReason` surfaced as a tooltip on the disabled Play button.
**Backend gap, not fixed here:** bringing this back needs a new route (e.g.
`GET /draft_transforms/{id}/binary`) to fetch a draft's own compiled
artifact pre-publish. Composite "Try it" is unaffected — it's a purely
client-side `GraphCompiler`-based preview, unrelated to any of this.

### Port-shape-diff advisory removed

`GET /transforms/{id}/publish/port-diff` doesn't exist on either backend
controller. `apiGetPublishPortShapeDiff` and its only call site
(`usePublishWithPortShapeDiff.ts`, an advisory-only, already-failing-soft
pre-publish confirm dialog) were removed entirely; each publish button now
calls its mutation directly. **Backend gap, not fixed here:** "warn before
republish changes port shape" needs a new endpoint on
`draft_transforms_controller.rs` if still wanted.

## Consequences / follow-ups not done in this pass

- **`apiGetTransformDefinition`/`apiResolveTransformDefinitions`
  (`GET /transforms/{id}`, `POST /transforms/resolve`) were left pointed at
  the published-transform controller, unchanged.** This hook is genuinely
  shared: Creator's code editor/composite canvas call it to load *their own*
  currently-open draft, while `editor-agent`'s `node-details-modal.tsx`,
  `transform-details-modal.tsx`, and `canvas-panel.tsx` call the exact same
  hook/function to look up a *placed graph node's* published transform
  details (which may not even be owned by the current user). Those are two
  different needs that can't both be served by one fixed endpoint target,
  and picking one over the other risks silently breaking the other surface.
  Net effect: opening the Creator's code editor or composite canvas on a
  transform that has unsaved-since-last-publish or never-published state
  will show stale/blank source and graph — `TransformDto` (what this
  endpoint returns) only has data once something has been published;
  bucket 2's current draft state lives at `GET /draft_transforms/{id}`
  instead, which nothing currently calls for this purpose. **Needs a
  product-owner call**, likely either a second, Creator-scoped hook pointed
  at `/draft_transforms/{id}` (leaving the shared hook as-is for Editor), or
  a backend change that merges draft-over-published state back into one read.
- **`TransformPort`/`TransformParam` (`domain/Transform/`) keep their
  existing `port_id`/`port_order`/`param_id`/`param_order`/`default_value`/
  `min_value`/`max_value` shape**, deliberately not renamed to match the new
  wire field names (`order`, `default`, `min`, `max`, no id) — `editor-agent`'s
  `GraphCompiler.ts`, `canvas-panel.tsx`, `node-details-modal.tsx`, and
  `transform-details-modal.tsx` depend on exactly that shape, and this pass
  is Creator-surface-only. `Services/TransformService.ts`'s
  `parseTransformMetadata` bridges the two: it parses `metadata_json` and
  synthesizes `port_id`/`param_id` as a stable-per-response array index.
  This is a real, if narrow, backend gap: **no HTTP endpoint on either
  controller returns a real, persisted port id anymore, for any transform,
  published or draft** — everything is JSON-embedded, order-only. Whether
  that matters depends on whether anything durably persists a `port_id`
  reference across fetches (e.g. an editor graph's saved edge referencing a
  specific input port) — `CompositeGraphDefinition.ts`'s own prior doc
  comment already described `port_id` as "reassigned on every republish,"
  so this may already have been an accepted-fragile identifier rather than
  a new problem; flagging for `editor-agent`/`product-owner` to confirm
  rather than assuming either way.
- `TransformSummary.published` and the workspace-scoped
  `GET /v1/workspaces/{workspace_id}/transforms` catalog endpoint
  (`workspace_controller.rs`'s `WorkspaceTransformSummaryDto`) were found,
  while auditing this area, to already not carry a `published` field at
  all — meaning `transforms-sidebar.tsx`'s drag-a-published-leaf-onto-canvas
  gating (`t.published`) is currently always false. This predates and is
  unrelated to this reshape (that controller wasn't touched by it); noted
  here only because it was discovered in passing, not fixed in this pass.

## If you change any of this

Update `agents/transforms.md`'s route names/shapes and this file in the
same change.
