# Contributing To Sango

Thanks for considering a contribution.

Start here:

1. Read [README.md](./README.md) to understand the project and local setup.
2. Pick a scoped item from [ROADMAP.md](./ROADMAP.md).
3. Follow [DESIGN.md](./DESIGN.md) for backend, frontend, API, security, and AI
   contribution conventions.
4. Follow [PR_GUIDE.md](./PR_GUIDE.md) when preparing your pull request.

## Good First Contributions

Good first contributions are usually:

- Documentation improvements.
- Focused tests for existing behavior.
- Small frontend states such as loading, error, or empty states.
- One backend route test or validation improvement.
- One roadmap item marked as documentation or UI-only.

Avoid starting with deploy execution, SSH, secrets, or auth changes unless you
are prepared to add security-focused tests.

## Using AI Tools

AI-generated contributions are welcome when they are reviewed carefully. The
author remains responsible for the code.

Before opening a PR with AI-generated work:

- Read every changed file.
- Run the relevant tests.
- Check for accidental secret exposure.
- Check that OpenAPI annotations are updated for API changes.
- Remove unrelated refactors.
- Confirm the UI follows [DESIGN.md](./DESIGN.md).

## Communication

For larger work, open an issue or draft PR early and describe the intended
approach. Small roadmap items can be implemented directly.

## Version Updates

Sango uses one product version for the monorepo. The source of truth is
[sango.version.toml](./sango.version.toml).

When changing the app version, update all of these files in the same PR:

- `sango.version.toml`
- `backend/Cargo.toml`
- `frontend/package.json`
- `frontend/package-lock.json`
- `CHANGELOG.md`

Then run:

```bash
make check-version
```

Version format:

- Stable production versions use `MAJOR.MINOR.PATCH`, for example `0.1.0`.
- Development prereleases from `dev` use `MAJOR.MINOR.PATCH-beta-N`, for example
  `0.2.0-beta-1`.
- Increment the beta number for each new prerelease of the same base version.

Release tags create Docker images:

- Stable releases are tagged from `main` as `vMAJOR.MINOR.PATCH`.
- Beta releases are tagged from `dev` as `vMAJOR.MINOR.PATCH-beta-N`.
- The tag must match `sango.version.toml` exactly with a leading `v`.

Example:

```bash
git checkout dev
# update version files to 0.2.0-beta-1
make check-version
git tag v0.2.0-beta-1
git push origin v0.2.0-beta-1
```

The release image workflow publishes images to GitHub Container Registry.
