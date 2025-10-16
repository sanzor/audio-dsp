# syntax=docker/dockerfile:1

### 🏗 Stage 1: Build ###
FROM rust:slim AS builder
WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev libudev-dev libasound2-dev \
    libx11-dev libxcb1-dev libxrandr-dev libxi-dev \
 && rm -rf /var/lib/apt/lists/*

# --- NEW: Enforce your pinned Rust toolchain ---
# Copy the toolchain file if you have one
COPY rust-toolchain.toml ./
# Ensure the right toolchain is installed
RUN rustup show && rustup override set $(grep channel rust-toolchain.toml | cut -d '"' -f2)

# Copy manifests for caching
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/
COPY crates/common/Cargo.toml crates/common/
COPY crates/db/Cargo.toml crates/db/

# Pre-fetch dependencies
RUN cargo fetch --locked

# Copy the rest of the workspace
COPY . .

# Build API crate in release mode
RUN cargo build --release -p api

### 🚀 Stage 2: Runtime ###
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/api /usr/local/bin/app

EXPOSE 8080
ENTRYPOINT ["app"]
