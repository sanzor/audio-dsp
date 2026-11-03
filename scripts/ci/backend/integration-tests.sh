#!/usr/bin/env bash
# Real Rust -> wasm32 compile pipeline (backend/api/tests/transform_compile_pipeline.rs).
source "$(dirname "${BASH_SOURCE[0]}")/../lib.sh"
cd "${BACKEND_DIR}"
rustup target list --installed | grep -qx wasm32-unknown-unknown \
  || die "missing target: rustup target add wasm32-unknown-unknown"
log "cargo test (integration, --ignored)"
# Serial on purpose: every compile writes <shared target dir>/.../transform_build.wasm,
# so concurrent builds read each other's output (see build_job.rs). Remove once fixed.
cargo test -p api --test transform_compile_pipeline -- --ignored --test-threads=1
