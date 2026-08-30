# Invariants

## Real-time and UI safety

- WASM compilation belongs to the backend compile-ticket pipeline, not to the editor frontend.
- The editor frontend may fetch and cache published transform binaries, but it must never request compilation as part of editor workflows.
- The editor frontend may build and derive graph artifacts locally, including transform lists, adjacency data, and region-level DAG views, as long as that work stays fast enough for normal interaction.
- Published transforms execute on the frontend through the worklet runtime, so UI-thread code must stay separate from real-time audio execution paths.
- Never put heavy DSP, decoding, or unbounded graph recomputation on the React main thread during interaction or playback.
- Never block an audio callback path with network, disk, or UI work.
- Any lifecycle around Wavesurfer or playback resources must clean up deterministically.
- Post-compile metadata introspection (`backend/api/src/ticket_worker/processor/metadata_introspector.rs`) briefly executes attacker-controlled wasm server-side. It must stay fuel-limited, use zero host imports, and never grant WASI/IO access.

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
- The three buckets — compile (`transform_tickets`/`transform_resources`), save (`transform_saved_state`), publish (`transform_binaries`/`transforms`/`transform_ports`/`transform_params`) — must stay independently writable. Compile must never write to save or publish state; save must never trigger a compile or a publish; publish must never compile, only bundle what save already has. See `agents/transforms.md` for the full model.
- **A composite draft's explicit validate action (`transform_draft.is_validated`, `POST /transforms/{id}/validate`) is a fourth, independent checkpoint — not a fifth bucket and not a dependency between the existing three.** `save_composite_draft` must never validate the graph or derive `ports`; it persists `graph_definition` structurally and unconditionally resets `is_validated = false`. `publish_transform`'s composite branch must check `is_validated == true` as a publish precondition, failing fast with a distinct validation error if not — and, once that gate passes, must still keep re-running `composite_validator::validate_composite_graph` from scratch before actually publishing, since a leaf transform may have been unpublished or deleted since the last Save/Validate. The gate and the re-validation are both required and neither substitutes for the other. See `agents/decisions/0008-publish-requires-validated-composite-draft.md` (superseding item 4 of `agents/decisions/0007-composite-draft-validation-gate.md` — 0007's items 1-3/5-7 on Save/Validate/the flag itself are unaffected) and `agents/transforms.md`'s "Composite draft validation" section.
- Publish must fail validation if `transform_saved_state` holds a binary that doesn't correspond to its current saved source (possible because a source-only save leaves a previously-attached binary untouched). A published transform's advertised source must always match what its published binary actually does. Enforced by comparing `transform_saved_state.wasm_source_code` (a snapshot taken at attach time) against `transform_saved_state.source_code` in `publish_transform`. See `agents/decisions/0002-transform-draft-lifecycle-decisions.md`.
- A transform may only be deleted if it has never been published (no row in `transform_binaries`). Once published at least once, it must not be deletable through the draft-management path, since editor graphs may already reference it. Enforced in `PostgresTransformsDataProvider::delete_transform` (checks `transform_binaries` before deleting, returns `Conflict`/409 otherwise); cascade to `transform_saved_state`/`transform_tickets`/`transform_resources`/`transform_ports`/`transform_params` relies on existing `ON DELETE CASCADE` FKs.
- The only transform field the creator frontend may let a user edit directly is source text. During the temporary Creator handoff, it may additionally return a successful ticket's exact `{source_code, wasm_base64}` package to `PUT /draft_transforms/{id}/save-primitive`; Save re-introspects the supplied WASM under the normal fuel/zero-import limits and writes the resulting source/binary/metadata snapshot atomically. The frontend sends that package only while its source exactly matches the active editor buffer. A source-only save preserves an older binary snapshot, which Publish must reject as stale. This temporary flow does not prove that a submitted binary was produced from its submitted source; see decision 0009.
- The creator's "Try it" preview (a just-compiled, not-yet-saved/published binary run client-side) must execute through the same worklet module and message protocol the editor uses post-publish (`graph-worklet.js` / `WorkletMessageSender`), never a separate hand-rolled runtime, and must never reuse the editor's stateful `WorkletController`/`useWorkletSetup` (which write into editor-only global state and assume a Wavesurfer media element). See decision 0009 and `agents/ownership.md`'s Shared zones.
- **Port kind and cardinality are part of a transform's compile-derived definition, same as name/description/ports/params.** `transform_ports.kind` (`program` | `sidechain`) and `.cardinality` (`single` | `many`) are introspected from `metadata()` at compile time, never hand-entered, and validated by `metadata_introspector.rs`: output ports must be `program`/`single`, a transform must declare exactly one output port, and port names must be unique within a direction. See `agents/transforms.md`'s ABI contract section and `agents/decisions/0004-multi-input-named-ports.md`.
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
