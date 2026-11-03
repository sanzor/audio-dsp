# Skill: Git / Open Pull Request

Use this workflow to get a pushed branch reviewed and merged into `main` (or `staging`).

## Trigger

- the user asks for a PR / merge request / review
- a branch from `agents/skills/git/create-branch.md` is finished and pushed

## Before opening

1. Branch follows `<area>/<purpose>` and contains only that purpose:
   `git log --oneline origin/main..HEAD` and `git diff --stat origin/main...HEAD`.
2. Rebase on the latest base if it moved: `git fetch origin && git rebase origin/main`
   (only on branches nobody else has pulled; otherwise merge `origin/main`).
3. Run the CI scripts for the touched lane locally, so the PR doesn't open red:
   - backend: `scripts/ci/backend/{fmt,clippy,unit-tests,integration-tests,audit}.sh`
   - frontend: `scripts/ci/frontend/{lint,test,build}.sh`
   - pick the smallest slice from `agents/testing-matrix.md`
4. Branch is pushed: `git push -u origin <branch>` (see `commit-and-push.md`).

## Title

Same format as commit messages: `<area>: <imperative summary>` (max ~72 chars).
For a single-commit PR, reuse the commit subject.

## Body template

```markdown
## Summary
<1–3 sentences: what this PR does and why>

## Changes
- <grouped by surface/area; mention renamed/deleted files explicitly>

## Testing
- <which scripts/tests ran and their result; what was NOT verified and why>

## Risk / rollout
- <migrations, dependency upgrades, config or secret changes, CD effects, data the other surface depends on>

## Follow-ups
- <known issues found but deliberately not fixed here>
```

Be factual in Testing: name what passed, and say plainly what could not be run (e.g. "Docker image not built locally").

## Creating it

- With GitHub CLI:
  `gh pr create --base main --head <branch> --title "<title>" --body-file <file>`
- Without it: open
  `https://github.com/sanzor/audio-dsp/compare/main...<branch>?expand=1`
  and paste the title and body.

Base is `main` unless the change targets `staging` only.

## Merging

- Required check is **`ci-ok`** (aggregates every lane in `ci.yml`); don't merge while it's red or pending.
- Merging to `main` triggers CD for the changed lanes — say so in Risk / rollout when it matters.
- Cross-surface or destructive data changes need the doc update required by `agents/ownership.md` in the same PR.
- After merge: delete the branch (`git push origin --delete <branch>`, `git branch -d <branch>`) and `git switch main && git pull`.

## Done when

- PR is open against the right base with the title/body format above
- `ci-ok` is green (or failures are explained in the PR)

## Minimum verification

- PR page shows only the intended commits and files
- Testing section matches what was actually run
