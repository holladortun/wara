# Changelog

All notable changes to Wara will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and Wara uses SemVer while it is pre-1.0.

## [Unreleased]

### Added

- Initial platform scaffold with Rust Axum backend, Angular frontend, Toasty
  persistence, JWT auth, invite flow, and contribution documentation.

## Versioning Policy

- Core platform image releases are tagged as `vMAJOR.MINOR.PATCH`, for example
  `v0.1.0`. The root `wara.version.toml` file is the source of truth for the
  core platform version.
- Backend and frontend package manifests must match `wara.version.toml` before
  a core platform release tag is created.
- CLI, MCP server, and documentation site versions are independent from the core
  platform version and live in their own package manifests.
- Independent component release tags use namespaces: `cli-vVERSION`,
  `mcp-vVERSION`, and `docs-vVERSION`.
- Beta versions use `MAJOR.MINOR.PATCH-beta-N` and are allowed only on `dev`.
  Stable and other prerelease types can be used on both `main` and `dev`.
