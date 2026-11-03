#!/usr/bin/env bash
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
cd "${FRONTEND_DIR}"
# tsc -b (typecheck) + vite build
log "pnpm build"
pnpm build
