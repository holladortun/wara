# Wara Docs

This directory contains Wara documentation assets, specs, and configuration
examples. It will also host the future documentation website.

Current contents include:

- CLI/MCP product spec.
- Example Wara config file.
- Docs package metadata for independent documentation-site versioning.

## Local Development

Run docs validation from the repository root:

```bash
make docs-validate
```

This checks documentation formatting and version policy.

## Releasing A Docs Version

The docs site is independently versioned from the core platform.

To release a new docs version:

1. Update `docs/package.json`.
2. Update docs-specific changelog or release notes once they exist.
3. Run `make docs-validate`.
4. Open and merge a PR into the release branch.
5. Create a matching docs tag, for example `docs-v0.2.0`.

Beta versions use `MAJOR.MINOR.PATCH-beta-N` and are allowed only on `dev`.
Stable and other prerelease types can be used on both `main` and `dev`.

The tag must match `docs/package.json` exactly with the `docs-v` prefix.
