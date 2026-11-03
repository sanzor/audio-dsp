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

## Cross-surface and UI-visible changes (owned by qa-agent)

Run when a change crosses the creator/editor boundary, or when a UI change needs visual confirmation beyond `pnpm lint`/`pnpm test`.

- `cd frontend && pnpm test:e2e` — Playwright smoke coverage (`frontend/e2e/smoke.spec.ts`) plus any scenario specs for the changed flow
- `cd frontend && pnpm screenshots` — automated capture into `frontend/e2e/screenshots/` for design review (see `agents/skills/design/capture-ui-screenshots.md`); never take screenshots by hand
- requires the real stack: `make up-db && make migrate && make db-seed && make up-backend`, frontend dev server on `:3000` (Playwright's `webServer` starts it if not already running)

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

## CI

`.github/workflows/ci.yml` is the entry point and calls four lanes: `backend-ci`, `backend-cd`, `frontend-ci`, `frontend-cd` (CD only on pushes to main/staging/dev). `ci-ok` is the single required check. Every job calls a script, so run the same script locally to reproduce a CI failure:

- `scripts/ci/backend/`: `audit.sh`, `clippy.sh`, `fmt.sh` (`--fix` to rewrite), `unit-tests.sh`, `integration-tests.sh` (the `#[ignore]`d wasm compile-pipeline tests; serial — concurrent compiles share one output path), `build-image.sh`
- `scripts/ci/frontend/`: `install.sh`, `lint.sh`, `test.sh`, `build.sh`, `build-image.sh` (production nginx image: `frontend/Dockerfile`; `frontend/Dockerfile.dev` is the local dev-server image)
- `scripts/ci/shared/`: image publish, GitOps manifest update, the `ci-ok` gate

Advisory ignores for `cargo audit` live in `backend/.cargo/audit.toml`, each with a reason; remove them when upstream fixes land.
