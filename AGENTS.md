# Agent Entry Point

This repository uses a root entrypoint plus a dedicated top-level `agents/` folder for persistent build and review context.

## Canonical files

- `agents/mission.md`: what this product is, who it's for, and why the creator/editor split exists — read this first
- `agents/architecture.md`: current system shape, boundaries, and unresolved architecture questions
- `agents/invariants.md`: rules that changes must not violate
- `agents/ownership.md`: directory and subsystem ownership, and how the two agent layers below relate to each other
- `agents/testing-matrix.md`: what to test for each change category
- `agents/skills/*.md`: repeatable workflows and checklists
- `agents/consultants/*.md`: advisory personas (not invocable agents) for domain judgment outside software engineering
- `agents/roster/*.md`: canonical instructions for each real subagent — `.claude/agents/*.md` only shims to these
- `agents/market-research.md`: evolving evidence on the product bet — not yet decisions
- `agents/decisions/*.md`: append-only log of decisions actually made — see `agents/decisions/README.md` for the flow (design doc → business-analyst/product-owner → decision recorded here → handed to editor-agent/creator-agent → living docs updated)

## Agents (real, invocable subagents)

Invocable via the Agent tool from `.claude/agents/*.md` — but those are thin shims. The canonical instructions for each live in `agents/roster/*.md`; edit there, not in `.claude/agents/`.

Product shape (no implementation):

- `product-owner`: scopes and prioritizes already-agreed work, decides which surface(s) a request touches, hands scoped tasks to the two builders below
- `business-analyst`: questions whether the work is the right work — who the product actually serves, unvalidated assumptions, unknown-unknowns, competitive/market context. Distinct from `product-owner`, which assumes the work is worth doing and focuses on scoping it.

Surface builders:

- `editor-agent`: owns the Editor/DAW surface end-to-end (frontend + the backend that serves it)
- `creator-agent`: owns the Creator surface end-to-end (frontend + the backend that serves it)

`editor-agent`/`creator-agent` must consult `product-owner` before cross-surface work, and update the relevant doc in `agents/` in the same change if they make a destructive change to data the other surface depends on — see `agents/ownership.md`'s escalation rule.

Quality (cross-surface, doesn't own application code):

- `qa-agent`: owns Playwright E2E testing (`frontend/e2e/`) and automated UI screenshot capture across both surfaces — cross-surface scenario coverage, per-surface smoke tests, and screenshots for design review. Doesn't fix bugs it finds; hands them back to `editor-agent`/`creator-agent`. Doesn't replace single-surface unit tests, which stay with the owning surface agent.

## Expert consultants (advisory personas, not invocable agents)

Read the relevant file in `agents/consultants/` and adopt that framing when a decision needs domain judgment rather than more code:

- `sound-engineer`: DSP/audio-domain correctness — transform design, timing, terminology
- `dag-ui-expert`: node-graph UI/UX — port/edge conventions, legibility, ergonomics
- `brainstormer`: divergent-thinking mode for explicit ideation requests
- `marketing-ui-expert`: landing-page/marketing conventions, for a surface that doesn't exist yet

## Default agent set (technical layers, documentation-only)

- `audio-runtime-agent`: `backend/audiolib`, `backend/player`, transform execution, playback behavior
- `frontend-graph-agent`: React Flow, Wavesurfer, Zustand, orchestration, node UX
- `backend-data-agent`: `backend/api`, `backend/domain`, DTOs, persistence, auth, project/workspace/track APIs
- `regression-review-agent`: targeted review of risks, regressions, invariants, and missing tests
- `observability-debug-agent`: logs, dashboards, queue/worker debugging, deployment runtime signals

## Planned in-product AI features (not dev-time agents)

Not to be confused with `editor-agent`/`creator-agent` above — those build and maintain the surfaces; these are planned chat assistants that would run *inside* the product for end users:

- `creator-ai-agent`: chat-assisted transform authoring inside the creator surface; generates or edits source and metadata, then hands off to the normal compile/save/publish pipeline
- `editor-ai-agent`: chat-assisted graph composition inside the editor surface; inspects audio and region context, uses the published transform catalog, builds graph changes, and saves them through the normal editor flows

## Working rules

- Read `agents/invariants.md` before changing audio, graph, playback, or persistence behavior.
- Use `agents/testing-matrix.md` to choose the smallest test slice that still covers the change.
- Treat `agents/skills/` as command-palette workflows for recurring tasks.
- Update the relevant file in `agents/` when architecture or workflow assumptions change.

## How To Use Skills

Skills are triggered by asking for the workflow they describe. You can invoke them implicitly with a product-level request or explicitly by naming the skill.

Examples:

- implicit: "Create a transform, compile it, publish it, and make it usable in the editor."
- explicit: "Use `scenario/publish-transform-and-use-in-editor` for this new transform."

Prefer product workflows over file-level instructions. Ask for creator work, editor work, scenario validation, debugging, or testing.

## Skill Index

### Creator

- `creator/create-transform`
  - Use when authoring or revising a transform before compilation.
  - Example prompt: "Create a new saturation transform with drive and mix parameters."

- `creator/compile-and-publish-transform`
  - Use when a transform should move from source to a published editor-usable artifact.
  - Example prompt: "Compile this transform and publish it so the editor can use it."

### Editor

- `editor/add-transform-to-graph`
  - Use when a published transform must become usable as a node in the editor graph.
  - Example prompt: "Make this published transform available as a node in the region graph."

- `editor/fetch-and-cache-transform-binary`
  - Use when the editor must retrieve and cache a published transform artifact.
  - Example prompt: "Fetch this transform binary in the editor and cache it by version."

- `editor/build-region-execution-plan`
  - Use when deriving the ordered transform chain from a region graph.
  - Example prompt: "Check how this region DAG becomes the runtime execution plan."

### Scenario

- `scenario/publish-transform-and-use-in-editor`
  - Use for end-to-end creator-to-editor validation. Implemented with Playwright by `qa-agent` (`frontend/e2e/`).
  - Example prompt: "Create this transform, publish it, attach it to a region graph, and verify it runs."

### Design

- `design/capture-ui-screenshots`
  - Use for automated Playwright screenshot capture of the app, reviewed through the `dag-ui-expert`/`marketing-ui-expert` consultant lens. Owned by `qa-agent`. No manual screenshotting.
  - Example prompt: "Grab screenshots of the dashboard and creator so we can review the layout."

### Platform

- `platform/add-migration-and-seed`
  - Use when schema or seed data must change for creator/editor features.
  - Example prompt: "Add the schema and seed changes needed for transform version metadata."

### Debug

- `debug/debug-playback-desync`
  - Use when waveform state, graph state, and runtime playback drift apart.
  - Example prompt: "Debug why seek and playback timing diverge between the waveform and worklet output."

### Test

- `test/write-regression-test`
  - Use after fixing a bug or hardening a risky path.
  - Example prompt: "Write a regression test for the graph-order bug that broke worklet execution."

## Current architecture baseline

The current intended model is hybrid:

- transforms are authored in the creator surface
- creator AI may assist with source generation and editing in the UI
- WASM compilation happens on the backend through the ticket pipeline (a check only — see `agents/transforms.md`'s three-bucket model)
- transforms move from compile → save → publish as three independent, explicit steps; a successful compile never auto-publishes
- editor AI may assist with graph composition using the published transform store
- the editor may fetch and cache published transform binaries on the frontend
- the editor builds region-level graphs on the frontend
- published transform chains execute on the frontend through an audio worklet runtime

If a repo document disagrees with that model, treat `agents/architecture.md` as the canonical source and update the stale document.
