#!/usr/bin/env python3
"""Seed dev data for the `sources` / `source_storage` tables.

Mirrors db_seed_tracks.py's shape: a flat list of seed dicts, one row per
signal source, upserted by (name, project_id). Reuses db_seed_tracks.py's
generic DB-seed helpers and its sine-wave WAV generator rather than
duplicating them, since importing it has no side effects (TRACK_SEEDS is
just a module-level list; nothing runs until main() is called under
`if __name__ == "__main__"`).
"""
from db_seed_tracks import (
    generate_wav_bytes,
    project_id_for,
    run_psql,
    scalar,
    sql_literal,
)


SOURCE_SEEDS = [
    # Default Project (admin)
    {
        "project_name": "Default Project",
        "name": "Warm-Up Tone",
        "duration_seconds": 0.50,
        "frequency_hz": 220.0,
    },
    {
        "project_name": "Default Project",
        "name": "Calibration Sweep",
        "duration_seconds": 0.30,
        "frequency_hz": 1000.0,
    },
    # Canonical Audio Lab (test user)
    {
        "project_name": "Canonical Audio Lab",
        "name": "A440 Source",
        "duration_seconds": 0.40,
        "frequency_hz": 440.0,
    },
    {
        "project_name": "Canonical Audio Lab",
        "name": "A220 Source",
        "duration_seconds": 0.40,
        "frequency_hz": 220.0,
    },
    # Mix Review Sandbox (test user)
    {
        "project_name": "Mix Review Sandbox",
        "name": "Mix Reference Tone",
        "duration_seconds": 0.50,
        "frequency_hz": 300.0,
    },
    {
        "project_name": "Mix Review Sandbox",
        "name": "Mix Check Tone",
        "duration_seconds": 0.50,
        "frequency_hz": 600.0,
    },
    # Regression Pack (test user)
    {
        "project_name": "Regression Pack",
        "name": "Regression Tone Low",
        "duration_seconds": 0.40,
        "frequency_hz": 100.0,
    },
    {
        "project_name": "Regression Pack",
        "name": "Regression Tone High",
        "duration_seconds": 0.40,
        "frequency_hz": 5000.0,
    },
]


def upsert_source(seed: dict[str, object]) -> None:
    project_id = project_id_for(str(seed["project_name"]))
    duration_seconds = float(seed["duration_seconds"])
    audio_bytes = generate_wav_bytes(float(seed["frequency_hz"]), duration_seconds)
    audio_hex = audio_bytes.hex()
    source_name_sql = sql_literal(str(seed["name"]))
    extension_sql = sql_literal("wav")

    source_insert_sql = f"""
INSERT INTO sources (name, extension, length_seconds, project_id)
SELECT
  {source_name_sql},
  {extension_sql},
  {duration_seconds:.2f},
  {project_id}
WHERE NOT EXISTS (
  SELECT 1 FROM sources WHERE name = {source_name_sql} AND project_id = {project_id}
);
"""
    run_psql("-v", "ON_ERROR_STOP=1", "-c", source_insert_sql)
    source_id = scalar(
        f"SELECT source_id::text FROM sources WHERE name = {source_name_sql} AND project_id = {project_id} LIMIT 1;"
    )
    if not source_id:
        raise RuntimeError(
            f"Failed to resolve source_id for seed '{seed['name']}' in project '{seed['project_name']}'."
        )

    storage_sql = f"""
INSERT INTO source_storage (source_id, storage_type, data)
VALUES ({source_id}, 'inline', decode('{audio_hex}', 'hex'))
ON CONFLICT (source_id) DO UPDATE SET data = EXCLUDED.data;
"""
    run_psql("-v", "ON_ERROR_STOP=1", stdin=storage_sql)


def main() -> None:
    for source_seed in SOURCE_SEEDS:
        upsert_source(source_seed)
    print(f"Seeded {len(SOURCE_SEEDS)} signal sources across all seed projects.")


if __name__ == "__main__":
    main()
