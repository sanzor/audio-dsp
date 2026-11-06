# Codebase Audit and Plan of Action

_Date: 2026-11-06 · Reviewed: `60bf510` (`editor/worklet-stereo-and-fault-isolation`) · Supersedes `CODEBASE_REVIEW.md` (2026-11-05)_

## Summary
**Not deployable and not safe for real users at the moment.**

- **Auth is broken at the root.** Login never checks the password, `/users` has no auth, and the workspace role comes from a header the client controls.
- **Deploy is broken at every stage.** CI fails `cargo fmt`, CD can't push, the Helm chart doesn't render, and the backend image crashes on boot.
- **Audio has user-audible correctness bugs.** Param order gets scrambled after a reload, there is no NaN or overload protection, and audio drops out dry on every param edit.

Commit `60bf510` correctly fixed stereo, catching transform traps, and allocations in the JS hot path.

Tags: **[V]** = verified in code; **[I]** = inferred (runtime or browser behaviour that wasn't executed).

---

## Part 1 — Report

### A. Security (backend)

| # | Sev | Issue | Evidence |
|---|---|---|---|
| S1 | Critical | **Login never verifies the password.** Anyone who knows an email gets that user's session. The client sends `password_hash`, which is stored as-is, so it works like a plaintext password. `is_active` is also not checked. [V] | `backend/api/src/auth/auth_provider_service.rs:59-76` |
| S2 | Critical | **`/users` CRUD has no JWT.** The only middleware is `PermissionsContextMiddleware::allow_all()`. Anyone can list, delete, or `PUT` a new password on any user. [V] | `main.rs:460,515-519`, `controllers/user_controller.rs` |
| S3 | Critical | **Cross-workspace privilege escalation.** The membership middleware sits on the outer `/v1/workspaces` scope, so the `{workspace_id}` path match isn't available there and the role is resolved from the `X-Project-Id` header. Handlers then act on the *path* workspace. The owner of workspace 1 can delete or invite themselves into workspace 2. [V code; actix ordering checked in crate source] | `middlewares/membership/membership_middleware_service.rs:62-71`, `main.rs:527-529`, `controllers/workspace_controller.rs` |
| S4 | Critical | **The transform compile leaks server secrets.** _Env-var leak fixed 2026-11-06 (`env_clear()` + allowlist, regression tests in `api/tests/transform_compile_pipeline.rs`). Still open: same-uid file reads such as `include_str!("/proc/<api-pid>/environ")` or config files, which need a separate user or sandbox._ `cargo` inherits the full environment (`JWT_SECRET`, `DATABASE_URL`, `GOOGLE_CLIENT_SECRET`), and stderr is returned to the user. `compile_error!(env!("JWT_SECRET"))` leaks it, and with that secret anyone can forge admin tokens. The process runs as root. [V] | `transform_drafts/processor/build_job.rs:153-161,179-183`, `backend/api/Dockerfile` (no `USER`) |
| S5 | High | **Concurrent compiles can return each other's binaries.** All compiles share one target dir and the same `transform_build.wasm` path, which is read after cargo's lock is released. There is no concurrency cap; compiles run inline in the HTTP request with a 60 s timeout. [V code / I race] | `build_job.rs:118-126`, `processor/processor.rs:36-58`, `main.rs:407-410` |
| S6 | High | **IDOR on graphs, regions and region-sets.** Queries filter by id only, and `get_region_sets()` returns every row in the DB. [V] | `graphs/data_provider/…:51-130`, `regions/data_provider/…:32-112`, `region_sets/data_provider/…:34-130` |
| S7 | High | **Sources are global.** Any member can download or delete anyone's uploaded audio. [V] | `sources/data_provider/sources_data_provider_service.rs:22-75`, `controllers/sources_controller.rs:147,214,254` |
| S8 | High | **The JWT middleware accepts refresh, verification and invite tokens as access tokens.** `purpose` is never checked. Refresh copies `is_admin` without re-checking the DB, so a deleted user keeps access. The mock email sender logs tokens at INFO. [V] | `middlewares/jwt/jwt_middleware_service.rs:51-65`, `controllers/auth_controller.rs:274-306`, `auth/mock_email_sender.rs:9` |
| S9 | High | **Viewers can upload tracks.** The `can_edit()` check is commented out. [V] | `controllers/tracks_crud_controller.rs:95-104` |
| S10 | High | **CSRF and WebSocket hijack exposure.** Cookies are `SameSite=None`, the JWT middleware falls back to the cookie, the CSRF extractor is never used, and the WS upgrade doesn't check Origin. Fixing S3 removes the header that currently blocks this by accident. [V/I] | `auth_controller.rs:15-21`, `jwt_middleware_service.rs:42`, `ws_controller.rs:53-87` |
| S11 | Medium | **Weak secret defaults.** `JWT_SECRET=test_secret` is the compose default and is in the tracked `dev.env`. The secret is `expect`ed on every request instead of once at boot. `temp/backend/config/prod/settings.json` with prod-looking DB and Redis passwords is in git history. `invite_member` accepts `SuperAdmin`. [V] | `yml/backend.yml:12`, `backend/api/dev.env`, `token/token_utils.rs:80,106`, `workspace_controller.rs:66-69` |

### B. Deployment and CI/CD

| # | Sev | Issue | Evidence |
|---|---|---|---|
| D1 | Critical | **The backend image panics on startup.** The config file is required at `{APP_CONFIG_PATH\|CARGO_MANIFEST_DIR\|.}/config/{APP_ENV}/settings`, but the image copies only the binary and sets no `APP_CONFIG_PATH`. There is no `config/prod`. [V] | `backend/api/src/config.rs:25-43`, `main.rs:124`, `Dockerfile:57-58` |
| D2 | Critical | **`DATABASE_URL` is ignored.** The config reads `APP__DATABASE__URL`, while compose and Helm set only `DATABASE_URL`. The pod would try `localhost:5433`. [V] | `config.rs:40`, `yml/backend.yml:13`, `helm/rust-api/templates/deployment.yaml:46-50` |
| D3 | Critical | **CI can never pass.** `cargo fmt --check` fails (5 diffs), so lint fails and build, test and publish are all skipped. `quick-check` fails the same way, and also because it lacks `libasound2-dev`. [V] | `transform_drafts_controller.rs:204`, `transform_drafts/dto/requests_tests.rs`, `.github/workflows/quick-check.yml` |
| D4 | Critical | **The Helm chart cannot render.** `{{-define`, a stray `{{ -define … }}` and `{{ - end -}}` are invalid syntax. [V] | `helm/rust-api/templates/_helpers.tpl:5,19-21` |
| D5 | Critical | **`cd.yml` never pushes and can be injected.** It sets `persist-credentials: false` and then runs `git push`. `head_branch` is interpolated into shell. `workflow_run` fires for PR, dev and feature branches, but images are only published for `main`. The `sed` pattern only matches once. `workflow_dispatch` can never run. [V/I] | `.github/workflows/cd.yml:14-59` |
| D6 | High | **No frontend CI and no production frontend.** No tsc, eslint, vitest, build or Playwright runs in CI. `frontend/Dockerfile` runs `pnpm dev`. There is no nginx and no Helm chart for the frontend. CORS only allows localhost. [V] | `ci.yml`, `frontend/Dockerfile:14`, `main.rs:467-479` |
| D7 | High | **Nothing applies migrations in a deploy.** They run only via `make migrate` on the host, and the migrations directory is outside the backend Docker context. [V] | `Makefile:207-211` |
| D8 | High | **Helm values don't match the app.** The image repo is `your-org/rust-api` but CI pushes `sanzor/audio-dsp`. The `*-latest` tags are never pushed. The chart sets `FRONTEND_REDIRECT`, but the app reads `FRONTEND_URL`. The required Secrets are never created. There are no probes and no health endpoint. Resources are `{}`. There is no volume for the compile cache or build dir. [V] | `helm/rust-api/values*.yaml`, `templates/deployment.yaml` |
| D9 | High | **The integration-tests job is dead code.** It is gated on a non-existent `tests/integration/`, the `postgres` hostname is unreachable from `docker run`, `--test-mode` doesn't exist, and `ENTRYPOINT` makes the command `app /usr/local/bin/app …`. The real `#[ignore]` compile tests never run. [V] | `ci.yml:70-110`, `backend/api/tests/transform_compile_pipeline.rs` |
| D10 | High | **`make up` and the backend-only targets fail.** `backend.yml` depends on `postgres` but is composed without `postgres.yml`. [V] | `Makefile:114,151,172,186,200` |
| D11 | Medium | **The image is 2.09 GB and runs as root.** It has a double toolchain, no dependency layer cache, and ALSA and X11 libs that are build-only. `rust:slim` (trixie) is paired with a `bookworm` runtime, which risks a glibc mismatch. The SDK pre-warm uses a different profile than real compiles, so it doesn't warm anything. [V/I] | `backend/api/Dockerfile` |
| D12 | Medium | **Monitoring is non-functional.** Prometheus scrapes a `/metrics` endpoint that doesn't exist. The dashboards come from another project. Grafana is admin/admin. [V] | `monitoring/`, `yml/monitoring.yml` |
| D13 | Info | **The unmerged `infra/ci-pipeline-and-fixes` branch already fixes D3, D5, D6 (CI and an nginx image), D9 (partly), and quick-check.** It doesn't touch D1, D2, D4, D7, D8, D11 or CORS. Its own bugs: the nginx upstream `rust-api:3080` doesn't match the Helm service name, the gitops `sed` rewrites every ghcr image, and there is no container-starts smoke test. | `git diff main...infra/ci-pipeline-and-fixes` |

### C. Frontend audio playback (worklet / transport)

**How audio flows today:**
1. `apiGetStoredTrack` downloads the whole file as a blob.
2. `AudioCacheStore` keeps it plus an object URL forever.
3. WaveSurfer refetches and decodes it for peaks, and its `<audio>` element is the transport clock.
4. The signal path is `MediaElementSource` → `AudioWorkletNode('graph-worklet', 2ch)` → `destination`.
5. Effects apply only while a region with a graph is selected.
6. WASM binaries are cached by `transformId`, transferred on `SET_GRAPH`, and compiled and instantiated inside the worklet.

| # | Sev | Issue | Evidence |
|---|---|---|---|
| F1 | Critical | **Node params reach WASM in the wrong order after a save and reload.** Params are `Object.values(node.params)`, but transforms read them by position. The backend stores `HashMap<String,f32>` in JSONB, so key order is lost. Example: the compressor gets ratio as threshold. [V] | `frontend/src/audio/pipeline/GraphCompiler.ts:335`, `backend/domain/src/graphs/node.rs:34` |
| F2 | Critical | **No NaN, Inf or overload protection.** Transform output goes straight to `output[c]`, and feedback edges carry it on. A gain ≥1 loop or a buggy transform blasts the speakers at full scale, and a NaN persists in state. [V] | `audio/worklet/graph-worklet.js:355-360` |
| F3 | High | **Every graph load goes dry and resets effect state.** `_loading` forces bypass, which contradicts the comment that the old graph keeps running. Each param edit sends 2× `SET_GRAPH` (compile twice) instead of `UPDATE_PARAMS`, giving an audible dry gap, lost tails, and clicks. [V] | `graph-worklet.js:285,340`, `canvas-panel.tsx:431`, `useSaveCompileFlow.ts:129` |
| F4 | High | **Failed modules are reported as ready.** After `MODULE_ERROR` the worklet still posts `GRAPH_READY`, and `WorkletController` overwrites the error with `playable:true`. The chain is silent while the UI says ready. [V] | `graph-worklet.js:299-329`, `audio/transport/WorkletController.ts:13-17` |
| F5 | High | **Editor multi-input ports (sidechain) can't be wired.** `toPortId: 0` is hard-coded, so all edges are summed into port 0. This diverges from the creator preview. [V] | `buildRuntimeGraph.ts:56` |
| F6 | High | **A stale graph survives a track or region switch.** On reconnect only `setEffects` is re-sent, the old `compiledGraph` is re-uploaded, and there is no bypass when `activeGraphId` is null. [V/I] | `audio/hooks/useWorkletSetup.ts:44`, `useWorklet.ts:13-17`, `WaveformPlayer.tsx:75-77` |
| F7 | High | **Creator Play/Stop races.** `audioCtx` is assigned only after `await addModule`. Stop during load does nothing, and a double Play leaks a context. [V] | `Stores/CreatorPlaybackStore.ts:379-462`, `components/creator/creatorTransformPlayback.ts:69-84` |
| F8 | High | **Long tracks: full download before the first sound, and memory blow-up.** Blob plus WaveSurfer copy plus decode plus 8 kHz peaks comes to about 635 MB for a 1 h WAV, and the cache is never evicted. [V/I] | `Services/TracksService.ts:79-96`, `AudioCacheStore`, `waveformEngine.ts` |
| F9 | Medium | **No request ids in the worklet protocol.** A stale `MODULE_ERROR` can mark a good graph broken. There is no ack for bypass or params. [V] | `graph-worklet.js`, `WorkletMessageSender.ts` |
| F10 | Medium | **`_loadGraph` can wedge in permanent dry mode.** It has no try/finally, so a throw leaves `_loading=true` forever, silently. [V] | `graph-worklet.js:285-327` |
| F11 | Medium | **WASM compile and instantiate run on the audio render thread**, with no module cache across loads. [I] | `graph-worklet.js:302-305` |
| F12 | Medium | **Binaries aren't versioned and the ABI is unchecked.** A republished transform plays the old binary until reload, and `transform_abi_version` is never checked. [V] | `hooks/transforms/queries.ts:20-43` |
| F13 | Medium | **Transforms don't know the sample rate.** The ABI has no `sample_rate`, so time and Hz-based effects differ at 44.1 vs 48 kHz. [V] | `transform-sdk` export ABI |
| F14 | Medium | **No state reset on seek, and bypass toggles are hard switches (clicks).** Region end comes from WaveSurfer's ~16 ms timer `pause()`, so it is not sample-accurate and cuts hard. [V] | `graph-worklet.js`, wavesurfer `play(start,end)` |
| F15 | Medium | **`AudioContext.resume()` is never called in the editor.** Under autoplay policy (Safari) the cursor moves but there is silence. [V/I] | `useWorkletSetup.ts:21` |
| F16 | Medium | **The volume slider sits *before* the effects.** It is the media element's volume, so it changes the compressor and saturation drive. [I] | `waveRef.setVolume` |
| F17 | Medium | **Creator input sources can double up.** A late mic stream is never stopped, and dragging restarts the buffer source on every move (clicks). [V] | `creatorTransformPlayback.ts:88-156` |
| F18 | Low | Leaked AudioContexts in `audio/utils.ts:13` and `create-track-modal.tsx:98`. The upload re-encodes to WAV but keeps the original extension. `BLOCK_SIZE=128` is hard-coded. There's no output-latency compensation on the cursor. `AudioCommandDispatcher.ts` is dead. The Rust `process` allocates 2 `Vec`s per block. | various |

### D. Backend audio pipeline (upload → storage → stream)

**How audio flows today:**
1. Multipart upload is read fully into memory.
2. WAV is decoded with hound as 16-bit only; anything else is treated as raw f32 using the sample rate and channel count the client claims.
3. Everything is re-encoded to 16-bit WAV.
4. It is stored in Postgres `BYTEA` (`track_storage.data`).
5. `GET …/stored-tracks/get` loads the whole blob and returns it.

| # | Sev | Issue | Evidence |
|---|---|---|---|
| A1 | High | **No upload size limit, and 3–4 copies in RAM.** `PayloadConfig` doesn't apply to `Multipart`. A multi-GB POST OOMs the process. `bytea` caps at 1 GB. [V] | `tracks/multipart_audio_parser/…:21-104`, `tracks_provider_service.rs:104`, `main.rs:484` |
| A2 | High | **Panics on ordinary input.** 24-bit and float WAV (the common DAW exports) hit `s.unwrap()`, and a client disconnect hits `chunk.unwrap()`. Also: a UTF-8 byte slice of compile errors, and a WASM `len` of `-1` cast to `usize::MAX`. [V] | `backend/audiolib/src/utils.rs:36-37`, `multipart_audio_parser_service.rs:101`, `build_job.rs:199`, `wasm/wasm_parser.rs:67-71` |
| A3 | Medium | **The stream endpoint isn't a media endpoint.** It has no Range/206 or `Accept-Ranges`, loads the whole blob per request, and sends no ETag or Cache-Control. **The Content-Type is wrong:** it's guessed from the user-typed extension while the bytes are always WAV, and an `html` extension serves `text/html`. `Content-Disposition` is unescaped. [V] | `controllers/track_payload_controller.rs:25-58`, `sources_controller.rs:254-287` |
| A4 | Medium | **CPU-heavy work on async workers.** WAV decode and encode, and wasmtime `Module::new`, run inline with no `spawn_blocking`. [V] | `multipart_audio_parser_service.rs:51-76`, `wasm_parser.rs:31-36` |
| A5 | Medium | **Client audio metadata isn't validated.** `sample_rate` 0, NaN or negative is accepted, odd stereo sample counts are accepted, and a truncated upload is silently accepted. [V] | `multipart_audio_parser_service.rs:28-79` |
| A6 | Medium | **Multi-step writes aren't atomic.** Track meta and blob go in two statements with a best-effort delete. Graph `version` is never compared, so the last write silently wins. [V] | `tracks_provider_service.rs:98-139`, `graphs_data_provider_service.rs:84-100` |
| A7 | Medium | **The leftover server-side player is still wired in.** The cpal player, actors, `/player/*` and `/ws/run` are dead and broken (the WS can't send the header, closes on ping, and has an unbounded queue), and they force ALSA into the build. [V] | `main.rs:261-264,554-558`, `backend/player`, `backend/actors` |

### E. Structure and hygiene
- **Two Cargo workspaces that disagree:** root vs `backend/`. `transform-sdk` and `dtos` aren't in the backend workspace, and there are two lockfiles. [V]
- **Stale duplicate migration folders:** `database/audio_db/migrations/0001_init/` and `0002_auth/`. [V]
- **Copyrighted MP3s in `samples/` (~76 MB):** a licensing risk if the repo goes public.
- **Two frontend lockfiles:** `frontend/` has both `package-lock.json` and `pnpm-lock.yaml`. [V]
- **Test gaps:** none of F1, F3–F7, F9 or F10 is covered, there is no WaveSurfer↔context↔region integration test, and `audiolib`, `domain` and `player` have zero tests.

---

## Part 2 — Plan of Action

Each step is one small branch. Owners follow `agents/ownership.md`:
- **backend** = backend-data / audio-runtime layer
- **editor-agent** = Editor surface
- **creator-agent** = Creator surface
- **infra** = CI and deploy

Cross-surface items go through `product-owner` first.

### Phase 0 — Stop the bleeding (security; before any real user or public deploy)
1. **Auth basics** (backend): server-side argon2 hashing and verification on login, reject inactive users, a one-off migration to rehash or force-reset existing rows. (S1)
2. **Lock `/users`** (backend): JWT plus an admin check, or delete the routes. (S2)
3. **Membership from the path** (backend): move the middleware onto the inner `/{workspace_id}` scope. Reject a header/path mismatch, then drop the header fallback. Reject `SuperAdmin` in invites. (S3, S11)
4. **Compile isolation, minimum** (backend): `env_clear()` plus an allowlist, a per-job `CARGO_TARGET_DIR` subfolder or a unique package name (or read the artifact under a lock), a `tokio::Semaphore` cap, a non-root `USER` in the Dockerfile, and cleanup of job dirs on cancel. (S4, S5, L6)
5. **Tenant scoping** (backend): pass `WorkspaceContext` into the graph, region, region-set and source providers. Join to `tracks.workspace_id` in every query. Delete the unscoped `get_region_sets()`. Restore `can_edit()` on upload. (S6, S7, S9)
6. **Token purpose** (backend): the middleware requires access tokens only, refresh re-reads the user from the DB, and the mock sender stops logging tokens. (S8)
7. **CSRF / WS** (backend): `SameSite=Lax` or enforce the CSRF token, and check Origin on the WS. Do this together with step 3. (S10)
8. **Secrets** (infra): load `JWT_SECRET` once at boot and refuse weak values. Rotate the Google secret and any secret that was in `temp/` history. (S11)

*Tests:* an integration test per item: wrong password → 401; a cross-workspace request → 403/404; a refresh token used as access → 401; `env!("JWT_SECRET")` in a transform does not leak; two concurrent compiles each get their own binary.

### Phase 1 — Make it deployable
1. **Rebase and merge `infra/ci-pipeline-and-fixes` onto HEAD**, plus `cargo fmt --all`. This fixes D3, D5, D6 and quick-check. While doing it, fix that branch's nginx upstream name and its gitops `sed` scope. (D3, D5, D6, D13)
2. **Backend boots in a container:** `COPY api/config /app/config`, `ENV APP_CONFIG_PATH=/app`, add `config/prod`, and accept `DATABASE_URL` explicitly (or switch compose and Helm to `APP__DATABASE__URL`). (D1, D2)
3. **A CI smoke test that the container actually starts**, against a Postgres service, hitting a new `GET /healthz` and `/readyz` (DB ping). (D1, D9)
4. **Migrations:** embed `sqlx::migrate!` behind `RUN_MIGRATIONS=true`, or add a Helm pre-upgrade Job. (D7)
5. **Helm:** fix `_helpers.tpl` and run `helm lint` in CI. Align the image repo and tags with CI. Fix the env names (`FRONTEND_URL`, `APP_ENV`, the DB var). Document or create the Secrets. Add probes, resources and ephemeral-storage limits, and a PVC or emptyDir for the compile cache and build dir. Add a frontend chart or nginx deployment plus an ingress. (D4, D8)
6. **CORS from config** (an allowed-origins list). (D6)
7. **Replace the integration job** with the real `--ignored` compile tests on a runner with the wasm32 target. (D9)
8. **Makefile:** always compose `postgres.yml` with `backend.yml`. (D10)

*Done when:* a push to `main` goes green, pushes the backend and frontend images, the GitOps commit lands, the chart renders, the pod passes readiness, and migrations apply on a fresh DB.

### Phase 2 — Audio correctness (frontend worklet/transport; editor-agent, creator preview to creator-agent)
1. **Param order:** build the params array from the transform definition's `param_order` (look up by name, fall back to the default). Unit test with shuffled keys. (F1)
2. **Output safety:** per-lane non-finite check (zero the buffers, count a fault, reset feedback buffers), plus a hard clip or limiter at the worklet output. (F2)
3. **Seamless graph swaps:** keep the old graph running while the new one loads, crossfade over a few blocks on swap, send `UPDATE_PARAMS` when the topology is unchanged, and compile once per submit. (F3)
4. **Honest status:** add a `requestId` to every message and echo it back. `GRAPH_READY` carries the failed nodes, and the controller doesn't overwrite errors. Wrap `_loadGraph` in try/finally and post an error. (F4, F9, F10)
5. **Port mapping:** map `edge.targetHandle` to the real port id in `buildRuntimeGraph`, so the editor and creator agree. (F5)
6. **Graph lifecycle:** key the compiled graph by `graphId`, and send bypass or an empty graph when `activeGraphId` changes or is null. Re-sync the full state on reconnect. (F6)
7. **Transport polish:** call `ctx.resume()` in the play handler, put a post-worklet `GainNode` for volume, send a `RESET` on seek, and use short fades on bypass toggles and region stops. (F14–F16)
8. **Creator session guards:** a session token or `AbortController` checked after every await, the context assigned before awaiting, and a guarded `attachInputSource`. Stop late mic streams. (F7, F17) — creator-agent.
9. **Binary versioning:** key the cache by `(transformId, version)`, check `transform_abi_version` in the worklet, and surface fetch failures. (F12)

*Tests:* extend `graph-worklet.test.ts` to cover NaN injection, swapping without a dry gap, a stale `MODULE_ERROR` being ignored, and `_loadGraph` throwing. Add unit tests for `GraphCompiler` param order and for `buildRuntimeGraph` ports. Add a Playwright scenario: edit a param while playing and assert there are no silent or dry blocks via an analyser tap (qa-agent).

### Phase 3 — Audio delivery and scale (backend + editor)
1. **Correct media endpoint:** always send `audio/wav`, support Range/206 and `Accept-Ranges`, add a strong ETag and `Cache-Control: private`, `nosniff`, and a sanitized filename. (A3)
2. **Upload hardening:** enforce a byte cap while streaming, handle or reject 24-bit and float WAV explicitly (no panics), validate `sample_rate` and channels, stop accepting truncated uploads, and use `spawn_blocking` for decode, encode and wasmtime. (A1, A2, A4, A5)
3. **Atomic writes:** one transaction for track meta plus the blob, and optimistic locking on graph `version` (409 on mismatch). (A6)
4. **Stop the double download in the frontend:** the backend serves precomputed peaks. WaveSurfer uses `peaks` plus `duration` with a streamable URL instead of a blob, with an LRU cache and `revokeObjectURL`. (F8) — needs the backend change first, so it goes via product-owner.
5. **Longer term:** move audio blobs out of Postgres into object storage with signed URLs, and move compiles to a sandboxed worker queue (no network, read-only FS, resource caps).

### Phase 4 — Structure, DSP model, hygiene
1. **Delete the server player:** `backend/player`, `backend/actors`, `/player/*` and `/ws/run`. Then drop the ALSA and X11 libs from the Dockerfile. (A7, D11)
2. **Dockerfile:** use cargo-chef or a deps-layer cache, a single pinned toolchain, matching builder and runtime Debian releases, `gcc libc6-dev` instead of `build-essential`, a pre-warm with the real compile profile, and a non-root user. (D11)
3. **One Cargo workspace:** include `transform-sdk` and `dtos`, delete the other manifest and lockfile, and run the SDK tests in CI. Remove the stale migration folders and one of the two frontend lockfiles. Pin the package manager via `packageManager`.
4. **ABI v2:** add `init(sample_rate, max_block)`, and consider an optional stereo-linked process call. Fix the per-block `Vec` allocations in the SDK. (F13, F18)
5. **Sample-accurate regions:** schedule region boundaries and graph switches inside the worklet using `currentFrame`, with fades. Move WASM compilation to the main thread and post the `WebAssembly.Module`, cached by `(transformId, version)`. (F11, F14)
6. **Monitoring:** add a `/metrics` endpoint (actix-web-prom) and real dashboards, or delete `monitoring/`. Remove the Grafana admin/admin default. (D12)
7. **Repo:** replace the copyrighted `samples/` with royalty-free clips (a history rewrite is optional), and purge `temp/` secrets from history if the repo goes public.

### Suggested order at a glance
Phase 0 steps 1–5 → Phase 1 steps 1–3 → Phase 2 steps 1–4 → the rest of Phase 1 → the rest of Phase 2 → Phase 3 → Phase 4.

