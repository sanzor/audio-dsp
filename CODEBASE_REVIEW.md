# Codebase Review: Potential Issues

_Date: 2026-11-05 · Branch reviewed: `infra/ci-pipeline-and-fixes` (commit `83cf394`)_

The review found two critical security holes, one critical data-mixing bug, and a few audio problems you'd hear the moment you play a stereo track. The standard checks are clean: `cargo clippy`, `cargo test --workspace`, `tsc`, ESLint and `vitest` all pass with no warnings.

## Critical: fix before any real users

### 1. Saving a transform can expose your server's secrets

When a user saves a transform, the server compiles their Rust code, and the compiler runs with the API server's full environment ([build_job.rs:150](backend/api/src/transform_drafts/processor/build_job.rs#L150); nothing strips it). User code can therefore read environment variables at compile time.

This was tested: `compile_error!(concat!("leak:", env!("PROBE_SECRET")))` printed `error: leak:hunter2`, and that error text goes back to the user. In production, `JWT_SECRET`, `DATABASE_URL` and `GOOGLE_CLIENT_SECRET` are environment variables ([deployment.yaml](helm/rust-api/templates/deployment.yaml)). The same trick with `include_str!("/proc/self/environ")` or any readable file can bake secrets into a published binary. The code comments already describe compiled transforms as attacker-controlled.

- **Fix:** clear the compiler's environment and pass through only `PATH`, `CARGO_HOME`, `RUSTUP_HOME` and `CARGO_TARGET_DIR`.
- **Better fix:** run the compile in a separate sandbox (its own container, read-only filesystem, no network, no secrets).

### 2. Any signed-in user can read, edit or delete anyone's graphs and regions

The access check only confirms the caller belongs to the workspace named in the request. Every graph query then filters on `graph_id` alone ([graphs_data_provider_service.rs:116](backend/api/src/graphs/data_provider/graphs_data_provider_service.rs#L116)), and regions and region sets do the same ([regions_data_provider_service.rs:32](backend/api/src/regions/data_provider/regions_data_provider_service.rs#L32)). The IDs are sequential integers (`SERIAL`), so a user can just count through them.

Tracks are scoped correctly (`AND workspace_id = $2`), so copy that pattern. Separately, `PermissionsContextMiddleware::allow_all()` ([main.rs:460](backend/api/src/main.rs#L460)) is still a stub that grants every permission.

### 3. Two saves at the same time can swap each other's binaries

Every compile shares one build folder and reads back the same fixed file, `transform_build.wasm` ([build_job.rs:115](backend/api/src/transform_drafts/processor/build_job.rs#L115)). Cargo locks the folder while building, but the file is read after the lock is released. If two users save at once, user A's draft can be stored with user B's compiled binary, which breaks the rule that a transform's source must match its binary. CI runs these tests one at a time (`--test-threads=1`), so it can't catch this.

- **Fix:** give each job its own output location or copy the file out while still holding the lock.
- **Also:** add a limit on concurrent compiles. Each save is a synchronous release build with `lto = true` inside the HTTP request, and nothing caps how many run at once.

## High: audio correctness (the core of the product)

### 4. Stereo tracks will lose their right channel

The worklet reads `inputs[0][0]` and writes `outputs[0][0]` only, and neither place that creates the worklet node sets a channel count ([useWorkletSetup.ts:28](frontend/src/audio/hooks/useWorkletSetup.ts#L28), [creatorTransformPlayback.ts:75](frontend/src/components/creator/creatorTransformPlayback.ts#L75)). With the Web Audio defaults, the output follows the input, so a stereo source should come out with the right channel silent. This was read from the code, not confirmed by playing audio. The invariants require the channel layout to be explicit, and it isn't.

### 5. One crashing transform silences playback until the page is reloaded

The SDK builds with `panic = "abort"`, so any panic in a user's transform crashes the WASM call. Nothing in `process()` catches that, and nothing listens for the worklet's error event. The worklet stops for good with no error shown. Wrap the call, switch to bypass, and report the failure to the UI.

### 6. The audio thread creates new buffers on every audio block

[graph-worklet.js:127](frontend/src/audio/worklet/graph-worklet.js#L127) creates new arrays all the time: `SIL()`, `addAll`, and `Float32Array.from` for every WASM call. That's several allocations per node about every 2.9 ms at 44.1 kHz. The resulting garbage collection causes clicks and dropouts as graphs get bigger. Pre-allocate the buffers once per graph.

## Medium: structure and tooling

### 7. Two Cargo workspaces disagree

`/Cargo.toml` includes `transform-sdk`, but `backend/Cargo.toml`, which CI and Docker use, does not. As a result:

- `transform-sdk` (the crate every transform author builds against) and `dtos` don't build from their own folders.
- The SDK's 2 tests never run in CI.
- There are two lockfiles and two build folders (13 GB and 19 GB).

Pick one workspace, delete the other, and add `transform-sdk` to it (or `exclude` it on purpose).

### 8. Leftover server-side player

The `backend/player` crate (it uses the `cpal` audio library), the actors and the `/player/{play,pause,seek,stop}` endpoints are still wired in. The frontend never calls them, and the architecture says playback happens in the browser. They also force ALSA audio libraries into the production image. Remove them, or document why they exist.

### 9. Very little test coverage for an audio product

The 22.5k-line frontend has one unit test file (`GraphCompiler.test.ts`, 7 tests). `audiolib`, `domain` and `player` have zero tests, and the worklet runtime, stores and waveform/playback sync have none. The invariant that the DAG order matches the worklet's execution order is only partly covered.

### 10. Stale duplicate migrations

[migrations/0001_init/](database/audio_db/migrations/0001_init) and `0002_auth/` are folders whose `up.sql` files differ from the real `0001_init.up.sql` and `0002_auth.up.sql`. sqlx ignores folders, so they only confuse readers. Delete them.

### 11. CORS only allows localhost origins

See [main.rs:468](backend/api/src/main.rs#L468). This works in production only because nginx serves the API under `/api` on the same origin. Calling the API from any other origin will fail quietly.

## Low: repo hygiene

- **Copyrighted MP3s in git.** [samples/](samples) holds about 76 MB of commercial soundtracks (Taylor Swift, The Witcher 3, Black Hawk Down). That's a licensing risk if the repo is ever made public or shared, and the main reason the packed repo is about 126 MB. Replace them with royalty-free clips; removing them from history needs a history rewrite.
- **`code.html` is still committed** even though `.gitignore` lists it. Same for `scripts/__pycache__/` if it's tracked.
- **Dev secrets in git.** [dev.env](backend/api/dev.env) is committed and contains a `JWT_SECRET`. Fine for local development, but make sure production never shares that value.

## Suggested order

1. Items 1 and 2 (security).
2. Item 3 (data integrity).
3. Items 4 and 5, since they affect every real track.
4. Item 7, which is cheap and makes the SDK testable.

Each of these fits a small branch on its own.

## Not covered by this review

- Playwright end-to-end tests (they need the full stack running).
- An in-depth audit of the frontend Zustand stores or the billing code.
- A dependency audit (`scripts/ci/backend/audit.sh`).
