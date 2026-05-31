#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

version="$(sed -n 's/^version = "\(.*\)"$/\1/p' sango.version.toml)"

if [[ -z "$version" ]]; then
  echo "Could not read version from sango.version.toml" >&2
  exit 1
fi

if [[ ! "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+(-beta-[0-9]+)?$ ]]; then
  echo "Invalid Sango version '$version'." >&2
  echo "Use MAJOR.MINOR.PATCH or MAJOR.MINOR.PATCH-beta-N." >&2
  exit 1
fi

backend_version="$(awk -F '"' '/^version = / { print $2; exit }' backend/Cargo.toml)"
frontend_version="$(awk -F '"' '/^[[:space:]]*"version": / { print $4; exit }' frontend/package.json)"
frontend_lock_version="$(awk -F '"' '
  /^[[:space:]]*"": \{/ { in_root_package = 1 }
  in_root_package && /^[[:space:]]*"version": / { print $4; exit }
' frontend/package-lock.json)"

if [[ "$backend_version" != "$version" ]]; then
  echo "backend/Cargo.toml version '$backend_version' does not match '$version'." >&2
  exit 1
fi

if [[ "$frontend_version" != "$version" ]]; then
  echo "frontend/package.json version '$frontend_version' does not match '$version'." >&2
  exit 1
fi

if [[ "$frontend_lock_version" != "$version" ]]; then
  echo "frontend/package-lock.json root version '$frontend_lock_version' does not match '$version'." >&2
  exit 1
fi

if [[ "${GITHUB_REF_TYPE:-}" == "tag" ]]; then
  tag="${GITHUB_REF_NAME:-}"
  expected_tag="v$version"

  if [[ "$tag" != "$expected_tag" ]]; then
    echo "Git tag '$tag' does not match version '$expected_tag'." >&2
    exit 1
  fi
fi

echo "Sango version $version is consistent."
