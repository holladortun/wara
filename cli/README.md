# Sango CLI

The Sango CLI is the planned Rust command-line client for deploying and managing
Sango resources from a local environment. It will talk to the Sango public API
instead of bypassing platform safety controls.

The CLI should stay agent-friendly:

- Every input must be available through flags or arguments.
- Interactive prompts may exist only as an optional mode.
- Progress output should be readable in terminals and automation logs.
- Errors should explain what failed and what action to take next.
- A `--tree` command should show all commands and subcommands in one output.
- An `api` command should expose direct safe API calls.

## Local Development

Useful checks:

```bash
make cli-validate
```

Individual commands:

```bash
make cli-fmt-check
make cli-lint
make cli-test
make cli-build
```

## Releasing A CLI Version

The CLI is independently versioned from the core platform.

To release a new CLI version:

1. Update `cli/Cargo.toml`.
2. Update CLI-specific changelog or release notes once they exist.
3. Run `make cli-validate`.
4. Open and merge a PR into the release branch.
5. Create a matching CLI tag, for example `cli-v0.2.0`.

Beta versions use `MAJOR.MINOR.PATCH-beta-N` and are allowed only on `dev`.
Stable and other prerelease types can be used on both `main` and `dev`.

The tag must match `cli/Cargo.toml` exactly with the `cli-v` prefix.
