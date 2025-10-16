### 🏗 Stage 1: Build ###
FROM --platform=$BUILDPLATFORM rust:slim AS builder
WORKDIR /app

# Install build system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libasound2-dev \
    libssl-dev \
    libudev-dev \
    libx11-dev \
    libxcb1-dev \
    libxrandr-dev \
    libxi-dev \
    build-essential \
 && rm -rf /var/lib/apt/lists/*

ENV PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig
# Verify that ALSA .pc file exists
RUN find /usr -name alsa.pc || (echo "alsa.pc not found" && exit 1)

# Copy and enforce pinned toolchain
COPY rust-toolchain.toml ./
RUN RUST_VERSION=$(grep channel rust-toolchain.toml | cut -d '"' -f2) && \
    rustup toolchain install "$RUST_VERSION" && \
    rustup default "$RUST_VERSION" && \
    rustc --version

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
