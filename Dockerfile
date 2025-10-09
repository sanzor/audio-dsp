# syntax=docker/dockerfile:1

### 🏗 Stage 1: Build the Rust binary ###
FROM rust:1.82 AS builder

# Create app directory inside container
WORKDIR /app

# Copy manifest first for caching dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build for release
RUN cargo build --release

### 🚀 Stage 2: Create the lightweight runtime image ###
FROM debian:bookworm-slim AS runtime

# Install minimal dependencies for SSL (if needed)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary from builder stage
COPY --from=builder /app/target/release/<your-binary-name> /usr/local/bin/app

# Expose the API port
EXPOSE 8080

# Command to start your API
CMD ["app"]
