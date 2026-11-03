#!/usr/bin/env bash
# Shared helpers for CI scripts. Source it; don't run it.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BACKEND_DIR="${ROOT_DIR}/backend"
FRONTEND_DIR="${ROOT_DIR}/frontend"

# Local image tags, shared by the build (CI) and publish (CD) steps.
BACKEND_IMAGE="${BACKEND_IMAGE:-audio-dsp-api:ci}"
FRONTEND_IMAGE="${FRONTEND_IMAGE:-audio-dsp-web:ci}"

log() { printf '\n==> %s\n' "$*"; }
die() { echo "error: $*" >&2; exit 1; }
