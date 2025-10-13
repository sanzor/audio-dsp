# syntax=docker/dockerfile:1

### 🏗 Stage 1: Build ###
FROM rust:1.82 AS builder
WORKDIR /app

# Copy the entire workspace (Cargo will find all members)
COPY . .

# Build only the API crate
RUN cargo build --release -p api

### 🚀 Stage 2: Runtime ###
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/api /usr/local/bin/app

EXPOSE 8080
CMD ["app"]
