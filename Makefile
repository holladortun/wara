SHELL := /bin/bash
.DEFAULT_GOAL := help

DOCKER_COMPOSE ?= docker compose
FRONTEND_DIR := frontend
QUEUE ?= default

.PHONY: help
help: ## Show available make targets.
	@awk 'BEGIN {FS = ":.*## "; printf "\nSango commands:\n\n"} /^[a-zA-Z0-9_.-]+:.*## / {printf "  %-24s %s\n", $$1, $$2} END {printf "\n"}' $(MAKEFILE_LIST)

.PHONY: setup
setup: frontend-install ## Install local development dependencies.
	cargo fetch --locked

.PHONY: up
up: ## Start the full local stack with Docker Compose.
	$(DOCKER_COMPOSE) up --build

.PHONY: up-detached
up-detached: ## Start the full local stack in the background.
	$(DOCKER_COMPOSE) up -d --build

.PHONY: deps-up
deps-up: ## Start local backend dependencies.
	$(DOCKER_COMPOSE) up -d postgres temporal

.PHONY: down
down: ## Stop the local Docker Compose stack.
	$(DOCKER_COMPOSE) down

.PHONY: logs
logs: ## Tail Docker Compose logs.
	$(DOCKER_COMPOSE) logs -f

.PHONY: ps
ps: ## Show Docker Compose service status.
	$(DOCKER_COMPOSE) ps

.PHONY: compose-config
compose-config: ## Validate the root Docker Compose config.
	$(DOCKER_COMPOSE) config --quiet

.PHONY: backend-run
backend-run: ## Run the backend API locally.
	cargo run -p sango-backend

.PHONY: backend-worker
backend-worker: ## Run a Temporal worker locally. Override with QUEUE=high.
	cargo run -p sango-backend --bin temporal_worker -- $(QUEUE)

.PHONY: backend-fmt
backend-fmt: ## Format Rust code.
	cargo fmt --all

.PHONY: backend-fmt-check
backend-fmt-check: ## Check Rust formatting.
	cargo fmt --all -- --check

.PHONY: backend-lint
backend-lint: ## Run Rust clippy with CI settings.
	cargo clippy --workspace --all-targets --locked -- -D warnings

.PHONY: backend-test
backend-test: ## Run Rust tests.
	cargo test --workspace --locked

.PHONY: backend-build
backend-build: ## Build Rust binaries.
	cargo build --workspace --bins --locked

.PHONY: frontend-install
frontend-install: ## Install frontend dependencies.
	cd $(FRONTEND_DIR) && npm install

.PHONY: frontend-ci-install
frontend-ci-install: ## Install frontend dependencies from package-lock.
	cd $(FRONTEND_DIR) && npm ci

.PHONY: frontend-start
frontend-start: ## Start the Angular dev server.
	cd $(FRONTEND_DIR) && npm start

.PHONY: frontend-format
frontend-format: ## Format frontend files with Biome.
	cd $(FRONTEND_DIR) && npm run format

.PHONY: frontend-format-check
frontend-format-check: ## Check frontend formatting with Biome.
	cd $(FRONTEND_DIR) && npm run format:check

.PHONY: frontend-lint
frontend-lint: ## Run frontend Biome lint and Angular typecheck.
	cd $(FRONTEND_DIR) && npm run lint

.PHONY: frontend-test
frontend-test: ## Run frontend tests.
	cd $(FRONTEND_DIR) && npm test

.PHONY: frontend-build
frontend-build: ## Build the frontend.
	cd $(FRONTEND_DIR) && npm run build

.PHONY: temporal-config
temporal-config: ## Validate Temporal Docker Compose config.
	$(DOCKER_COMPOSE) -f temporal/docker-compose.yml config --quiet

.PHONY: observability-config
observability-config: ## Validate observability Docker Compose config.
	$(DOCKER_COMPOSE) -f observability/docker-compose.yml config --quiet

.PHONY: docs-format-check
docs-format-check: ## Check Markdown and YAML formatting.
	npx --yes prettier@3.6.2 --check \
		"README.md" \
		"ROADMAP.md" \
		"DESIGN.md" \
		"PR_GUIDE.md" \
		"CONTRIBUTING.md" \
		"CHANGELOG.md" \
		"docs/**/*.{md,yml,yaml}" \
		".github/workflows/*.yml"

.PHONY: check-version
check-version: ## Check that all package versions match sango.version.toml.
	./scripts/check-version.sh

.PHONY: format
format: backend-fmt frontend-format ## Format Rust and frontend files.

.PHONY: format-check
format-check: backend-fmt-check frontend-format-check docs-format-check ## Check formatting.

.PHONY: lint
lint: backend-lint frontend-lint ## Run backend and frontend linters.

.PHONY: test
test: backend-test ## Run stable automated tests.

.PHONY: build
build: backend-build frontend-build ## Build backend binaries and frontend.

.PHONY: validate
validate: check-version format-check lint backend-test frontend-build compose-config temporal-config observability-config ## Run the main local validation suite.
