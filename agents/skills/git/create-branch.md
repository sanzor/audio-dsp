# Skill: Git / Create Branch

Use this workflow before starting any change that will be committed, so the work lands on a branch named for its purpose instead of on `main`.

## Trigger

- the user asks to start a feature, fix, refactor, or chore
- you are about to commit and the current branch is `main`, `staging`, or `dev`
- a branch already exists but holds unrelated work (one purpose per branch)

## Why this matters here

Pushes to `main`, `staging`, and `dev` run the full pipeline **including CD**: images are published and the GitOps config repo is updated for that environment (see `.github/workflows/ci.yml`). Work must reach those branches through a PR, never by a direct push.

## Naming

```
<area>/<short-purpose-in-kebab-case>
```

`<area>` is the same set as commit messages (`agents/skills/git/commit-and-push.md`):
`editor`, `creator`, `backend`, `frontend`, `db`, `infra`, `agents`, `repo`, `e2e`.

- purpose is what the branch achieves, not how: `editor/fix-seek-drift`, not `editor/change-store`
- 2–5 words, lowercase, hyphens only, no ticket-less abbreviations
- cross-surface work: pick the area that owns the main change, and consult `product-owner` first (see `agents/ownership.md`)

Examples:

- `infra/ci-pipeline-and-fixes`
- `creator/compile-timeout-setting`
- `backend/serialize-transform-builds`
- `db/transform-version-metadata`

## Checklist

1. `git status --short` — note any uncommitted changes. They carry over to the new branch; that's fine, but only commit what belongs to this purpose (stage by explicit path).
2. `git fetch origin` so the base is current.
3. Create from the up-to-date base, normally `main`:
   `git switch -c <area>/<purpose> origin/main`
   (branch from `origin/staging` only when the fix targets staging specifically.)
4. Check the name isn't already taken: `git branch -a --list '*<purpose>*'`.
5. Confirm: `git status -sb` shows the new branch.
6. First push sets upstream: `git push -u origin <area>/<purpose>`.

## Renaming

If a branch has the wrong name and has **not** been pushed: `git branch -m <new-name>`.
If it was pushed, create the new name, push it, open the PR from it, then delete the old remote branch — only after confirming with the user.

## Done when

- you are on a branch named `<area>/<purpose>`, based on current `origin/main`
- nothing unrelated has been committed to it

## Minimum verification

- `git status -sb` shows the branch
- `git log --oneline origin/main..HEAD` lists only this branch's commits
