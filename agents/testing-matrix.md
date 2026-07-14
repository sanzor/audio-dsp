# Testing Matrix

Use the smallest test slice that covers the changed risk surface.

## Frontend graph and state changes

Run when changing React Flow nodes, Zustand stores, orchestration hooks, selectors, or playback UI wiring.

- `cd frontend && pnpm test`
- `cd frontend && pnpm lint`
- manual smoke:
  - create or edit tracks/regions if relevant
  - verify node interactions remain responsive
  - verify playback UI still cleans up resources correctly

## Backend API and persistence changes

Run when changing `backend/api`, `backend/domain`, DTOs, or database-facing services.

- `cargo test`
- `cargo clippy --all-targets --all-features`
- if schema changed:
  - `make migrate-info`
  - apply the migration path in a dev database
  - verify affected frontend payload expectations

## Audio runtime and playback changes

Run when changing `backend/audiolib`, `backend/player`, transform code, sinks/sources, or playback semantics.

- `cargo test -p audiolib`
- `cargo test -p player`
- add or update a deterministic test where possible
- smoke the expected control path:
  - play
  - pause
  - stop
  - seek

## Migration and seed changes

Run when changing `database/audio_db/migrations` or `database/seeds`.

- `make migrate-info`
- `make db-seed`
- verify idempotency assumptions for changed seed scripts
- if rollback matters, test the down path or document why it does not

## Review baseline

Before closing a change, confirm:

- the relevant invariant in `agents/invariants.md` still holds
- at least one targeted automated test or deterministic manual validation exists
- docs were updated if the change altered architecture or workflow assumptions
