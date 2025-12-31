#!/bin/bash
set -e

# --- 1. Configuration ---
DB_HOST="localhost"
DB_PORT="5432"
DB_USER="user"
DB_PASS="password"
DB_NAME="my_daw_db"

export DATABASE_URL="postgres://${DB_USER}:${DB_PASS}@${DB_HOST}:${DB_PORT}/${DB_NAME}"

# --- 2. Start PostgreSQL via Docker Compose ---
echo "Starting PostgreSQL container with Docker Compose..."
docker compose up -d postgres

# --- 3. Wait for the database to be ready ---
echo "Waiting for database to accept connections..."
ATTEMPTS=0
MAX_ATTEMPTS=15
until docker compose exec postgres pg_isready -U ${DB_USER} -d ${DB_NAME} -t 1 || [ $ATTEMPTS -eq $MAX_ATTEMPTS ]; do
  >&2 echo "Postgres is unavailable - sleeping (Attempt $ATTEMPTS/$MAX_ATTEMPTS)"
  sleep 2
  ATTEMPTS=$((ATTEMPTS + 1))
done

if [ $ATTEMPTS -eq $MAX_ATTEMPTS ]; then
    >&2 echo "Error: Postgres container failed to start or respond."
    exit 1
fi
>&2 echo "Postgres is up and running!"


# --- 4. Run Diesel Migrations (Containerized) ---
echo "Running Diesel migrations inside a temporary Docker container..."

# We update the image to a currently supported Debian release (bullseye).
docker run --rm \
  --network host \
  -e DATABASE_URL="${DATABASE_URL}" \
  -v "$(pwd)":/app \
  -w /app \
  rust:1.79-bullseye \
  bash -c "apt-get update -y && apt-get install -y libpq-dev && cargo install diesel_cli --no-default-features --features postgres && /root/.cargo/bin/diesel migration run"

echo "---------------------------------------------------------"
# ... rest of the script ...