# Wara Observability

This directory contains the optional local observability stack for developing
and testing Wara platform telemetry. It is intended to monitor Wara platform
services and workflows only, not hosted user applications.

Current local services include Prometheus, Jaeger, Fluent Bit, Elasticsearch,
and Grafana wiring through Docker Compose.

## Local Development

Validate the observability Compose file:

```bash
make observability-config
```

Start the full local stack:

```bash
make up
```

## Releasing

Observability configuration is part of the core platform release. It does not
have an independent version.

To release changes here, follow the core platform release process:

1. Update `wara.version.toml`.
2. Update backend/frontend versions to match.
3. Update `CHANGELOG.md`.
4. Run `make check-version`.
5. Tag the core platform with `vVERSION`.

Beta versions use `MAJOR.MINOR.PATCH-beta-N` and are allowed only on `dev`.
