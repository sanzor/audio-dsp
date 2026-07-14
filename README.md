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
