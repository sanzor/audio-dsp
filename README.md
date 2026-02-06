Digital Audio Workstation (DAW) SaaS platform with Rust (Kameo actors, Tokio, Actix) supporting:
- track management
- streaming audio files
- applying digital transforms on tracks

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
