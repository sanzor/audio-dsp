# audio_db

This folder contains database assets for the Audio DSP backend.

## Migrations

Migrations live in `database/audio_db/migrations/`.

- `0001_init/` creates tables for domain models:
  - tracks
  - region_sets
  - regions
  - graphs
  - graph_nodes
  - graph_edges

### Apply manually (psql)

```bash
psql "$DATABASE_URL" -f database/audio_db/migrations/0001_init/up.sql
```

### Roll back manually (psql)

```bash
psql "$DATABASE_URL" -f database/audio_db/migrations/0001_init/down.sql
```

