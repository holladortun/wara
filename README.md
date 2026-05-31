# Wara

Wara is a self-hosted, Docker-native application platform for running apps on
your own servers. The goal is to provide a practical Coolify/Heroku/Render-style
experience where users can connect Docker hosts, create projects and
environments, attach Docker credentials and env vars, deploy Docker images,
Compose files, or Dockerfiles, view logs, restart services, and generate reverse
proxy configuration.

The project is early and intentionally contribution-friendly. Backend APIs are in
Rust with Axum, Toasty, PostgreSQL, Utoipa OpenAPI, Temporal workflow scaffolding,
and opt-in platform-only telemetry. The frontend is Angular with ZardUI and
Tailwind.

## What Wara Is Aiming For

Wara should become a self-hosted platform that can:

- Manage one or more Docker servers over SSH.
- Run Docker image, Docker Compose, and Dockerfile based services.
- Support projects, environments, services, domains, credentials, env vars,
  templates, and project duplication.
- Use Temporal for deploy, restart, proxy, log collection, and template jobs.
- Expose safe, documented APIs for automation.
- Offer an agent-friendly Rust CLI and Rust MCP server for local deploy flows.
- Provide opt-in OpenTelemetry for Wara platform internals only, never hosted
  application telemetry.
- Keep secrets encrypted or hashed at rest and redacted in public responses.

## Repository Layout

- `backend/`: Rust Axum API, Toasty models, Utoipa OpenAPI, auth, services, and
  Temporal worker binary.
- `frontend/`: Angular dashboard using ZardUI and Tailwind utilities.
- `temporal/`: Local Temporal setup matching the project conventions.
- `observability/`: Optional local Prometheus, Jaeger, Fluent Bit,
  Elasticsearch, and Grafana stack.
- `deploy/`: Deployment examples such as Nginx config.
- `docs/`: Extended specs and configuration examples.

## Current State

Implemented foundations include:

- Rust Axum API scaffold with `/api/v1`.
- Swagger/OpenAPI docs.
- Toasty/PostgreSQL persistence for core product resources.
- Project and environment creation.
- Server records with SSH key-pair metadata and encrypted private key material.
- Services, credentials, env vars, domains, deployments, templates, and project
  duplication backed by Toasty.
- DB-backed users with Argon2 password hashes.
- RS256 JWT access tokens.
- Admin user invite flow with one-time hashed invite tokens.
- Basic Angular dashboard and invite acceptance screen.
- Platform telemetry scaffolding.

See [ROADMAP.md](./ROADMAP.md) for the next implementation slices.

## Prerequisites

- Rust 1.95, pinned by `rust-toolchain.toml`.
- Docker and Docker Compose.
- Node.js compatible with Angular 20.
- PostgreSQL for backend integration tests.

## Configuration

Wara reads configuration from environment variables and from
`~/.wara/config.yml`. The config file path can be overridden with
`WARA_CONFIG_FILE`.

Environment variables override config file values. See:

- [.env.example](./.env.example)
- [docs/config.example.yml](./docs/config.example.yml)

For development, the default bootstrapped admin is:

- Email: `admin@wara.local`
- Password: `change-me`

Override this before using a shared or exposed environment.

## Run Locally With Docker Compose

```bash
cp .env.example .env
make up
```

Default URLs:

- Frontend: `http://localhost:4200`
- Backend: `http://localhost:8080`
- Swagger UI: `http://localhost:8080/docs`
- OpenAPI JSON: `http://localhost:8080/api/openapi.json`

## Run Backend Locally

Start dependencies:

```bash
make deps-up
```

Run the backend:

```bash
make backend-run
```

Run the Temporal worker:

```bash
make backend-worker
```

Run tests:

```bash
make backend-test
```

Integration tests use `WARA_TEST_DATABASE_URL`. The default in `.env.example`
points at the local compose database.

## Database Migrations

Wara manages its schema with versioned SQL migrations in
[backend/migrations/](./backend/migrations). Each migration is recorded in a
`_wara_schema_migrations` table with a checksum, applied inside a transaction,
and serialized with an advisory lock so concurrent boots are safe.

Apply pending migrations explicitly:

```bash
make backend-migrate
```

This builds and runs the `wara-migrate` binary against `DATABASE_URL`. Run it in
CI and production before starting the backend.

Boot-time behavior is controlled by two settings:

- `WARA_DB_AUTO_MIGRATE` (default `true`): the backend applies pending
  migrations on startup, so a fresh database initializes without any extra step.
  Set it to `false` in production if you prefer to run `wara-migrate` as a
  separate deploy step.
- `WARA_DB_PUSH_SCHEMA` (default `false`): a development-only escape hatch that
  lets Toasty regenerate the schema directly while iterating on models. It
  bypasses migrations and is not a production workflow.

Migration failures surface actionable errors and never silently fall back to
schema push.

When you add or change a Toasty model, add a new
`backend/migrations/NNNN_description.sql` file with the additive DDL (never edit
an already-applied migration; the checksum guard rejects modified migrations).
During local iteration you can set `WARA_DB_PUSH_SCHEMA=true` to let Toasty shape
a scratch database, then capture the delta into a new migration file.

## Run Frontend Locally

```bash
cd frontend
npm install
npm start
```

Or from the repository root:

```bash
make frontend-install
make frontend-start
```

Build:

```bash
cd frontend
npm run build
```

Or:

```bash
make frontend-build
```

## Make Commands

Run `make help` from the repository root to see common commands for Docker
Compose, Rust, Angular, version checks, formatting, linting, builds, and local
validation.

Focused validation targets are available for separately tracked components:

- `make cli-validate`
- `make mcp-validate`
- `make docs-validate`

## API Usage

Swagger is available at `/docs` when `WARA_DOCS_ENABLED=true`. Public API routes
are versioned under `/api/v1`.

Current auth flow:

1. Login with `POST /api/v1/auth/login`.
2. Use the returned JWT as `Authorization: Bearer <token>`.
3. Admins can invite users with `POST /api/v1/admin/users`.
4. Invited users open `/accept-invite?token=...` and create a password.

## Contributing

Start with:

- [ROADMAP.md](./ROADMAP.md)
- [DESIGN.md](./DESIGN.md)
- [PR_GUIDE.md](./PR_GUIDE.md)

Keep contributions small and complete. A good PR should implement one roadmap
item end to end, including tests and documentation updates where relevant.

## Versioning

Wara uses separate versions for independently released components:

- Core platform images use [wara.version.toml](./wara.version.toml), and the
  backend/frontend package versions must match it.
- CLI versions live in [cli/Cargo.toml](./cli/Cargo.toml).
- MCP server versions live in [mcp/Cargo.toml](./mcp/Cargo.toml).
- Documentation site versions live in [docs/package.json](./docs/package.json).

Core platform releases use `vMAJOR.MINOR.PATCH` tags. Independent components use
namespaced tags such as `cli-vMAJOR.MINOR.PATCH`,
`mcp-vMAJOR.MINOR.PATCH`, and `docs-vMAJOR.MINOR.PATCH`.

Beta versions use `MAJOR.MINOR.PATCH-beta-N` and are allowed only on `dev`.
Stable and other prerelease types can be used on both `main` and `dev`.

Core platform version tags trigger Docker image publishing to GitHub Container
Registry.

## License

Wara is licensed under the [Apache License 2.0](./LICENSE).
