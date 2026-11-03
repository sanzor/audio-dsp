#!/usr/bin/env bash
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
cd "${FRONTEND_DIR}"
log "pnpm test"
pnpm test
