# Sango

Sango is a self-hosted, Docker-native application platform for running apps on
your own servers. The goal is to provide a practical Coolify/Heroku/Render-style
experience where users can connect Docker hosts, create projects and
environments, attach Docker credentials and env vars, deploy Docker images,
Compose files, or Dockerfiles, view logs, restart services, and generate reverse
proxy configuration.

The project is early and intentionally contribution-friendly. Backend APIs are in
Rust with Axum, Toasty, PostgreSQL, Utoipa OpenAPI, Temporal workflow scaffolding,
and opt-in platform-only telemetry. The frontend is Angular with ZardUI and
Tailwind.

## What Sango Is Aiming For

Sango should become a self-hosted platform that can:

- Manage one or more Docker servers over SSH.
- Run Docker image, Docker Compose, and Dockerfile based services.
- Support projects, environments, services, domains, credentials, env vars,
  templates, and project duplication.
- Use Temporal for deploy, restart, proxy, log collection, and template jobs.
- Expose safe, documented APIs for automation.
- Offer an agent-friendly Rust CLI and Rust MCP server for local deploy flows.
- Provide opt-in OpenTelemetry for Sango platform internals only, never hosted
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

Sango reads configuration from environment variables and from
`~/.sango/config.yml`. The config file path can be overridden with
`SANGO_CONFIG_FILE`.

Environment variables override config file values. See:

- [.env.example](./.env.example)
- [docs/config.example.yml](./docs/config.example.yml)

For development, the default bootstrapped admin is:

- Email: `admin@sango.local`
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

Integration tests use `SANGO_TEST_DATABASE_URL`. The default in `.env.example`
points at the local compose database.

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

## API Usage

Swagger is available at `/docs` when `SANGO_DOCS_ENABLED=true`. Public API routes
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

Sango uses one monorepo-wide product version in
[sango.version.toml](./sango.version.toml). Stable releases are tagged from
`main` as `vMAJOR.MINOR.PATCH`. Development prereleases are tagged from `dev` as
`vMAJOR.MINOR.PATCH-beta-N`.

Git tags trigger Docker image publishing to GitHub Container Registry.

## License

Sango is licensed under the [Apache License 2.0](./LICENSE).
