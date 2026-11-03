#!/usr/bin/env bash
# Install the toolchain pinned in backend/rust-toolchain.toml, plus optional components/targets.
# Usage: backend-setup-toolchain.sh [--components "clippy rustfmt"] [--targets "wasm32-unknown-unknown"] [--system-deps]
source "$(dirname "${BASH_SOURCE[0]}")/_lib.sh"

components="" targets="" system_deps=false
while [[ $# -gt 0 ]]; do
  case "$1" in
    --components) components="$2"; shift 2 ;;
    --targets) targets="$2"; shift 2 ;;
    --system-deps) system_deps=true; shift ;;
    *) echo "unknown arg: $1" >&2; exit 2 ;;
  esac
done

cd "${BACKEND_DIR}"
log "Installing pinned toolchain"
rustup toolchain install --profile minimal
for c in ${components}; do rustup component add "$c"; done
for t in ${targets}; do rustup target add "$t"; done
rustc --version

if [[ "${system_deps}" == true ]]; then
  log "Installing system dependencies"
  sudo apt-get update
  sudo apt-get install -y --no-install-recommends pkg-config libasound2-dev libudev-dev libssl-dev
fi
