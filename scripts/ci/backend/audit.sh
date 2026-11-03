#!/usr/bin/env bash
# Ignored advisories (with reasons) live in backend/.cargo/audit.toml.
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
command -v cargo-audit >/dev/null || die "cargo-audit not installed (cargo install cargo-audit)"
cd "${BACKEND_DIR}"
log "cargo audit"
cargo audit "$@"
