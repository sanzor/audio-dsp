# syntax=docker/dockerfile:1

### 🏗 Stage 1: Build the Rust binary ###
FROM rust:1.82 AS builder

# Set the working directory inside the container
WORKDIR /app

# Copy the workspace manifest files first (for dependency caching)
COPY Cargo.toml Cargo.lock ./

# Copy the API crate manifest and source
COPY api/Cargo.toml ./api/Cargo.toml
COPY api/src ./api/src

# If you have other crates in the workspace (like `shared/`), copy them too:
# COPY shared ./shared

# Build the API crate in release mode
RUN cargo build --release -p api

### 🚀 Stage 2: Create the lightweight runtime image ###
FROM debian:bookworm-slim AS runtime

# Install minimal runtime dependencies (e.g. SSL certs)
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy compiled binary from the builder stage
COPY --from=builder /app/target/release/api /usr/local/bin/app

# Expose the API port (adjust if your app uses a different one)
EXPOSE 8080

# Run the binary
CMD ["app"]