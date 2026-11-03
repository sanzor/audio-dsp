# Skill: Git / Commit And Push

Use this workflow when work should be committed and pushed to `origin`.

## Trigger

- the user asks to commit, push, or "ship" changes
- a cleanup or fix is finished and should land on the remote

## Checklist

1. Run `git status --short` and `git diff --cached --stat` to see exactly what will be committed.
2. Stage only the files that belong to this change, by explicit path (`git add <paths>`). Never `git add -A` / `git add .` blindly: the working tree often holds unrelated in-progress work.
3. Check the staged set for things that must never be committed: `.env` with real secrets, `target/`, `node_modules/`, `.pnpm-store/`, `.claude/settings.local.json`, large binaries, copyrighted media.
4. Write a descriptive commit message (format below). Never use placeholder messages like `c` or `fix`.
5. Commit, then `git push origin <branch>`.
6. Confirm with `git status -sb` that the branch is in sync with its upstream.

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

- only the intended files are in the commit
- the push succeeded and `git status -sb` shows no ahead/behind

## Minimum verification

- `git show --stat HEAD` lists only the intended files
- `git status -sb` shows the branch matching `origin/<branch>`
