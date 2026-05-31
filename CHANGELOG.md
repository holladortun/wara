# Changelog

All notable changes to Sango will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and Sango uses SemVer while it is pre-1.0.

## [Unreleased]

### Added

- Initial platform scaffold with Rust Axum backend, Angular frontend, Toasty
  persistence, JWT auth, invite flow, and contribution documentation.

## Versioning Policy

- Stable releases are tagged from `main` as `vMAJOR.MINOR.PATCH`, for example
  `v0.1.0`.
- Development prereleases are tagged from `dev` as
  `vMAJOR.MINOR.PATCH-beta-N`, for example `v0.2.0-beta-1`.
- The root `sango.version.toml` file is the source of truth for package and
  image versions.
- Package manifests must match `sango.version.toml` before a release tag is
  created.
