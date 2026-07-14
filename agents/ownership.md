# Ownership

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

- `database/` is shared between `backend-data-agent` and `audio-runtime-agent` when schema changes affect runtime or transform metadata.
- AI-assisted creator/editor flows usually cross `frontend-graph-agent`, `backend-data-agent`, and either `creator-ai-agent` or `editor-ai-agent`.
- Root docs and workflow files are shared and should stay concise.

## Escalation rule

If a change crosses more than one ownership boundary, the builder agent should hand the result to `regression-review-agent` before it is considered done.
