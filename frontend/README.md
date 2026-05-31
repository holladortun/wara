# Wara Frontend

The frontend is the Angular dashboard for Wara. It provides the browser UI for
managing servers, projects, environments, services, credentials, domains,
deployments, telemetry settings, and invite acceptance.

The UI uses ZardUI components, Tailwind utilities, and Biome for frontend
formatting/linting.

## Local Development

Install dependencies:

```bash
make frontend-install
```

Start the Angular dev server:

```bash
make frontend-start
```

Useful checks:

```bash
make frontend-format-check
make frontend-lint
make frontend-build
```

## Releasing A Core Platform Version

The frontend is part of the core platform release. It does not release
independently from the backend and core Docker images.

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
