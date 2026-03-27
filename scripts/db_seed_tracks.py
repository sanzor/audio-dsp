#!/usr/bin/env python3
import io
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
    result = scalar("SELECT to_regclass('public.track_storage') IS NOT NULL;")
    return result == "t"


def upsert_track(seed: dict[str, str | float], use_track_storage: bool) -> None:
    project_id = project_id_for(str(seed["project_name"]))
    wav_hex = generate_wav_bytes(
        float(seed["frequency_hz"]),
        float(seed["duration_seconds"]),
    ).hex()

    if use_track_storage:
        track_sql = f"""
INSERT INTO tracks (name, extension, length_seconds, project_id)
SELECT
  '{seed["name"]}',
  'wav',
  {float(seed["duration_seconds"]):.2f},
  {project_id}
WHERE NOT EXISTS (
  SELECT 1 FROM tracks WHERE name = '{seed["name"]}' AND project_id = {project_id}
)
RETURNING track_id;
"""
        result = run_psql("-tAc", track_sql)
        track_id = result.strip()
        if not track_id:
            return  # already existed, skip storage insert

        storage_sql = f"""
INSERT INTO track_storage (track_id, data)
VALUES ({track_id}, decode('{wav_hex}', 'hex'))
ON CONFLICT (track_id) DO NOTHING;
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


def main() -> None:
    ensure_seed_users_exist()
    use_track_storage = has_track_storage_table()
    for track_seed in TRACK_SEEDS:
        upsert_track(track_seed, use_track_storage)
    print(f"Seeded {len(TRACK_SEEDS)} canonical tracks across all seed projects.")


if __name__ == "__main__":
    main()
