# Invariants

## Real-time and UI safety

- WASM compilation belongs to the backend compile-ticket pipeline, not to the editor frontend.
- The editor frontend may fetch and cache published transform binaries, but it must never request compilation as part of editor workflows.
- The editor frontend may build and derive graph artifacts locally, including transform lists, adjacency data, and region-level DAG views, as long as that work stays fast enough for normal interaction.
- Published transforms execute on the frontend through the worklet runtime, so UI-thread code must stay separate from real-time audio execution paths.
- Never put heavy DSP, decoding, or unbounded graph recomputation on the React main thread during interaction or playback.
- Never block an audio callback path with network, disk, or UI work.
- Any lifecycle around Wavesurfer or playback resources must clean up deterministically.

## State ownership

- The active source of truth for graph editing must be unambiguous for a given feature.
- During editing, the frontend may be the working source of truth for graph composition and derived region-level execution order before persistence.
- AI-assisted editor actions must resolve into explicit graph mutations, not hidden side effects outside the normal editor state flow.
- Client normalized stores and backend persisted state must not silently diverge.
- Mutations that affect parent-child trees must update both the entity and the relevant parent references.

## Data integrity

- AI-generated transform source must go through the same validation, compilation, ticketing, and publish flow as manually authored source.
- AI-assisted editor flows may only use published transforms from the catalog, not unpublished source or ad hoc binaries.
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
