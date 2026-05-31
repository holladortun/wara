#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

SEMVER_PATTERN='^[0-9]+\.[0-9]+\.[0-9]+(-[0-9A-Za-z]+([.-][0-9A-Za-z]+)*)?$'

read_toml_version() {
  awk -F '"' '/^version = / { print $2; exit }' "$1"
}

read_package_json_version() {
  awk -F '"' '/^[[:space:]]*"version": / { print $4; exit }' "$1"
}

check_semver() {
  local label="$1"
  local value="$2"

  if [[ -z "$value" ]]; then
    echo "Could not read version for $label." >&2
    exit 1
  fi

  if [[ ! "$value" =~ $SEMVER_PATTERN ]]; then
    echo "Invalid version '$value' for $label." >&2
    echo "Use MAJOR.MINOR.PATCH or MAJOR.MINOR.PATCH-prerelease." >&2
    exit 1
  fi
}

is_beta_version() {
  [[ "$1" =~ -beta($|[-.]) ]]
}

current_branch() {
  if [[ -n "${GITHUB_BASE_REF:-}" ]]; then
    echo "$GITHUB_BASE_REF"
    return
  fi

  if [[ "${GITHUB_REF_TYPE:-}" == "branch" && -n "${GITHUB_REF_NAME:-}" ]]; then
    echo "$GITHUB_REF_NAME"
    return
  fi

  git branch --show-current 2>/dev/null || true
}

check_beta_allowed() {
  local label="$1"
  local value="$2"
  local branch="$3"

  if [[ "$branch" == "main" ]] && is_beta_version "$value"; then
    echo "Beta version '$value' for $label is not allowed on main." >&2
    echo "Use beta versions only on dev. Other prerelease types may be used on main or dev." >&2
    exit 1
  fi
}

version="$(sed -n 's/^version = "\(.*\)"$/\1/p' sango.version.toml)"

if [[ -z "$version" ]]; then
  echo "Could not read version from sango.version.toml" >&2
  exit 1
fi

check_semver "Sango core" "$version"

branch="$(current_branch)"

if [[ -n "$branch" ]]; then
  check_beta_allowed "Sango core" "$version" "$branch"
fi

backend_version="$(read_toml_version backend/Cargo.toml)"
frontend_version="$(read_package_json_version frontend/package.json)"
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

independent_labels=()
independent_versions=()

if [[ -f cli/Cargo.toml ]]; then
  independent_labels+=("CLI")
  independent_versions+=("$(read_toml_version cli/Cargo.toml)")
fi

if [[ -f mcp/Cargo.toml ]]; then
  independent_labels+=("MCP server")
  independent_versions+=("$(read_toml_version mcp/Cargo.toml)")
fi

if [[ -f docs/package.json ]]; then
  independent_labels+=("Docs")
  independent_versions+=("$(read_package_json_version docs/package.json)")
fi

for index in "${!independent_labels[@]}"; do
  label="${independent_labels[$index]}"
  package_version="${independent_versions[$index]}"
  check_semver "$label" "$package_version"

  if [[ -n "$branch" ]]; then
    check_beta_allowed "$label" "$package_version" "$branch"
  fi
done

if [[ "${GITHUB_REF_TYPE:-}" == "tag" ]]; then
  tag="${GITHUB_REF_NAME:-}"
  case "$tag" in
    v*)
      expected_tag="v$version"
      ;;
    cli-v*)
      expected_tag="cli-v$(read_toml_version cli/Cargo.toml)"
      ;;
    mcp-v*)
      expected_tag="mcp-v$(read_toml_version mcp/Cargo.toml)"
      ;;
    docs-v*)
      expected_tag="docs-v$(read_package_json_version docs/package.json)"
      ;;
    *)
      echo "Unsupported version tag '$tag'." >&2
      echo "Use vVERSION, cli-vVERSION, mcp-vVERSION, or docs-vVERSION." >&2
      exit 1
      ;;
  esac

  if [[ "$tag" != "$expected_tag" ]]; then
    echo "Git tag '$tag' does not match expected tag '$expected_tag'." >&2
    exit 1
  fi

  tag_version="${tag#v}"
  tag_version="${tag_version#cli-v}"
  tag_version="${tag_version#mcp-v}"
  tag_version="${tag_version#docs-v}"

  if is_beta_version "$tag_version" && git branch -r --contains HEAD | grep -qE '(^|[[:space:]])origin/main$'; then
    echo "Beta tag '$tag' points at a commit reachable from origin/main." >&2
    echo "Beta versions must be released from dev, not main." >&2
    exit 1
  fi
fi

echo "Sango core version $version is consistent."
