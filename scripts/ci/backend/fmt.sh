#!/usr/bin/env bash
# Check formatting. Pass --fix to rewrite files instead.
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
cd "${BACKEND_DIR}"
if [[ "${1:-}" == "--fix" ]]; then
  log "cargo fmt"; cargo fmt --all
else
  log "cargo fmt --check"; cargo fmt --all -- --check
fi
