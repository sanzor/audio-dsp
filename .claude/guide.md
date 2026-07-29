nvestigate this codebase thoroughly and generate an agent-oriented knowledge base for it, under .agents/ (concise, agent-facing summaries) and docs/ (longer canonical documentation). Also create/update CLAUDE.md at the repo root to auto-load this via @AGENTS.md, so it loads every session without the user needing to reference it. Do not write any of this from assumptions or generic templates — every claim must be grounded in something you actually read in this repo. If a section doesn't apply to this project (e.g. no test suite exists, no queue/broker is used, no backend exists at all in this repo), say so explicitly rather than inventing content to fill the section.

Treat every existing doc, README, and comment in the repo as a claim to verify, not a fact to inherit — stale or template-inherited docs (an unrelated demo README, a copy-pasted architecture doc from a different project) are common and actively misleading if repeated. If something reads like boilerplate from a starter template rather than a description of what's actually in this repo, say so explicitly and don't use it as a source.

## Step 0 — Scale the approach to the repo size

Before diving in, get a rough size (file count, e.g. git ls-files | wc -l or a find`/Glob` count on the main source dir). If it's small enough to read directly in a handful of greps/reads, just do that. If it's large (hundreds of files, multiple feature areas), don't try to read everything yourself — delegate Step 1 to one or more research subagents with a *self-contained, specific brief*: list exactly what you already know (tech stack, folder layout from a top-level listing), number the open questions from the list below, name concrete files/patterns to check for each, and ask for file-path-and-snippet-backed answers, not summaries. Verify a sample of the subagent's most load-bearing claims yourself (grep/read the exact files it cited) before writing anything — don't publish a claim you haven't independently confirmed at least once.

## Step 1 — Explore before writing anything

Figure out:
1. *Tech stack*: languages, frameworks, package managers, build tools (frontend and backend/services separately if it's a multi-app repo; note if it's a single client app with no backend in-repo at all — e.g. a mobile/SPA client of a remote API hosted elsewhere).
2. *Structure*: is it layered (domain/application/infrastructure) or feature-based (package-by-feature)? Where do routes/controllers, services, and data access actually live? Don't assume — check by reading the actual routing/module file and folder layout, not by pattern-matching the tech stack to a stereotype.
3. *Data model*: what's the database/ORM, migration strategy (or lack of one), and what are the core entities/tables and their relationships? If there's no local database at all (a client app persisting only to a remote API, with local storage used just as a key/value session cache), say that plainly instead of forcing a "tables and relationships" framing onto TypeScript interfaces — document the request/response model shapes instead, and name the actual persistence mechanism (localStorage, IndexedDB wrapper, mobile secure storage, SQLite, etc.).
4. *Domain*: what business concepts, roles, and core workflows does this app implement? What are the main user-facing flows end to end (e.g. auth, checkout, onboarding — whatever applies)? For unfamiliar or product-specific terms found in folder/class names, open a representative file per term and ground the definition in its actual fields/usage rather than guessing from the name alone.
5. *Integrations*: every external service actually called from the code — auth providers, file storage, email, payment, queues, third-party APIs, maps, push notifications, analytics/crash reporting, OTA/code-push, realtime brokers (MQTT/WebSocket/etc.). For each, note whether it's called from the backend or directly from the frontend/client, and what config it needs. For mobile/hybrid apps, config often lives outside .env files — check native config files too (e.g. capacitor.config.ts, google-services.json, GoogleService-Info.plist, a hardcoded config/constants class) in addition to environment files. Note if any of these constants can be overridden at runtime from a server-provided remote config — if so, say the hardcoded value is a default, not necessarily what's active.
6. *Auth & security*: how auth works (sessions/JWT/OAuth/2FA), where authorization rules live (a real per-route/role permission table, or just a binary logged-in check — check the actual guard/middleware code, don't assume granularity exists), password hashing, any existing security debt. Actively grep for hardcoded-looking credentials (API keys, broker usernames/passwords, tokens) across config files, not just `.env`/env-var patterns — mobile and legacy projects often hardcode these as source constants. Note real committed secrets explicitly and specifically (file + line), rather than a generic "secrets may be present" caveat.
7. *Testing*: what test frameworks are actually configured, and — critically — whether the test files are real or unmodified scaffolding. Don't just count *.spec.ts`/*.test.js` files; open a sample of a few (not just one) across different areas of the codebase and check if they assert real behavior or are generated boilerplate with a single trivial assertion. Check for CI config (.github/workflows/, .gitlab-ci.yml, etc.) — if none exists, say plainly that nothing runs these commands automatically. If there's no test suite (or it's all scaffolding), say that plainly instead of inventing commands or implying coverage exists.
8. *Existing docs*: check for any pre-existing .agents/, docs/, AGENTS.md, CLAUDE.md, or README content already describing the project — reconcile with it (correct anything inaccurate you find) rather than duplicating or contradicting it. Explicitly flag any existing doc that turns out to be stale, template-inherited, or describing a different/earlier version of the product.

Use targeted greps/reads, or a research subagent per Step 0, for this — don't guess.

## Step 2 — Write the structure

Create (or correct) these files:

**CLAUDE.md** (repo root) — if it doesn't already load agent instructions, set/append its content to import the pointer file:
@AGENTS.md

**AGENTS.md** (repo root) — pointer file (adjust the "Read when relevant" list to what's actually relevant for this project — drop sections that don't apply, e.g. no queues/no security-sensitive surface):
# Project Agent Instructions

Before making changes, inspect the existing code and read the relevant
documentation listed below.

## Always read

- `.agents/architecture.md`
- `.agents/domain.md`

## Read when relevant

- Database changes: `.agents/data-model.md`
- External services: `.agents/integrations.md`
- Application flows: `.agents/workflows.md`
- Tests: `.agents/testing.md`
- Security-sensitive changes: `.agents/security.md`
- Architectural changes: `.agents/decisions/`

## Working rules

- Make focused, minimal changes.
- Follow existing project conventions.
- Do not modify unrelated files.
- Preserve compatibility unless explicitly instructed otherwise.
- Add or update tests when behavior changes.
- Run applicable tests, linters, and type checks.
- Never claim validation passed unless it was actually run.
- Summarize changed files and validation performed.
- If a change affects architecture, the data model, an integration, a
  workflow, or security-relevant behavior, update the matching file(s) under
  `.agents/` (and `docs/` if applicable) as part of the same task — not as a
  follow-up. Treat stale docs as a regression, since agents are expected to
  trust these files without re-verifying them against the code.

**.agents/README.md** — index of the other files, one line each, noting that docs/ is canonical when the two conflict.

**.agents/architecture.md** — real system structure: apps/services in the repo, how they're organized (feature-based vs layered — state which, don't default to claiming "modular monolith with domain/application/infrastructure layers" unless you verified that's actually true), key infrastructure facts (DB, auth mechanism, realtime, file storage, email), and constraints agents must preserve (real ones you found in config/security setup, not generic advice).

**.agents/domain.md** — actual business concepts/entities/roles and what they mean, in plain language.

**.agents/data-model.md** — a table of real entities/tables with their key fields and relationships, plus the actual migration strategy (or lack of one).

**.agents/integrations.md** — every external service found in Step 1, how it's called (backend vs frontend-direct), and what config/env vars it needs.

**.agents/workflows.md** — the real end-to-end flows (auth, core business flows), referencing actual file/method names.

**.agents/testing.md** — the real, runnable commands for compiling/type-checking/linting/testing this specific project. Be honest if there's no test suite.

**.agents/security.md** — real authz rules (e.g. an actual route-permission table if one exists), real secret-handling conventions (including debt/gaps you found), and any security-relevant gaps (rate limiting, enumeration, etc.) worth flagging.

**.agents/decisions/** — only create ADRs for decisions actually made during this session (e.g. if you're documenting a feature you just built alongside this scaffold). Don't fabricate historical decisions you didn't witness.

**docs/architecture/system-overview.md** and **docs/architecture/backend.md** (or equivalent per-app files) — longer-form versions of the architecture summary, since .agents/architecture.md should stay short and these are the canonical detail.

## Step 3 — Sanity check

Before finishing, re-read each file you wrote and ask: "did I verify this, or did I assume it, or did I take a subagent's word for it without checking?" Fix anything that's a guess. For every concrete claim tied to a specific file (a hardcoded secret, a service's exact behavior, a "no CI exists" statement), confirm you (or a step you explicitly verified) actually opened that file — don't let a plausible-sounding subagent summary stand in unverified. Keep every file scannable in under a minute — these are meant to be read by an agent before every task, not once.