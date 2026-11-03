# Skill: Git / Commit And Push

Use this workflow when work should be committed and pushed to its own branch on `origin`.
Invocable as `/commit-and-push` (shim: `.claude/skills/commit-and-push/SKILL.md`).

## Trigger

- the user asks to commit, push, or "ship" changes
- a cleanup or fix is finished and should land on the remote

## Checklist

1. **Branch first.** Run `git branch --show-current`.
   - On `main`, `staging`, or `dev`: do not commit there — pushes to those branches trigger CD (image publish + GitOps deploy). Create a purpose branch with `agents/skills/git/create-branch.md` (`<area>/<purpose>`); uncommitted changes carry over.
   - On a branch whose purpose doesn't match this change: create a new branch for it instead of piling unrelated work on.
2. Run `git status --short` and `git diff --cached --stat` to see exactly what will be committed.
3. Stage only the files that belong to this change, by explicit path (`git add <paths>`). Never `git add -A` / `git add .` blindly: the working tree often holds unrelated in-progress work. If unrelated changes are present, say so and leave them unstaged (or ask whether they get their own branch).
4. Check the staged set for things that must never be committed: `.env` with real secrets, `target/`, `node_modules/`, `.pnpm-store/`, `.claude/settings.local.json`, large binaries, copyrighted media.
5. Write a descriptive commit message (format below). Never use placeholder messages like `c` or `fix`. Split unrelated parts into separate commits.
6. Commit, then push the branch: `git push -u origin <branch>` (`-u` on first push; plain `git push` after).
7. Confirm with `git status -sb` that the branch is in sync with its upstream.
8. If the push is rejected:
   - non-fast-forward: `git pull --rebase origin <branch>`, re-run checks, push again. Never `--force` a shared branch; `--force-with-lease` only on your own branch and only with the user's OK.
   - `Permission denied (publickey)` / auth errors: stop and report it. Don't change remotes, keys, or credential config yourself — the commits are safe locally; tell the user to push from the host, switch the remote to HTTPS, or fix SSH-agent forwarding into the dev container.
9. Never push directly to `main`/`staging`/`dev`; those are reached via `agents/skills/git/open-pull-request.md`.

## Commit message format

```
<area>: <imperative summary, max ~72 chars>

<optional body: what changed and why, wrapped at ~72 chars>
```

`<area>` is one of: `editor`, `creator`, `backend`, `frontend`, `db`, `infra`, `agents`, `repo`, `e2e`.

Examples:

- `editor: fix seek drift between waveform and worklet`
- `repo: untrack temp/ and ignore pnpm store`

## Repo touchpoints

- `.gitignore` (root), `backend/.gitignore`, `frontend/.gitignore`
- remote: `origin` (`git@github.com:sanzor/audio-dsp.git`)

## Done when

- the work is on a purpose branch (`<area>/<purpose>`), not `main`/`staging`/`dev`
- only the intended files are in the commit
- the push succeeded and `git status -sb` shows no ahead/behind

## Minimum verification

- `git show --stat HEAD` lists only the intended files
- `git status -sb` shows the branch matching `origin/<branch>`
