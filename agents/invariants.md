# Invariants

## Real-time and UI safety

- WASM compilation belongs to the backend compile-ticket pipeline, not to the editor frontend.
- The editor frontend may fetch and cache published transform binaries, but it must never request compilation as part of editor workflows.
- The editor frontend may build and derive graph artifacts locally, including transform lists, adjacency data, and region-level DAG views, as long as that work stays fast enough for normal interaction.
- Published transforms execute on the frontend through the worklet runtime, so UI-thread code must stay separate from real-time audio execution paths.
- Never put heavy DSP, decoding, or unbounded graph recomputation on the React main thread during interaction or playback.
- Never block an audio callback path with network, disk, or UI work.
- Any lifecycle around Wavesurfer or playback resources must clean up deterministically.
- Post-compile metadata introspection (`backend/api/src/ticket_worker/processor/wasm/wasm_parser.rs`, validated by `backend/api/src/ticket_worker/processor/validator/validator.rs`) briefly executes attacker-controlled wasm server-side. It must stay fuel-limited, use zero host imports, and never grant WASI/IO access. As of `agents/decisions/0010-remove-ticket-based-compile.md` this runs synchronously on the Save request, not in a background ticket worker.

## State ownership

- The active source of truth for graph editing must be unambiguous for a given feature.
- During editing, the frontend may be the working source of truth for graph composition and derived region-level execution order before persistence.
- AI-assisted editor actions must resolve into explicit graph mutations, not hidden side effects outside the normal editor state flow.
- Client normalized stores and backend persisted state must not silently diverge.
- Mutations that affect parent-child trees must update both the entity and the relevant parent references.

## Data integrity

- AI-generated transform source must go through the same validation, compilation, ticketing, and publish flow as manually authored source.
- AI-assisted editor flows may only use published transforms from the catalog, not unpublished source or ad hoc binaries.
- A successful **publish** (not a successful compile ticket) must leave a transform's binary and its full definition (name, description, ports, params) mutually consistent — `publish_compiled_transform` writes all of it in one transaction. A compile ticket succeeding says nothing about the live transform; it only proves a build works and stores its result in `transform_resources`.
- Name, description, ports, and params for a *published* transform are all derived from a compiled artifact's exported metadata, not hand-entered. The creator frontend must not offer any manual edit UI for them (no add/delete-port, no rename/re-describe form).
- Save and publish must stay independently writable. As of `agents/decisions/0010-remove-ticket-based-compile.md`, compile is no longer a separate bucket — it runs synchronously inside save (a primitive save that doesn't compile persists nothing) — but save must still never trigger a publish, and publish must never compile, only bundle what save already has. See `agents/transforms.md` for the full model.
- **A composite draft's explicit validate action (`transform_draft.is_validated`, `POST /transforms/{id}/validate`) is a fourth, independent checkpoint — not a fifth bucket and not a dependency between the existing three.** `save_composite_draft` must never validate the graph or derive `ports`; it persists `graph_definition` structurally and unconditionally resets `is_validated = false`. `publish_transform`'s composite branch must check `is_validated == true` as a publish precondition, failing fast with a distinct validation error if not — and, once that gate passes, must still keep re-running `composite_validator::validate_composite_graph` from scratch before actually publishing, since a leaf transform may have been unpublished or deleted since the last Save/Validate. The gate and the re-validation are both required and neither substitutes for the other. See `agents/decisions/0008-publish-requires-validated-composite-draft.md` (superseding item 4 of `agents/decisions/0007-composite-draft-validation-gate.md` — 0007's items 1-3/5-7 on Save/Validate/the flag itself are unaffected) and `agents/transforms.md`'s "Composite draft validation" section.
- Publish must fail validation if `transform_draft` holds a binary that doesn't correspond to its current saved source. Since 0010 removed source-only saves, this can't actually happen through normal Save anymore (source and binary are always written together or not at all) — the check stays as a defensive backstop, not a live guard against a real path. A published transform's advertised source must always match what its published binary actually does. Enforced by comparing `transform_draft.wasm_source_code` against `transform_draft.source_code` in `publish_transform`. See `agents/decisions/0002-transform-draft-lifecycle-decisions.md` and `agents/decisions/0010-remove-ticket-based-compile.md`.
- A transform may only be deleted if it has never been published (no row in `transform_binaries`). Once published at least once, it must not be deletable through the draft-management path, since editor graphs may already reference it. Enforced in `PostgresTransformsDataProvider::delete_transform` (checks `transform_binaries` before deleting, returns `Conflict`/409 otherwise); cascade to `transform_saved_state`/`transform_tickets`/`transform_resources`/`transform_ports`/`transform_params` relies on existing `ON DELETE CASCADE` FKs.
- The only transform field the creator frontend may let a user edit directly is source text. As of `agents/decisions/0010-remove-ticket-based-compile.md`, `PUT /draft_transforms/{id}/save-primitive` takes only `{source_code}` — Save compiles and introspects it synchronously under the normal fuel/zero-import limits and writes the resulting source/binary/metadata snapshot atomically, or rejects the whole save if it doesn't compile. There is no frontend-supplied WASM package anymore (superseded decision 0009).
- The creator's "Try it" preview (a just-compiled, not-yet-saved/published binary run client-side) must execute through the same worklet module and message protocol the editor uses post-publish (`graph-worklet.js` / `WorkletMessageSender`), never a separate hand-rolled runtime, and must never reuse the editor's stateful `WorkletController`/`useWorkletSetup` (which write into editor-only global state and assume a Wavesurfer media element). See `agents/ownership.md`'s Shared zones.
- **Port kind and cardinality are part of a transform's compile-derived definition, same as name/description/ports/params.** `transform_ports.kind` (`program` | `sidechain`) and `.cardinality` (`single` | `many`) are introspected from `metadata()` at compile time, never hand-entered, and validated by `validator::validate_primitive`: output ports must be `program`/`single`, a transform must declare exactly one output port, and port names must be unique within a direction. See `agents/transforms.md`'s ABI contract section and `agents/decisions/0004-multi-input-named-ports.md`.
- **Fail-closed vs. silent, by port kind.** An unwired `Program` input port must never silently execute — the graph pipeline must reject the graph before `process()` runs (a visible error, not a revived version of the old cross-port-summing bug). An unwired `Sidechain` input port must always resolve to silence, never an error — this is expected, steady-state behavior for a control/detector port, not a fault. This enforcement lives at the graph-pipeline validation layer (editor-agent's surface, e.g. `validateRuntimeGraph.ts`), not in `backend/transform-sdk` — the SDK's `PortKind` enum only documents the contract both surfaces must honor.
- Migrations must be reversible unless there is a documented exception.
- Seeds should stay idempotent for local development.
- API and DTO changes must be reflected consistently across frontend consumers and backend producers.

## Audio correctness

- Channel layout, sample rate assumptions, and transform ordering must be explicit in code or tests.
- The transform order derived from the editor DAG must match the order executed by the frontend worklet chain.
- Playback controls such as play, pause, stop, and seek must preserve expected timing semantics.
- Transform changes need at least one deterministic verification path, even if the final UX is interactive.

## Operational discipline

- Prefer small, targeted test runs over broad unverified changes.
- If a change alters an invariant, update this file in the same change.
