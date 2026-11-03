#!/usr/bin/env bash
# Shared helpers for CI scripts. Source it; don't run it.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
BACKEND_DIR="${ROOT_DIR}/backend"
FRONTEND_DIR="${ROOT_DIR}/frontend"

# Local image tag shared by build-image and publish-image.
CI_IMAGE="${CI_IMAGE:-audio-dsp-api:ci}"

log() { printf '\n==> %s\n' "$*"; }
