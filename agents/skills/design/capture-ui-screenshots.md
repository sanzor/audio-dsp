# Skill: Design / Capture UI Screenshots

Use this workflow when a UI change or design question needs visual evidence, not just a code read — owned by `qa-agent`.

## Trigger

- a UI/layout change needs review before or after implementation
- the user asks a design/positioning/ergonomics question that code alone can't answer
- `dag-ui-expert` or `marketing-ui-expert` needs current visual state to apply their lens

## Checklist

1. Ensure the real stack is reachable: `make up-db && make migrate && make db-seed && make up-backend`, frontend dev server on `:3000` (Playwright's `webServer` config starts it automatically if not already running).
2. Run `cd frontend && pnpm screenshots` — captures `dashboard.png`, `creator.png`, `login.png` into `frontend/e2e/screenshots/` (gitignored, regenerated each run).
3. If the question is about a state not covered by the default capture (a specific dialog, a graph with nodes, an error state), add a temporary spec under `frontend/e2e/` that drives to that state before capturing — don't hand-navigate and screenshot manually.
4. Read the resulting PNGs and apply the relevant consultant lens:
   - `agents/consultants/dag-ui-expert.md` for node-graph/canvas ergonomics questions
   - `agents/consultants/marketing-ui-expert.md` for positioning/messaging questions (once that surface exists)
5. Report findings; if a defect is found, hand it to `editor-agent` or `creator-agent` rather than fixing it here.

## Done when

- fresh screenshots exist for every state the question actually depends on
- findings are grounded in the captured images, not assumption

## Minimum verification

- screenshots were captured in this session, not reused from a stale run
- the capture ran against the real stack (real seed data), not a mocked/empty state
