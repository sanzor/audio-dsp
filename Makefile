-include .env
export

# ─── Variables ────────────────────────────────────────────────────────────────

PROJECT_NAME  ?= audio-dsp
COMPOSE_DIR   := ./yml
ENV_PATH      ?= ./.env

ENV_FILE_FLAG := $(if $(wildcard $(ENV_PATH)),--env-file $(ENV_PATH),)
COMPOSE       := docker compose --project-name $(PROJECT_NAME) $(ENV_FILE_FLAG)

# Postgres connection (mirrors postgres.yml defaults)
PG_USER   ?= postgres
PG_PASS   ?= postgres
PG_HOST   ?= 127.0.0.1
PG_PORT   ?= 5433
PG_DB     ?= audio_dsp
DATABASE_URL ?= postgres://$(PG_USER):$(PG_PASS)@$(PG_HOST):$(PG_PORT)/$(PG_DB)

MIGRATIONS_DIR := ./database/audio_db/migrations
SEEDS_DIR      := ./database/seeds

# ─── Compose File Shortcuts ───────────────────────────────────────────────────

DB  := -f $(COMPOSE_DIR)/postgres.yml
BE  := -f $(COMPOSE_DIR)/backend.yml
FE  := -f $(COMPOSE_DIR)/frontend.yml
MON := -f $(COMPOSE_DIR)/monitoring.yml

ALL := $(DB) $(BE) $(FE) $(MON)

# ─── Phony Targets ───────────────────────────────────────────────────────────

.PHONY: help \
        up up-s up-db up-backend up-frontend up-mon \
        down down-db down-backend down-frontend down-mon \
        restart restart-backend restart-frontend \
        build build-backend build-frontend \
        logs logs-db logs-backend logs-frontend logs-mon \
        sh-db sh-backend sh-frontend \
        status clean pull \
        migrate migrate-down migrate-info migrate-redo \
        db-cli db-seed db-reinit db-reset \
        check-backend \
        dev-backend dev-frontend \
        lint lint-backend lint-frontend \
        test test-backend test-frontend

# ─── Help ────────────────────────────────────────────────────────────────────

help:
	@echo ""
	@echo "Usage: make <target>"
	@echo ""
	@echo "Stack"
	@echo "  up               Build + start everything (db, backend, frontend, monitoring)"
	@echo "  up-s             Start everything without rebuilding"
	@echo "  down             Stop + remove all containers"
	@echo "  restart          down then up"
	@echo "  status           Show running containers"
	@echo "  clean            down -v (removes volumes too)"
	@echo "  pull             Pull latest base images"
	@echo ""
	@echo "Individual services"
	@echo "  up-db            Start Postgres (waits for healthy)"
	@echo "  up-backend       Build + start backend"
	@echo "  up-frontend      Build + start frontend"
	@echo "  up-mon           Start Prometheus + Grafana"
	@echo "  down-backend     Stop + remove backend container"
	@echo "  down-frontend    Stop + remove frontend container"
	@echo "  restart-backend  Rebuild + restart backend only"
	@echo "  restart-frontend Rebuild + restart frontend only"
	@echo ""
	@echo "Database"
	@echo "  migrate          Run all pending sqlx migrations"
	@echo "  migrate-down     Revert last migration"
	@echo "  migrate-info     List migration status"
	@echo "  migrate-redo     Revert then re-apply last migration"
	@echo "  db-cli           Open psql shell inside the running container"
	@echo "  db-seed          Seed dev data (skips if users already exist)"
	@echo "  db-reinit        Drop volumes, start fresh, run migrations + seed"
	@echo "  db-reset         Revert all migrations then re-apply"
	@echo ""
	@echo "Local dev"
	@echo "  dev-backend      cargo watch (requires cargo-watch)"
	@echo "  dev-frontend     pnpm dev"
	@echo "  lint             lint backend + frontend"
	@echo "  lint-backend     cargo clippy"
	@echo "  lint-frontend    pnpm lint"
	@echo "  test             test backend + frontend"
	@echo "  test-backend     cargo test"
	@echo "  test-frontend    pnpm test"
	@echo ""
	@echo "Logs"
	@echo "  logs             Tail all containers"
	@echo "  logs-db          Tail postgres"
	@echo "  logs-backend     Tail backend"
	@echo "  logs-frontend    Tail frontend"
	@echo "  logs-mon         Tail prometheus + grafana"
	@echo ""
	@echo "Monitoring URLs"
	@echo "  Grafana:         http://localhost:$${GRAFANA_PORT:-3001}  (admin/admin)"
	@echo "  Prometheus:      http://localhost:$${PROMETHEUS_PORT:-9090}"
	@echo ""

# ─── Full Stack ───────────────────────────────────────────────────────────────

up: up-db
	$(COMPOSE) $(BE) $(FE) $(MON) up -d --build

up-s: up-db
	$(COMPOSE) $(BE) $(FE) $(MON) up -d

down:
	$(COMPOSE) $(ALL) down --remove-orphans

restart: down up

status:
	$(COMPOSE) $(ALL) ps

clean:
	$(COMPOSE) $(ALL) down -v --remove-orphans

pull:
	$(COMPOSE) $(DB) pull

# ─── Individual Services ──────────────────────────────────────────────────────

up-db:
	$(COMPOSE) $(DB) up -d
	@echo "Waiting for Postgres to be healthy..."
	@bash -c 'until [ "$$(docker inspect --format "{{.State.Health.Status}}" $$($(COMPOSE) $(DB) ps -q postgres) 2>/dev/null)" = "healthy" ]; do sleep 1; done'
	@echo "Postgres is ready."

up-backend:
	$(COMPOSE) $(DB) $(BE) up -d --build backend

up-frontend:
	$(COMPOSE) $(FE) up -d --build frontend

up-mon:
	$(COMPOSE) $(MON) up -d

down-backend:
	$(COMPOSE) $(BE) stop backend
	$(COMPOSE) $(BE) rm -f backend

down-frontend:
	$(COMPOSE) $(FE) stop frontend
	$(COMPOSE) $(FE) rm -f frontend

down-mon:
	$(COMPOSE) $(MON) down

restart-backend: down-backend
	$(COMPOSE) $(DB) $(BE) up -d --build backend

restart-frontend: down-frontend
	$(COMPOSE) $(FE) up -d --build frontend

# ─── Build ───────────────────────────────────────────────────────────────────

build: build-backend build-frontend

build-backend:
	$(COMPOSE) $(BE) build backend

build-frontend:
	$(COMPOSE) $(FE) build frontend

# ─── Logs ────────────────────────────────────────────────────────────────────

logs:
	$(COMPOSE) $(ALL) logs -f --tail=200

logs-db:
	$(COMPOSE) $(DB) logs -f --tail=200

logs-backend:
	$(COMPOSE) $(BE) logs -f --tail=200

logs-frontend:
	$(COMPOSE) $(FE) logs -f --tail=200

logs-mon:
	$(COMPOSE) $(MON) logs -f --tail=200

# ─── Shells ───────────────────────────────────────────────────────────────────

sh-db:
	$(COMPOSE) $(DB) exec postgres psql -U $(PG_USER) -d $(PG_DB)

sh-backend:
	$(COMPOSE) $(BE) exec backend sh

sh-frontend:
	$(COMPOSE) $(FE) exec frontend sh

# ─── Database / Migrations ───────────────────────────────────────────────────

migrate:
	@echo "Running migrations against $(DATABASE_URL)..."
	sqlx migrate run \
		--source $(MIGRATIONS_DIR) \
		--database-url "$(DATABASE_URL)"

migrate-down:
	@echo "Reverting last migration..."
	sqlx migrate revert \
		--source $(MIGRATIONS_DIR) \
		--database-url "$(DATABASE_URL)"

migrate-info:
	@echo "Migration status:"
	sqlx migrate info \
		--source $(MIGRATIONS_DIR) \
		--database-url "$(DATABASE_URL)"

migrate-redo: migrate-down migrate
	@echo "Migration re-applied."

db-cli:
	@echo "Connecting to $(PG_DB) as $(PG_USER)..."
	docker exec -it \
		$$($(COMPOSE) $(DB) ps -q postgres) \
		psql -U $(PG_USER) -d $(PG_DB)

db-seed:
	@COUNT=$$(docker exec \
		$$($(COMPOSE) $(DB) ps -q postgres) \
		psql -U $(PG_USER) -d $(PG_DB) -tAc "SELECT COUNT(*) FROM users" 2>/dev/null | tr -d '[:space:]'); \
	if [ "$$COUNT" = "0" ]; then \
		echo "Seeding database..."; \
		for f in $(SEEDS_DIR)/users.sql $(SEEDS_DIR)/projects.sql \
		          $(SEEDS_DIR)/tier_configs.sql $(SEEDS_DIR)/products.sql; do \
			[ -f "$$f" ] || continue; \
			echo "  -> $$f"; \
			docker exec -i \
				$$($(COMPOSE) $(DB) ps -q postgres) \
				psql -U $(PG_USER) -d $(PG_DB) -v ON_ERROR_STOP=0 < $$f; \
		done; \
		echo "Seeding complete."; \
	else \
		echo "Skipping seed — $$COUNT user(s) already in database."; \
	fi

# Nuke volumes, restart Postgres, run all migrations + seed from scratch
db-reinit:
	@echo "Dropping database volumes..."
	$(COMPOSE) $(DB) down -v
	@echo "Starting fresh Postgres..."
	$(MAKE) up-db
	@echo "Applying all migrations..."
	$(MAKE) migrate
	@echo "Seeding database..."
	$(MAKE) db-seed

# Revert all then re-apply (non-destructive to volumes, useful for schema iteration)
db-reset: migrate-down migrate
	@echo "Database reset complete."

# ─── Local Development ───────────────────────────────────────────────────────

check-backend:
	ALSA_NO_PKG_CONFIG=1 cargo check -p api

dev-backend:
	@command -v cargo-watch >/dev/null 2>&1 || (echo "Install cargo-watch: cargo install cargo-watch" && exit 1)
	cd backend && cargo watch -x "run --bin api"

dev-frontend:
	cd frontend && pnpm dev

# ─── Lint ────────────────────────────────────────────────────────────────────

lint: lint-backend lint-frontend

lint-backend:
	cd backend && cargo clippy --all-targets --all-features -- -D warnings

lint-frontend:
	cd frontend && pnpm lint

# ─── Test ────────────────────────────────────────────────────────────────────

test: test-backend test-frontend

test-backend:
	cd backend && cargo test --all

test-frontend:
	cd frontend && pnpm test
