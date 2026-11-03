#!/usr/bin/env bash
# Usage: frontend.sh <install|lint|test|build>
source "$(dirname "${BASH_SOURCE[0]}")/_lib.sh"
cd "${FRONTEND_DIR}"
case "${1:-}" in
  install) log "pnpm install"; pnpm install --frozen-lockfile ;;
  lint)    log "eslint";       pnpm lint ;;
  test)    log "vitest";       pnpm test ;;
  build)   log "tsc + vite";   pnpm build ;;
  *) echo "usage: $0 <install|lint|test|build>" >&2; exit 2 ;;
esac
