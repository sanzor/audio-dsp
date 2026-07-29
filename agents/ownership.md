# Ownership

## Roster layering

There are now two layers to the roster, and they answer different questions:

- **Agents** — real, invocable Claude Code subagents. Invoked via `.claude/agents/*.md`, but those are thin shims; canonical instructions live in `agents/roster/*.md` — edit there. Split into three kinds:
  - *Product shape* — `product-owner` scopes and prioritizes already-agreed work; `business-analyst` questions whether it's the right work at all (unknown-unknowns, who the product really serves, market/competitive context). Neither implements.
  - *Surface builders* — `editor-agent`/`creator-agent` own a surface end-to-end (full stack: frontend + the backend that serves it), including that surface's own unit/backend tests.
  - *Quality* — `qa-agent` owns Playwright E2E testing (`frontend/e2e/`) and screenshot capture for design review, cross-surface by nature but without ownership of application code in either surface — it reports defects to the owning surface builder rather than fixing them.
- **Technical-layer agents** (below, `audio-runtime-agent` through `observability-debug-agent`) — documentation-only ownership boundaries, not separately invocable. They describe who's responsible for a directory/subsystem when work is narrower than a full surface, or crosses surfaces (e.g. a migration, a shared audiolib fix). A surface do-er should still respect these boundaries and delegate narrow cross-cutting work rather than reaching into, say, `backend/audiolib` unsupervised.
- **Expert consultants** (`agents/consultants/*.md`) — documentation personas, not agents with ownership at all. Either layer above consults them for domain judgment (DSP correctness, graph-UI ergonomics, ideation, marketing) without handing them any file scope.

Important: `creator-ai-agent`/`editor-ai-agent` below are **planned in-product features** (chat assistants end users will interact with inside the Creator/Editor surfaces) — not the same thing as the dev-time `creator-agent`/`editor-agent` subagents that build and maintain those surfaces. Don't conflate the two when reading this file.

## Agent ownership

- `audio-runtime-agent`
  - `backend/audiolib`
  - `backend/player`
  - transform execution behavior
  - playback timing, sink/source semantics, audio correctness

- `frontend-graph-agent`
  - `frontend/src`
  - `frontend/docs/orchestrators.md`
  - React Flow nodes, graph interactions, Wavesurfer integration
  - Zustand normalization and React Query orchestration

- `creator-ai-agent`
  - creator chat UX and prompt-to-source workflows
  - transform source scaffolding, metadata scaffolding, compile handoff behavior
  - must preserve the normal creator compile and publish pipeline

- `editor-ai-agent`
  - editor chat UX and prompt-to-graph workflows
  - transform catalog consumption, region-scoped graph planning, graph save behavior
  - must preserve the normal editor graph persistence and runtime execution model

- `backend-data-agent`
  - `backend/api`
  - `backend/domain`
  - `backend/dtos`
  - project, workspace, track, graph persistence and API contract work

- `regression-review-agent`
  - review-only ownership across the whole repo
  - focuses on regressions, invariant breaks, missing tests, and bad assumptions

- `observability-debug-agent`
  - `monitoring/`
  - worker/runtime diagnostics
  - container/runtime debugging via `Makefile` targets and dashboards

## Shared zones

- `frontend/e2e/` (Playwright specs, config, screenshot output) is `qa-agent`'s — `editor-agent`/`creator-agent` should not add E2E specs there directly; ask `qa-agent` to add scenario/smoke coverage instead, same as any other cross-cutting narrow work per the technical-layer boundary above.
- `database/` is shared between `backend-data-agent` and `audio-runtime-agent` when schema changes affect runtime or transform metadata.
- AI-assisted creator/editor flows usually cross `frontend-graph-agent`, `backend-data-agent`, and either `creator-ai-agent` or `editor-ai-agent`.
- The published-transform contract (`transform_binaries`, `transform_ports`, `transform_params`) is `creator-agent`'s write side and `editor-agent`'s read side — see the escalation rule below for what a destructive change there requires.
- Root docs and workflow files are shared and should stay concise.
- Creator's client-side "Try it" preview (compiling-but-not-yet-published binary execution) depends on `frontend-graph-agent`'s worklet runtime: `frontend/src/audio/worklet/graph-worklet.js` (the `AudioWorkletProcessor`) and `frontend/src/audio/transport/WorkletMessageSender.ts` (the port message protocol) are reused directly by the creator surface, by design (see `agents/decisions/0003-transform-preview-flow.md`) — preview must run the identical worklet code the editor uses post-publish, not a hand-rolled runtime. Both are safe to reuse as-is: `graph-worklet.js` holds no module-level shared state (each `AudioWorkletNode` instance is independent) and `WorkletMessageSender` is a plain port wrapper with no store coupling. `WorkletController` (`frontend/src/audio/transport/WorkletController.ts`) and the `useWorkletSetup`/`useWorklet` hooks are explicitly **not** part of this shared surface — `WorkletController` is instantiated once at module scope inside `Stores/WorkletStore.ts` and hard-writes to editor-only global state (`useWorkletStore`, `useAudioEffectsStore`) on every connect/graph-ready/error event, and `useWorkletSetup` wires connection lifecycle to a Wavesurfer media element the creator surface doesn't have. Creator-agent must build its own thin, creator-scoped connect/disconnect wrapper on top of `graph-worklet.js`/`WorkletMessageSender` rather than importing `WorkletController` or the editor hooks. A future `frontend-graph-agent` change to the worklet message protocol (`SET_GRAPH`/`SET_BYPASS`/`UPDATE_PARAMS`, `GRAPH_READY`/`MODULE_ERROR`) or to `WorkletMessageSender`'s API must be checked against creator-side preview, not just the editor.

## Escalation rule

If a change crosses more than one ownership boundary, the builder agent should hand the result to `regression-review-agent` before it is considered done.

`editor-agent` and `creator-agent` specifically must:

- **Consult `product-owner` before starting any change that spans both surfaces**, or that touches data the other surface's agent doesn't own but depends on (e.g. shared schema, a contract one surface reads that the other writes).
- **Update the relevant doc in `agents/` in the same change** if the change is destructive to that shared data — a schema change, a removed/renamed field, a changed contract, anything that could silently break the other surface's assumptions. Which doc depends on what changed: `agents/architecture.md` for shape/boundary changes, `agents/invariants.md` for a broken or new rule, `agents/transforms.md` for anything in the compile/save/publish model. Do not leave this for `regression-review-agent` to catch after the fact — the doc update is part of the change, not a follow-up.
