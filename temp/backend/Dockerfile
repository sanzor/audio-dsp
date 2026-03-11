# syntax=docker/dockerfile:1
FROM rust:1.90-slim AS builder
WORKDIR /app

# 1. Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev build-essential mold \
    curl unzip \
 && rm -rf /var/lib/apt/lists/*

# 2. Copy workspace
COPY . .

# 4. Build ONLY the backend binary
RUN cargo build --release -p backend

# --- Runtime stage ---
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates procps && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 1. Copy the binary
COPY --from=builder /app/target/release/backend /usr/local/bin/app

# 2. Copy config
RUN mkdir -p ./config
COPY --from=builder /app/portal/backend/config/ ./config/

ENV IN_DOCKER=true
ENV APP_ENV=dev
ENV APP_CONFIG_PATH=.

EXPOSE 3091
ENTRYPOINT ["app"]
