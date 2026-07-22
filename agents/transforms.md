# Transforms: contract, compilation, and runtime

This is the single place to look for what a transform is allowed to do, how it gets compiled, and how the backend parses what came out. See `agents/architecture.md` for how transforms fit into the creator/editor product split, and `agents/invariants.md` for the hard rules derived from this doc.

## The contract (`backend/transform-sdk`)

- The author implements `transform_sdk::Transform` for a `Default` struct: `fn process(&mut self, samples: &mut [f32], params: &[f32])` (mutates `samples` in place, runs once per audio quantum) and `fn metadata() -> TransformMetadata` (static description, called once per compile, never at audio time).
- The author calls `transform_sdk::export_transform!(MyType)` exactly once. The macro generates the entire wasm32-unknown-unknown surface — the author never hand-writes `extern "C"` code:
  - `alloc(len) -> *mut f32` — a 1 MiB bump-and-wrap arena, not a real allocator. It never frees. This is safe only because at most two pointers (input samples, params) are ever live at once, and each is fully consumed synchronously before the next `alloc`. A transform must not stash a pointer across calls.
  - `process(ptr, len, params_ptr, params_len)` — the extern-C entry point that calls the author's `Transform::process`.
  - `transform_metadata_ptr()` / `transform_metadata_len()` — expose a JSON serialization of `Transform::metadata()` for the backend's post-compile introspection. `metadata()` must be stable for a given build.
  - `memory` is exported automatically via `crate-type = ["cdylib"]`.
- The module must declare **zero imports**. Both the editor's worklet and the backend's introspection step instantiate it with no host imports at all, and both will refuse to run a module that declares any.
- The whole transform definition — `name`, `description`, ports, and params — is entirely code-first: the author declares all of it literally in `metadata()` (see the `DEFAULT_CODE` template in `frontend/src/components/creator/code-editor.tsx` for the exact shape). Nothing infers any of this from function signatures or static analysis, and `metadata()` must stay in sync with what `process()` actually reads/writes — the SDK cannot enforce that.

## What's allowed in the editor at runtime

- `frontend/src/audio/worklet/graph-worklet.js` is the only runtime caller of a transform's wasm exports, on the dedicated `AudioWorkletProcessor` thread.
- Block size is fixed at `BLOCK_SIZE = 128` samples per quantum. `callWasm` allocates exactly two buffers per call — input and params — writes them into the instance's linear memory, calls `process`, and reads the (in-place mutated) input buffer back out.
- `WebAssembly.instantiate(binary)` is called with **no import object**. A transform that declares any import fails to instantiate in the browser; this is intentionally mirrored server-side (below), so a module that passes backend introspection is guaranteed loadable in the editor too.
- The editor never compiles transforms. It only fetches and caches already-published binaries (`apiGetTransformBinary`/`apiGetTransformBinaries`) and hands them to the worklet via the `SET_GRAPH` message.

## Three buckets: compile, save, publish

A transform's in-progress state is split into three independent storage locations, each owned by exactly one action. None of them infer or trigger another — moving from one to the next always requires an explicit user action.

| Bucket | Table(s) | Written by | Meaning |
|---|---|---|---|
| 1. Compile (check) | `transform_tickets` + `transform_resources` | a successful/failed compile ticket, always | "this specific compile attempt produced this artifact" — immutable history, many rows accumulate, never live |
| 2. Save | `transform_saved_state` (one row per transform) | the Save action | the creator's current state: source text always, plus a binary/metadata snapshot *if* a compiled resource was attached at save time |
| 3. Publish | `transform_binaries` + `transforms.{name,description}` + `transform_ports`/`transform_params` | the Publish action | the live artifact — what the editor surface actually fetches and runs |

None of these tables reference each other except by explicit action: a ticket's resource never becomes live on its own, and a save never gets published on its own. This is a deliberate rejection of the earlier "v1 auto-publish-on-success" behavior — it was replaced because a compile ticket succeeding is not the same claim as "the creator wants this live now."

### Bucket 1 — Compile

1. Creator submits `{transform_id, source_code}` (`POST /transforms/tickets`, handled by `backend/api/src/tickets/`) → a `DbTicket` is created in state `Processing`.
2. A worker (`backend/api/src/ticket_worker/worker.rs`) consumes a `TicketCreatedEvent` and runs `Processor::process` (`backend/api/src/ticket_worker/processor/processor.rs`):
   1. confirm the ticket exists;
   2. `build_job::compile_transform_source` writes the user's source **byte-for-byte** (no wrapping, so compiler error line numbers match what the user wrote) as `src/lib.rs` into a per-job scratch dir, generates an entirely backend-authored `Cargo.toml` pinned to the local `transform-sdk` path, and runs `cargo build --release --target wasm32-unknown-unknown --offline` as a subprocess with a timeout and output-size cap;
   3. `metadata_introspector::introspect_metadata` instantiates the resulting wasm with wasmtime, zero host imports, and a fuel budget — this briefly executes attacker-controlled code server-side — then reads back name/description/ports/params as JSON;
   4. validates the parsed metadata (name non-empty, port names non-empty, param names/orders unique);
   5. **stores the full artifact** (wasm bytecode + name + description + ports + params) into `transform_resources` via `create_resource` — this is the *only* write a compile ticket makes; it never touches `transform_saved_state` or the live tables;
   6. the ticket flips to `Successful{resource_id}`. Any failure at steps 2–4 instead flips it to `Failed{message}`.
3. The creator frontend polls ticket status (`useCompileTicketStatus`) until terminal, and on success records `{resourceId, sourceCode}` against the transform in `CreatorStore` — the exact text that was submitted, so it can tell later whether that resource is still current.
4. **Try it:** the creator can run that compile resource's binary client-side to preview it before saving. `CompileTicketStatusDto`/`CompileResourceDto` (`ticket_controller.rs`) carry `wasm_base64` (same encoding `TransformBinaryDto` already uses for published binaries) once the ticket is `Successful` — the handler fetches the resource's bytecode via `get_ticket_result` and encodes it inline, since `TicketStatus::Successful` itself only carries a `resource_id`. Frontend decodes it (`decodeBase64Binary`, exported from `TransformService.ts`) and drives it through the *same* worklet module and message protocol the editor uses (`graph-worklet.js` / `WorkletMessageSender`, **not** `WorkletController`), wrapped in a degenerate one-node `CompiledGraph` via a creator-scoped wrapper (`frontend/src/components/creator/creatorTransformPreview.ts`) — not a separate hand-rolled runtime — so preview execution can't diverge from what actually runs post-publish. This is a distinct data flow from Save's `resource_id` reference: preview bytes travel backend→frontend only and are never sent back. See `agents/decisions/0003-transform-preview-flow.md`.

### Bucket 2 — Save

- `PUT /transforms/{id}/save` (`useSaveTransform`, wired into the code editor's Save button) always overwrites `transform_saved_state.source_code`. This is a plain sync overwrite — never blocked by, or dependent on, compiling.
- It optionally takes a `resource_id`. If given, the backend verifies **both** that the resource belongs to this transform **and** that the resource's own compile ticket (`transform_tickets.source_code`, joined via `transform_resources.ticket_id`) equals the `source_code` being saved right now — one query, one condition set (`save_transform_state` in `transforms_data_provider_service.rs`). Only if both hold does it copy that resource's binary/metadata (plus a `wasm_source_code` snapshot of the matched source, added specifically so Publish can later detect drift — see bucket 3 below) from `transform_resources` into `transform_saved_state`. If omitted, any previously saved binary/metadata is left untouched — a source-only save never wipes out the last good build.
- **A mismatched or foreign `resource_id` is a hard validation error, not a silent downgrade.** The frontend's `attachableResourceId` guard (`code-editor.tsx`) already only ever sends a `resource_id` while the editor buffer still exactly matches the source that resource was compiled from, so a mismatch reaching the backend at all means a stale or buggy client — the whole save (including the source-only part) is rejected and rolled back, consistent with how the pre-existing "resource does not belong to this transform" case already behaved. Edit anything after compiling and Save reverts to source-only (no `resource_id` sent) until you compile again — that path is unaffected and still always succeeds.

### Bucket 3 — Publish

- `POST /transforms/{id}/publish` (`usePublishTransform`) reads `transform_saved_state` and, if it has a binary, calls the existing `publish_compiled_transform` to atomically write wasm bytecode + source + name + description + ports + params into the live tables. **It does not compile anything** — it bundles what's already been saved. If bucket 2 has no binary yet (never saved with a successful build), it fails with a validation error.
- This is intentionally the only place `publish_compiled_transform` is called from now — the ticket worker no longer calls it.
- **Publish requires the saved binary and saved source to correspond.** A source-only save (no `resource_id`) leaves a previously-attached binary in place without updating it — see `agents/decisions/0002-transform-draft-lifecycle-decisions.md`. `publish_transform` (`transforms_provider_service.rs`) checks `transform_saved_state.wasm_source_code == transform_saved_state.source_code` before calling `publish_compiled_transform`; a mismatch fails with a validation error telling the creator to recompile and re-attach. Note this check is *not* made redundant by Save's item-1 provenance check above: that check only prevents an attach from *creating* a mismatched pair; a later source-only save can still move `source_code` forward while `wasm_bytecode`/`wasm_source_code` stay put, and that's exactly the drift this gate exists to catch (defense-in-depth, but against a real, still-reachable state, not a hypothetical one).
- **Republish overwrites in place, with no version pin.** Hitting Publish on an already-published transform re-runs the same atomic write — no versioning, no compatibility check against existing editor graphs (which reference a transform by bare ID with no version pin in `graph_state`). This is an accepted risk, not an oversight — see `agents/decisions/0002-transform-draft-lifecycle-decisions.md`.

### Draft deletion

- `DELETE /transforms/{id}` (`apiDeleteTransform`, wired to a per-row delete button in `transforms-sidebar.tsx` via `useDeleteTransform`/`TransformController.handleDeleteTransform`) only succeeds for a transform that has never been published (no row ever written to `transform_binaries`) — `PostgresTransformsDataProvider::delete_transform` checks `transform_binaries` first and returns a `Conflict` (surfaced as HTTP 409) otherwise. On success it cascades to `transform_saved_state`, `transform_tickets` (and transitively `transform_resources`), `transform_ports`, and `transform_params` via the `ON DELETE CASCADE` FKs already in place from migrations 0004/0009/0011/0014 — no new migration was needed for the cascade itself, only for the unrelated `wasm_source_code` column above.

## What this means for other surfaces

- The creator's transform properties panel (`frontend/src/components/creator/transform-properties-panel.tsx`) is fully read-only — name, description, ports, and params only change via Publish (bucket 3), never hand-edited.
- Any change to the ABI (macro-generated exports, arena semantics, block size, import policy) must be reflected in all three of: `backend/transform-sdk`, `metadata_introspector.rs`'s validation, and `graph-worklet.js` — they currently agree by convention, not by shared code.

## If you change any of this

Update this file in the same change, and check whether `agents/invariants.md` needs a matching update.
