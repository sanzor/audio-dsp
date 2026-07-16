# syntax=docker/dockerfile:1
FROM rust:slim AS builder
WORKDIR /app

# Install system dependencies required for compilation
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libasound2-dev alsa-utils libssl-dev libudev-dev \
    libx11-dev libxcb1-dev libxrandr-dev libxi-dev build-essential \
 && rm -rf /var/lib/apt/lists/*

# Ensure pkg-config can find system .pc files
ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig
ENV PKG_CONFIG_LIBDIR=/usr/lib/x86_64-linux-gnu/pkgconfig

# (Optional sanity check — you can remove this later)
RUN find /usr -name alsa.pc && pkg-config --exists alsa && echo "✅ ALSA found" || (echo "❌ ALSA missing" && exit 1)

# Enforce Rust toolchain from your pinned file
COPY rust-toolchain.toml ./
RUN RUST_VERSION=$(grep channel rust-toolchain.toml | cut -d '"' -f2) && \
    rustup toolchain install "$RUST_VERSION" && \
    rustup default "$RUST_VERSION" && \
    rustc --version

# Copy the full workspace (no caching layers)
COPY . .

# Build your main crate
RUN cargo build --release -p api

# --- Runtime stage ---
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates build-essential && rm -rf /var/lib/apt/lists/*

# The API process compiles creator-submitted Rust source to wasm32-unknown-unknown
# in-process (see backend/api/src/worker/processor/build_job.rs). This reuses the
# builder stage's already-installed pinned toolchain instead of re-downloading it.
# v1 tradeoff: compiles run in the same container as the API (no separate
# resource-capped/network-isolated compiler service yet) — see agents/architecture.md.
COPY --from=builder /usr/local/rustup /usr/local/rustup
COPY --from=builder /usr/local/cargo /usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup CARGO_HOME=/usr/local/cargo PATH=/usr/local/cargo/bin:$PATH
RUN rustup target add wasm32-unknown-unknown

# Ship the transform SDK crate and pre-build it once so the first real compile
# job already hits a warm shared target dir. Builder's `cargo build -p api`
# above already pulled serde/serde_json/once_cell into CARGO_HOME's registry
# cache (transform-sdk's deps mirror api's), so this needs no network.
COPY --from=builder /app/transform-sdk /opt/transform-sdk
ENV TRANSFORM_SDK_PATH=/opt/transform-sdk
ENV TRANSFORM_CARGO_TARGET_DIR=/opt/transform-sdk/target
RUN cd /opt/transform-sdk && \
    CARGO_TARGET_DIR=/opt/transform-sdk/target cargo build --release --target wasm32-unknown-unknown --offline
ENV TRANSFORM_BUILD_WORKDIR=/tmp/audio-dsp-builds

WORKDIR /app
COPY --from=builder /app/target/release/api /usr/local/bin/app

EXPOSE 3080
ENTRYPOINT ["app"]
