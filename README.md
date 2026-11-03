Digital Audio Workstation (DAW) SaaS platform with Rust (Kameo actors, Tokio, Actix) and a React editor supporting:
- track management
- streaming audio files
- applying digital transforms on tracks

Architecture at a glance:

- transform creators author transform code in the creator surface
- the backend compiles transforms through a poll-based ticket workflow
- compiled transforms are saved/published by the platform
- the editor may fetch and cache published transform binaries for local runtime use
- artists use the editor surface to compose region-level transform graphs
- published transform chains execute on the frontend through an audio worklet pipeline

Currently supported operations:
- Load audio track
- Upload processed audio to disk
- Copy audio file
- Gain
- Normalize
- Low Pass Filter - applies low pass filter over a audio file
- High Pass Filter - applies high pass filter over a audio file
- Stream audio tracks with supporting controls (play/pause/stop/seek)

# First-time setup
./backend/scripts/setup-dev.sh

# Local dev (Docker Compose)
make up

# URLs
- Frontend: `http://localhost:3000`
- Backend: `http://localhost:3080`
- API docs (Swagger UI): `http://localhost:3080/docs`
- Prometheus: `http://localhost:9090` (or `PROMETHEUS_PORT`)
- Grafana: `http://localhost:3001` (or `GRAFANA_PORT`, default login `admin/admin`)

# Local dev (manual)
- Backend: `cargo run -p api` (from `./backend`)
- Frontend: `pnpm install && pnpm dev` (from `./frontend`)

# Development container

Install Docker and the VS Code Dev Containers extension, then choose **Dev Containers: Reopen in Container**. The first build installs Rust 1.90, the WASM target, Node, pnpm, SQLx CLI, and native audio dependencies; it may take a while. A separate PostgreSQL 16 service starts with the container and keeps its data in a Docker volume.

Inside the container, initialize the database and start the application in separate terminals:

```sh
make -e migrate
make -e db-seed
cargo run -p api
```

```sh
cd frontend && pnpm dev
```

Open `http://localhost:3000`; ports 3000 and 3080 are forwarded. The `-e` flag lets the container's PostgreSQL settings take precedence over the root `.env` file. The container database is separate from the host Compose stack; `make up` and the Docker-based stack commands are intended to run on the host. To use Playwright in the container, install its browser and system dependencies separately (`npx playwright install --with-deps chromium` from `frontend`).
