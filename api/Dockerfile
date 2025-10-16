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
    ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/api /usr/local/bin/app

EXPOSE 8080
ENTRYPOINT ["app"]
