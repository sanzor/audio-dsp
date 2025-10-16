# syntax=docker/dockerfile:1

### 🏗 Stage 1: Build ###
FROM rust:1.90.0 AS builder

WORKDIR /app

# ✅ Install ALSA + other common build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libasound2-dev \
    libssl-dev \
    libudev-dev \
    libx11-dev \
    libxcb1-dev \
    libxrandr-dev \
    libxi-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the full workspace
COPY . .

# Build only the API crate
RUN cargo build --release -p api

### 🚀 Stage 2: Runtime ###
FROM debian:bookworm-slim AS runtime

# Minimal runtime deps
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/api /usr/local/bin/app

EXPOSE 8080
CMD ["app"]
