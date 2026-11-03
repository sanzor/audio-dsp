#!/usr/bin/env bash
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
cd "${BACKEND_DIR}"
log "cargo clippy"
cargo clippy --workspace --all-targets -- -D warnings
