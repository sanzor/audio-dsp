# Decisions

An append-only log of real product/architecture decisions — the *why we chose this*, as distinct from `agents/architecture.md`/`agents/mission.md` (the *current state*, kept up to date) and `agents/market-research.md` (evolving evidence, not yet a decision).

## Convention

- One file per decision: `NNNN-slug.md`, numbered sequentially, never reused.
- Never edit a file after its status is `Accepted` — if a decision is reversed or changed, write a *new* numbered file with status `Supersedes 000X`, and update the old file's status line to `Superseded by 000Y` (that one-line status edit is the only exception to "never edit").
- Every file has: `Status`, `Date`, `Context` (what prompted it — link to `market-research.md`/design docs if relevant), `Decision` (what was actually decided, stated plainly), `Consequences` (what living docs this changes and why — link them).

## When to write one

- A design doc reaches the point of being acted on, not just discussed.
- Any change to `agents/mission.md`'s stated product bet.
- Any change flagged under `agents/ownership.md`'s "destructive change to shared data" rule — the decision doc is where the *why* lives; the doc update in `architecture.md`/`invariants.md`/`transforms.md` is where the *current state* lives.

## Template

```markdown
# 000N: <short title>

- **Status:** Proposed | Accepted | Superseded by 000X
- **Date:** YYYY-MM-DD

## Context

What prompted this — the problem, the evidence, the design doc it came from.

## Decision

What was actually decided. State it plainly, not as options.

## Consequences

What this changes, and where — link the specific docs updated as a result
(e.g. `agents/architecture.md`'s "Surface 2: Editor" section, `agents/invariants.md`'s data-integrity section).
```
