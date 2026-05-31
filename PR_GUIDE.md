# Pull Request Guide

This guide defines how to prepare pull requests for Wara.

## Before You Start

1. Pick one item from [ROADMAP.md](./ROADMAP.md).
2. Check whether it depends on a `Required First` item.
3. Open or comment on an issue if the scope is unclear.
4. Keep the PR focused on one complete behavior.

Avoid PRs that mix unrelated refactors, formatting, feature work, and dependency
updates. If cleanup is needed, make it small and directly related to the feature.

## PR Size

Good PRs are:

- Atomic: one roadmap item or one coherent subtask.
- Complete: code, tests, docs, and OpenAPI updates where needed.
- Reviewable: no unrelated churn.
- Revertable: the change can be backed out without breaking unrelated features.

Split the work when a PR touches multiple independent areas.

## Branch Names

Use short descriptive branch names:

- `feat/api-tokens`
- `feat/server-check`
- `fix/validation-errors`
- `docs/deployment-guide`

## Commit Messages

Use direct, descriptive commit messages:

- `Add DB-backed API token model`
- `Implement Nginx proxy preview tests`
- `Document production config file`

Do not rely on vague messages like `changes`, `updates`, or `fix stuff`.

## Required Checks Before Opening A PR

Run the checks that match your change:

```bash
make backend-fmt
make backend-test
```

For frontend changes:

```bash
make frontend-format-check
make frontend-lint
make frontend-build
```

Frontend formatting and linting use Biome. Run `npm run format` before opening
a PR when Biome reports formatting differences.

You can run `make help` from the repository root to list all common local
commands. For broader validation before review, run `make validate`.

When frontend tests are added:

```bash
cd frontend
npm test
```

For CLI changes:

```bash
make cli-validate
```

For MCP server changes:

```bash
make mcp-validate
```

For documentation changes:

```bash
make docs-validate
```

If your work touches Docker Compose, Temporal, or observability, also run the
relevant local stack and document what you verified.

## Backend PR Requirements

Backend PRs should include:

- Thin route handlers.
- Business logic in `services/`.
- Toasty records in `entities/`.
- Utoipa annotations for public API changes.
- Validation through `axum-valid` for request bodies.
- Tests for success and failure paths.
- Redaction tests for secrets.

Do not add:

- Raw shell execution APIs.
- Raw SSH command APIs.
- Docker socket proxy APIs.
- Direct database query APIs.
- APIs that return decrypted secrets.

## Frontend PR Requirements

Frontend PRs should include:

- ZardUI components where available.
- Tailwind utilities instead of custom CSS where practical.
- Loading, error, empty, and success states.
- Mobile-safe layout.
- Accessible form labels and buttons.
- No secret values re-rendered after submit.

Do not add:

- Marketing pages for operational workflows.
- Decorative blobs, bokeh, or one-note gradient backgrounds.
- Nested cards.
- Unbounded text that can overflow controls.

## API PR Requirements

API changes must include:

- Route implementation.
- Request/response DTOs.
- `utoipa::ToSchema` where exposed.
- `#[utoipa::path]` annotations.
- OpenAPI registration in `backend/src/openapi.rs`.
- Tests for auth, validation, success, and relevant failure states.

For breaking changes, update README or docs examples.

## Security Review Checklist

Every PR should answer:

- Does this touch secrets, auth, tokens, SSH keys, env vars, or logs?
- Are secrets encrypted or hashed at rest?
- Are secrets redacted in API responses?
- Could user input become shell syntax?
- Could hosted app logs leak into platform telemetry?
- Does RBAC or admin-only access apply?
- Are negative tests included?

If the answer is uncertain, call it out in the PR description.

## PR Description Template

Use this structure:

```markdown
## Summary

- What changed?
- Why?

## Scope

- What is included?
- What is intentionally not included?

## Tests

- [ ] `make backend-fmt`
- [ ] `make backend-test`
- [ ] `make frontend-build`
- [ ] Other:

## Security Notes

- Secret handling:
- Auth/RBAC impact:
- Telemetry/logging impact:

## Screenshots

Add screenshots for frontend changes.
```

## CI Behavior

CI runs automatically for PRs from contributors with write access to the
repository. For forked PRs from external contributors, CI is gated until a
maintainer approves by re-running the workflow. CI only runs for PRs targeting
`dev` or `main`.

CI is split by component so status checks are easy to track:

- Core CI covers backend, frontend, Temporal, Compose, observability, and core
  image versioning.
- CLI CI covers `cli/**`.
- MCP CI covers `mcp/**`.
- Docs CI covers documentation, repository Markdown, and docs package versioning.

Each workflow has its own path filters and only runs for relevant changes.

## Review Expectations

Reviewers should prioritize:

- Correctness.
- Security.
- Data model compatibility.
- API contract stability.
- Test quality.
- UI consistency with [DESIGN.md](./DESIGN.md).

Style comments should point to a documented convention where possible.

## Documentation Updates

Update documentation when changing:

- Setup steps.
- Config variables.
- Public APIs.
- Auth flows.
- Deployment behavior.
- CLI/MCP behavior.
- Contributor conventions.

## Dependency Updates

Dependency updates should be isolated unless required for the feature.

For dependency PRs, include:

- Why the dependency is needed.
- Why the version was chosen.
- Any security or license considerations.
- Whether it affects build size or runtime footprint.

## Version Change PRs

Core platform version changes must update:

- `wara.version.toml`
- `backend/Cargo.toml`
- `frontend/package.json`
- `frontend/package-lock.json`
- `CHANGELOG.md`

Independently released component version changes must update that component's
package metadata instead:

- CLI: `cli/Cargo.toml`
- MCP server: `mcp/Cargo.toml`
- Documentation site: `docs/package.json`

Run:

```bash
make check-version
```

Use `MAJOR.MINOR.PATCH` for stable releases. Beta versions use
`MAJOR.MINOR.PATCH-beta-N` and are allowed only on `dev`; they must not be used
on `main`. Other prerelease types may be used on both `main` and `dev`.

## Generated Files

Do not commit build outputs unless the repository already tracks that artifact
for a clear reason. In general:

- Do not commit `target/`.
- Do not commit frontend `dist/`.
- Do not commit local databases or logs.

## Maintainer Notes

Maintainers may ask contributors to split PRs even when the code works. This is
to keep review reliable and make regressions easier to isolate.
