#!/usr/bin/env bash
source "$(dirname "${BASH_SOURCE[0]}")/_lib.sh"
command -v cargo-audit >/dev/null || { echo "cargo-audit not installed (cargo install cargo-audit)" >&2; exit 1; }
cd "${BACKEND_DIR}"
log "cargo audit"
cargo audit "$@"
