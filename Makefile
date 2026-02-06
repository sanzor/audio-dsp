-include .env

ENV_PATH ?= ./.env
PROJECT_NAME ?= audio-dsp
COMPOSE_DIR := ./yml

ENV_FILE_FLAG := $(if $(wildcard $(ENV_PATH)),--env-file $(ENV_PATH),)
COMPOSE := docker compose --project-name $(PROJECT_NAME) $(ENV_FILE_FLAG)

DB := -f $(COMPOSE_DIR)/postgres.yml
BE := -f $(COMPOSE_DIR)/backend.yml
FE := -f $(COMPOSE_DIR)/frontend.yml
MON := -f $(COMPOSE_DIR)/monitoring.yml

ALL := $(DB) $(BE) $(FE) $(MON)

.PHONY: help up down restart status logs build pull clean \
        up-db up-backend up-frontend logs-db logs-backend logs-frontend \
        up-monitoring logs-monitoring sh-db sh-backend sh-frontend

help:
	@echo "Targets:"
	@echo "  make up            # start postgres + backend + frontend + monitoring"
	@echo "  make down          # stop everything"
	@echo "  make logs          # follow logs for all services"
	@echo "  make build         # build backend + frontend images"
	@echo "  make clean         # down + remove volumes"
	@echo "  Grafana:           # http://localhost:$${GRAFANA_PORT:-3001} (admin/admin by default)"
	@echo "  Prometheus:        # http://localhost:$${PROMETHEUS_PORT:-9090}"
	@echo ""
	@echo "Single services:"
	@echo "  make up-db | up-backend | up-frontend"
	@echo "  make up-monitoring"
	@echo "  make logs-db | logs-backend | logs-frontend"
	@echo "  make logs-monitoring"
	@echo "  make sh-db | sh-backend | sh-frontend"

up:
	$(COMPOSE) $(ALL) up -d --build

down:
	$(COMPOSE) $(ALL) down

restart: down up

status:
	$(COMPOSE) $(ALL) ps

logs:
	$(COMPOSE) $(ALL) logs -f --tail=200

build:
	$(COMPOSE) $(BE) build
	$(COMPOSE) $(FE) build

pull:
	$(COMPOSE) $(DB) pull

clean:
	$(COMPOSE) $(ALL) down -v

up-db:
	$(COMPOSE) $(DB) up -d

up-backend:
	$(COMPOSE) $(BE) up -d --build

up-frontend:
	$(COMPOSE) $(FE) up -d --build

up-monitoring:
	$(COMPOSE) $(MON) up -d

logs-db:
	$(COMPOSE) $(DB) logs -f --tail=200

logs-backend:
	$(COMPOSE) $(BE) logs -f --tail=200

logs-frontend:
	$(COMPOSE) $(FE) logs -f --tail=200

logs-monitoring:
	$(COMPOSE) $(MON) logs -f --tail=200

sh-db:
	$(COMPOSE) $(DB) exec postgres sh

sh-backend:
	$(COMPOSE) $(BE) exec backend sh

sh-frontend:
	$(COMPOSE) $(FE) exec frontend sh
