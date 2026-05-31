# Wara Temporal

This directory contains the local Temporal development stack and bootstrap
scripts. Wara uses Temporal for background jobs such as deploys, restarts,
proxy application, log collection, and template/bulk project creation.

The backend owns workflow and activity code. This directory owns the supporting
Temporal service configuration.

## Local Development

Validate the Temporal Compose file:

```bash
make temporal-config
```

Start it as part of backend dependencies:

```bash
make deps-up
```

Run a worker:

```bash
make backend-worker
```

## Releasing

Temporal configuration is part of the core platform release. It does not have an
independent version.

To release changes here, follow the core platform release process:

1. Update `wara.version.toml`.
2. Update backend/frontend versions to match.
3. Update `CHANGELOG.md`.
4. Run `make check-version`.
5. Tag the core platform with `vVERSION`.

Beta versions use `MAJOR.MINOR.PATCH-beta-N` and are allowed only on `dev`.
