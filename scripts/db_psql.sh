#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

PROJECT_NAME="${PROJECT_NAME:-audio-dsp}"
PG_USER="${PG_USER:-postgres}"
PG_PASS="${PG_PASS:-postgres}"
PG_HOST="${PG_HOST:-127.0.0.1}"
PG_PORT="${PG_PORT:-5433}"
PG_DB="${PG_DB:-audio_dsp}"

if command -v psql >/dev/null 2>&1; then
  export PGPASSWORD="${PG_PASS}"
  exec psql -h "${PG_HOST}" -p "${PG_PORT}" -U "${PG_USER}" -d "${PG_DB}" "$@"
fi

compose_cmd=(docker compose --project-name "${PROJECT_NAME}")
if [[ -f "${ROOT_DIR}/.env" ]]; then
  compose_cmd+=(--env-file "${ROOT_DIR}/.env")
fi
compose_cmd+=(-f "${ROOT_DIR}/yml/postgres.yml")

if "${compose_cmd[@]}" ps --services --status running 2>/dev/null | grep -qx postgres; then
  exec "${compose_cmd[@]}" exec -T postgres psql -U "${PG_USER}" -d "${PG_DB}" "$@"
fi

echo "psql is not installed and the repo Postgres container is not running." >&2
echo "Start it with 'make up-db' or install psql locally." >&2
exit 1
