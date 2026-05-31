# Wara MCP Server

The Wara MCP server is the planned Rust Model Context Protocol server for
letting local AI agents interact with Wara safely. It should expose controlled
platform operations through Wara APIs rather than raw shell, raw SSH, Docker
socket proxying, or direct database access.

Expected responsibilities:

- Authenticate against a Wara instance.
- Expose safe deploy, restart, log, project, service, and environment tools.
- Keep secret values redacted.
- Return actionable errors for agents.
- Avoid app telemetry exfiltration; it should operate on platform-scoped data
  only.

## Local Development

Useful checks:

```bash
make mcp-validate
```

Individual commands:

```bash
make mcp-fmt-check
make mcp-lint
make mcp-test
make mcp-build
```

## Releasing An MCP Version

The MCP server is independently versioned from the core platform.

To release a new MCP server version:

1. Update `mcp/Cargo.toml`.
2. Update MCP-specific changelog or release notes once they exist.
3. Run `make mcp-validate`.
4. Open and merge a PR into the release branch.
5. Create a matching MCP tag, for example `mcp-v0.2.0`.

Beta versions use `MAJOR.MINOR.PATCH-beta-N` and are allowed only on `dev`.
Stable and other prerelease types can be used on both `main` and `dev`.

The tag must match `mcp/Cargo.toml` exactly with the `mcp-v` prefix.
