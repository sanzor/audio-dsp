# syntax=docker/dockerfile:1

### 🏗 Stage 1: Build ###
FROM rust:1.85.1 AS builder
WORKDIR /app

# Install system dependencies for native crates
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev libudev-dev libasound2-dev \
    libx11-dev libxcb1-dev libxrandr-dev libxi-dev \
 && rm -rf /var/lib/apt/lists/*

# Copy manifests first for caching
COPY Cargo.toml Cargo.lock ./
COPY api/Cargo.toml api/
COPY crates/common/Cargo.toml crates/common/
COPY crates/db/Cargo.toml crates/db/

# Pre-fetch dependencies (cached unless Cargo.toml changes)
RUN cargo fetch --locked

# Now copy the full source
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
