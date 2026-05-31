# Wara Backend

The backend is the Rust Axum API for Wara. It owns authenticated platform APIs,
RBAC boundaries, Toasty/PostgreSQL persistence, Utoipa OpenAPI docs, platform
telemetry, and the Temporal worker binary used for deploy/restart/background
jobs.

## Local Development

Start backend dependencies from the repository root:

```bash
make deps-up
```

Run the API:

```bash
make backend-run
```

Run the Temporal worker:

```bash
make backend-worker
```

Useful checks:

```bash
make backend-fmt-check
make backend-lint
make backend-test
make backend-build
```

## Releasing A Core Platform Version

The backend is part of the core platform release. It does not release
independently from the frontend and core Docker images.

To release a new core platform version:

1. Update `wara.version.toml` in the repository root.
2. Update `backend/Cargo.toml` to the same version.
3. Update `frontend/package.json` and `frontend/package-lock.json` to the same
   version.
4. Update `CHANGELOG.md`.
5. Run `make check-version`.
6. Open and merge a PR into the release branch.
7. Create a matching core tag, for example `v0.2.0`.

Beta versions use `MAJOR.MINOR.PATCH-beta-N` and are allowed only on `dev`.
Stable and other prerelease types can be used on both `main` and `dev`.

Core tags publish Docker images through the release image workflow.
