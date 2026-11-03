#!/usr/bin/env bash
# #[ignore]d tests (slow, need the wasm32 target) run in integration-tests.sh.
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
cd "${BACKEND_DIR}"
log "cargo test (unit)"
cargo test --workspace --all-targets
