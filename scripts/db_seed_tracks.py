#!/usr/bin/env python3
import io
import json
import math
import struct
import subprocess
import wave
from pathlib import Path


ROOT_DIR = Path(__file__).resolve().parent.parent
DB_PSQL = ROOT_DIR / "scripts" / "db_psql.sh"


TRACK_SEEDS = [
    # Default Project (admin)
    {
        "project_name": "Default Project",
        "name": "Kick Drum",
        "duration_seconds": 0.40,
        "frequency_hz": 60.0,
    },
    {
        "project_name": "Default Project",
        "name": "Snare Hit",
        "duration_seconds": 0.30,
        "frequency_hz": 200.0,
    },
    {
        "project_name": "Default Project",
        "name": "Hi-Hat",
        "duration_seconds": 0.20,
        "frequency_hz": 8000.0,
    },
    # Canonical Audio Lab (test user)
    {
        "project_name": "Canonical Audio Lab",
        "name": "A440 Reference",
        "duration_seconds": 0.35,
        "frequency_hz": 440.0,
        "seed_region_hierarchy": True,
    },
    {
        "project_name": "Canonical Audio Lab",
        "name": "A880 Reference",
        "duration_seconds": 0.35,
        "frequency_hz": 880.0,
    },
    # Mix Review Sandbox (test user)
    {
        "project_name": "Mix Review Sandbox",
        "name": "C4 Pad",
        "duration_seconds": 0.50,
        "frequency_hz": 261.63,
    },
    # Regression Pack (test user)
    {
        "project_name": "Regression Pack",
        "name": "Low E Pulse",
        "duration_seconds": 0.45,
        "frequency_hz": 82.41,
    },
]


def run_psql(*args: str, stdin: str | None = None) -> str:
    proc = subprocess.run(
        ["bash", str(DB_PSQL), *args],
        input=stdin,
        text=True,
        capture_output=True,
        check=False,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or proc.stdout.strip() or "psql command failed")
    return proc.stdout


def scalar(sql: str) -> str:
    return run_psql("-tAc", sql).strip()


def table_exists(table_name: str) -> bool:
    result = scalar(f"SELECT to_regclass('public.{table_name}') IS NOT NULL;")
    return result == "t"


def column_exists(table_name: str, column_name: str) -> bool:
    result = scalar(
        f"""
SELECT EXISTS (
    SELECT 1
    FROM information_schema.columns
    WHERE table_schema = 'public'
      AND table_name = '{table_name}'
      AND column_name = '{column_name}'
);
"""
    )
    return result == "t"


def generate_wav_bytes(frequency_hz: float, duration_seconds: float) -> bytes:
    sample_rate = 22050
    sample_count = int(sample_rate * duration_seconds)
    pcm = bytearray()

    for index in range(sample_count):
        t = index / sample_rate
        sample = int(0.35 * 32767 * math.sin(2 * math.pi * frequency_hz * t))
        pcm.extend(struct.pack("<h", sample))

    buffer = io.BytesIO()
    with wave.open(buffer, "wb") as wav_file:
        wav_file.setnchannels(1)
        wav_file.setsampwidth(2)
        wav_file.setframerate(sample_rate)
        wav_file.writeframes(bytes(pcm))

    return buffer.getvalue()


def ensure_seed_users_exist() -> None:
    for email in ("admin@gmail.com", "test@gmail.com"):
        user_id = scalar(f"SELECT user_id::text FROM users WHERE email = '{email}' LIMIT 1;")
        if not user_id:
            raise RuntimeError(f"Seed user '{email}' was not found. Run users.sql first.")


def project_id_for(name: str) -> int:
    project_id = scalar(
        f"SELECT project_id::text FROM projects WHERE name = '{name}' ORDER BY project_id LIMIT 1;"
    )
    if not project_id:
        raise RuntimeError(f"Seed project '{name}' was not found. Run projects.sql first.")
    return int(project_id)


def has_track_storage_table() -> bool:
    return table_exists("track_storage")


def default_graph_state_json() -> str:
    nodes: list[dict[str, object]] = []
    edges: list[dict[str, object]] = []

    if table_exists("transforms") and table_exists("transform_ports"):
        gain_id = scalar(
            """
SELECT transform_id::text
FROM transforms
WHERE name = 'Gain'
ORDER BY transform_id
LIMIT 1;
"""
        )
        low_pass_id = scalar(
            """
SELECT transform_id::text
FROM transforms
WHERE name = 'Low-Pass Filter'
ORDER BY transform_id
LIMIT 1;
"""
        )
        gain_out_port_id = scalar(
            """
SELECT tp.port_id::text
FROM transform_ports tp
JOIN transforms t ON t.transform_id = tp.transform_id
WHERE t.name = 'Gain'
  AND tp.direction = 'output'
ORDER BY tp.port_order
LIMIT 1;
"""
        )
        low_pass_in_port_id = scalar(
            """
SELECT tp.port_id::text
FROM transform_ports tp
JOIN transforms t ON t.transform_id = tp.transform_id
WHERE t.name = 'Low-Pass Filter'
  AND tp.direction = 'input'
ORDER BY tp.port_order
LIMIT 1;
"""
        )

        if gain_id and low_pass_id:
            nodes = [
                {
                    "id": 1,
                    "transformId": int(gain_id),
                    "position": {"x": 180, "y": 140},
                    "params": {"gain": 1.0},
                },
                {
                    "id": 2,
                    "transformId": int(low_pass_id),
                    "position": {"x": 440, "y": 140},
                    "params": {"cutoffHz": 1200.0},
                },
            ]

            if gain_out_port_id and low_pass_in_port_id:
                edges = [
                    {
                        "id": 1,
                        "fromNodeId": 1,
                        "toNodeId": 2,
                        "fromPortId": int(gain_out_port_id),
                        "toPortId": int(low_pass_in_port_id),
                    }
                ]

    return json.dumps(
        {
            "schemaVersion": 1,
            "nodes": nodes,
            "edges": edges,
        },
        separators=(",", ":"),
    )


def ensure_default_graph_for_region(region_id: int) -> None:
    if not table_exists("graphs"):
        return

    if column_exists("graphs", "graph_state"):
        graph_state = default_graph_state_json()
        graph_sql = f"""
INSERT INTO graphs (region_id, name, graph_state, version, updated_at)
VALUES (
  {region_id},
  'Seed Graph',
  $seed_graph${graph_state}$seed_graph$::jsonb,
  1,
  now()
)
ON CONFLICT (region_id) DO UPDATE
SET
  name = EXCLUDED.name,
  graph_state = EXCLUDED.graph_state,
  version = 1,
  updated_at = now();
"""
    else:
        graph_sql = f"""
INSERT INTO graphs (region_id, name)
VALUES ({region_id}, 'Seed Graph')
ON CONFLICT (region_id) DO UPDATE
SET name = EXCLUDED.name;
"""

    run_psql("-v", "ON_ERROR_STOP=1", "-c", graph_sql)


def ensure_region_hierarchy_for_track(track_id: int) -> None:
    region_set_sql = f"""
INSERT INTO region_sets (track_id, name, track_length_seconds)
SELECT
  t.track_id,
  'Seed Region Set',
  t.length_seconds
FROM tracks t
WHERE t.track_id = {track_id}
  AND NOT EXISTS (
    SELECT 1
    FROM region_sets rs
    WHERE rs.track_id = t.track_id
      AND rs.name = 'Seed Region Set'
  );
"""
    run_psql("-v", "ON_ERROR_STOP=1", "-c", region_set_sql)

    region_set_id = scalar(
        f"""
SELECT region_set_id::text
FROM region_sets
WHERE track_id = {track_id}
  AND name = 'Seed Region Set'
ORDER BY region_set_id
LIMIT 1;
"""
    )
    if not region_set_id:
        raise RuntimeError(f"Failed to resolve seeded region set for track_id={track_id}.")

    region_sql = f"""
INSERT INTO regions (region_set_id, name, start_time_seconds, end_time_seconds)
SELECT
  {region_set_id},
  'Seed Region',
  0.00,
  LEAST(
    COALESCE((SELECT track_length_seconds FROM region_sets WHERE region_set_id = {region_set_id}), 0.25),
    0.25
  )
WHERE NOT EXISTS (
  SELECT 1
  FROM regions r
  WHERE r.region_set_id = {region_set_id}
    AND r.name = 'Seed Region'
);
"""
    run_psql("-v", "ON_ERROR_STOP=1", "-c", region_sql)

    region_id = scalar(
        f"""
SELECT region_id::text
FROM regions
WHERE region_set_id = {region_set_id}
  AND name = 'Seed Region'
ORDER BY region_id
LIMIT 1;
"""
    )
    if region_id:
        ensure_default_graph_for_region(int(region_id))


def upsert_track(seed: dict[str, str | float], use_track_storage: bool) -> None:
    project_id = project_id_for(str(seed["project_name"]))
    wav_hex = generate_wav_bytes(
        float(seed["frequency_hz"]),
        float(seed["duration_seconds"]),
    ).hex()

    if use_track_storage:
        track_insert_sql = f"""
INSERT INTO tracks (name, extension, length_seconds, project_id)
SELECT
  '{seed["name"]}',
  'wav',
  {float(seed["duration_seconds"]):.2f},
  {project_id}
WHERE NOT EXISTS (
  SELECT 1 FROM tracks WHERE name = '{seed["name"]}' AND project_id = {project_id}
);
"""
        run_psql("-v", "ON_ERROR_STOP=1", "-c", track_insert_sql)
        track_id = scalar(
            f"SELECT track_id::text FROM tracks WHERE name = '{seed['name']}' AND project_id = {project_id} LIMIT 1;"
        )
        if not track_id:
            raise RuntimeError(
                f"Failed to resolve track_id for seed '{seed['name']}' in project '{seed['project_name']}'."
            )

        storage_sql = f"""
INSERT INTO track_storage (track_id, data)
VALUES ({track_id}, decode('{wav_hex}', 'hex'))
ON CONFLICT (track_id) DO UPDATE SET data = EXCLUDED.data;
"""
        run_psql("-v", "ON_ERROR_STOP=1", "-c", storage_sql)
    else:
        # Legacy schema: canonical_audio lives directly on tracks
        run_psql("-v", "ON_ERROR_STOP=1", "-c", f"""
INSERT INTO tracks (name, extension, length_seconds, canonical_audio, project_id)
SELECT
  '{seed["name"]}',
  'wav',
  {float(seed["duration_seconds"]):.2f},
  decode('{wav_hex}', 'hex'),
  {project_id}
WHERE NOT EXISTS (
  SELECT 1 FROM tracks WHERE name = '{seed["name"]}' AND project_id = {project_id}
);
""")
        track_id = scalar(
            f"SELECT track_id::text FROM tracks WHERE name = '{seed['name']}' AND project_id = {project_id} LIMIT 1;"
        )
        if not track_id:
            raise RuntimeError(
                f"Failed to resolve track_id for seed '{seed['name']}' in project '{seed['project_name']}'."
            )

    if seed.get("seed_region_hierarchy"):
        ensure_region_hierarchy_for_track(int(track_id))


def main() -> None:
    ensure_seed_users_exist()
    use_track_storage = has_track_storage_table()
    for track_seed in TRACK_SEEDS:
        upsert_track(track_seed, use_track_storage)
    print(f"Seeded {len(TRACK_SEEDS)} canonical tracks across all seed projects.")


if __name__ == "__main__":
    main()
