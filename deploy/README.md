# Sango Deploy Assets

This directory contains deployment examples and runtime configuration assets
that support self-hosting Sango. These files are examples or templates consumed
by operators and future installers.

Current contents include an example Nginx configuration.

## Local Development

Validate the root Docker Compose configuration from the repository root:

```bash
make compose-config
```

## Releasing

Deploy assets are part of the core platform release. They do not have an
independent version.

To release changes here, follow the core platform release process:

1. Update `sango.version.toml`.
2. Update backend/frontend versions to match.
3. Update `CHANGELOG.md`.
4. Run `make check-version`.
5. Tag the core platform with `vVERSION`.

Beta versions use `MAJOR.MINOR.PATCH-beta-N` and are allowed only on `dev`.
