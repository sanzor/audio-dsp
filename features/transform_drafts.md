Transform Drafts

The platform (creator surface) supports a user working on an audio transform, saving it as a draft, and publishing it when ready for use on the editor surface (by any user, not just its author). This doc states the settled requirement; the full compile/save/publish design lives in `agents/transforms.md`, and the decisions that settled the open questions below live in `agents/decisions/0002-transform-draft-lifecycle-decisions.md`.

## Lifecycle

- **Create** — a creator starts a new transform. This creates its draft state (bucket 2); no compile or publish has happened yet.
- **Edit** — the creator changes source code by hand (or, in the future, via a chat interface with AI integrated) in the Monaco editor.
- **Compile** — the creator submits a compile ticket; a backend worker builds the source to WASM and stores the resulting artifact (bytecode + introspected metadata) as an immutable compile resource (bucket 1). A ticket succeeding proves the build works — it does not save or publish anything.
- **Try it** — the creator can run the just-compiled binary client-side, in the creator surface, before deciding whether to save it. The binary travels down as base64 on the compile ticket/resource response (reusing the same encoding already used for published binaries), and gets executed through the *same worklet pipeline the editor uses* (wrapped in a degenerate one-node graph, via a creator-scoped connect/disconnect wrapper — not `WorkletController`) rather than a separate preview runtime — so preview can't behave differently from what actually runs once published. This is a one-way flow: the bytes never get sent back up. See `agents/decisions/0003-transform-preview-flow.md`.
- **Save** — the creator saves source code, optionally attaching a just-compiled resource's binary if (and only if) it was compiled from the exact source currently being saved. This is a reference-passing operation (`resource_id`, not raw bytes) — the backend copies its own already-validated row into the saved-state table rather than trusting client-supplied bytes. Save can happen any number of times; each save overwrites the transform's one saved-state row (no history/versioning — a deliberate scope decision, see below).
- **Publish** — once ready, the creator publishes. This bundles whatever the saved state (bucket 2) currently holds into the live, catalog-visible transform (bucket 3). Publish never compiles.

## Entity separation

- The draft (bucket 2, `transform_saved_state`) is a distinct DB table from both the compile resource (bucket 1, `transform_resources`) and the published transform (bucket 3, `transform_binaries` + `transforms`/`transform_ports`/`transform_params`). "Different in db and storage" is satisfied by this table-level separation — no separate object-storage tier is required (Decision 0002).
- None of the three buckets write to each other implicitly. Moving from one to the next always requires an explicit user action (compile, save, publish).

## Settled decisions (see 0002 for full rationale)

- **Republish** (Publish on an already-published transform) overwrites the live transform in place. No versioning, no compatibility gate against existing editor graphs — an accepted risk given editor graphs currently have no version pin.
- **Publish is gated on source/binary consistency**: publish must fail if the saved binary doesn't correspond to the currently-saved source.
- **Draft deletion** is allowed only for transforms that have never been published. Once published at least once, deletion through this path is not allowed.
- **Save stays single-row overwrite**, no version history. Deferred, not a rejection — revisit if it becomes a real problem.
- **Concurrent edits** (same transform, two sessions/tabs) are last-write-wins. Deferred for the same reason.

## Implementation status

All four follow-ups scoped from the BA/PO refinement pass are implemented:

- The save-attach provenance check (a `resource_id`'s ticket source must match the source being saved, in the same query as the "belongs to this transform" check) is implemented in `save_transform_state` — a mismatch is a hard validation error, not a silent downgrade to source-only.
- The publish-time source/binary consistency check is implemented in `publish_transform`, backed by a new `transform_saved_state.wasm_source_code` snapshot column (migration `0015_transform_saved_state_wasm_source`) — needed because the provenance check above only prevents *creating* a mismatched pair; a later source-only save can still drift `source_code` away from an earlier-attached binary, which is the actual case this gate catches. Covered by unit tests in `transforms_provider_service.rs` (no DB required — pure logic over a fake data provider).
- Draft deletion is implemented end-to-end: `DELETE /transforms/{id}` returns 409 if `transform_binaries` has a row for that transform, otherwise cascades via existing FKs; a delete (trash) button is wired into `transforms-sidebar.tsx`.
- The "Try it" preview path is implemented: `wasm_base64` is included on the compile ticket/resource response, and the creator surface runs it through a creator-scoped wrapper (`frontend/src/components/creator/creatorTransformPreview.ts`) built directly on `graph-worklet.js`/`WorkletMessageSender` — not `WorkletController`. Reviewed by `regression-review-agent` per `agents/ownership.md`'s escalation rule, since this item touches (read-only) files owned by `frontend-graph-agent`.

Remaining known gaps (unchanged, still deferred):

- No test coverage for the SQL-level save-attach and delete-guard checks specifically (they're straightforward, reviewed-by-hand queries; a DB-backed integration test harness doesn't exist yet in this repo — the only other transform test drives the real compiler and is `#[ignore]`d). The publish consistency gate's *logic* is unit-tested since it's DB-independent.
- Save history/versioning and concurrent-edit locking remain last-write-wins, as already noted above.

## Deferred

- Bucket 1 (`transform_tickets`/`transform_resources`) retention/eviction: agreed this is needed eventually (it's ephemeral compile history, not meant to accumulate forever), but it needs a dedicated background worker and isn't blocking — `transform_saved_state` copies what it needs at save time and has no ongoing dependency on ticket/resource rows surviving. Scope this as separate follow-up work when the worker is built.
