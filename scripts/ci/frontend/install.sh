#!/usr/bin/env bash
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
cd "${FRONTEND_DIR}"
log "pnpm install --frozen-lockfile"
pnpm install --frozen-lockfile
