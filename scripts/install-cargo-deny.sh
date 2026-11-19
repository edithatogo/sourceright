#!/usr/bin/env bash
# Install the current policy tool without changing the application's Rust toolchain.
set -euo pipefail

if [[ "$(uname -s)" != Linux || "$(uname -m)" != x86_64 ]]; then
  echo 'cargo-deny installer supports Linux x86_64 only' >&2
  exit 1
fi
: "${RUNNER_TEMP:?RUNNER_TEMP must identify the runner temporary directory}"
: "${GITHUB_PATH:?GITHUB_PATH must identify the runner path file}"

version=0.22.0
package="cargo-deny-${version}-x86_64-unknown-linux-musl"
# Matches the official release asset digest and its companion .sha256 file.
archive_sha256=98c56e6a408bfa5c3c60744b6015aaa6aa08f743fdeb88386d06fca035bb1786
install_dir="$(mktemp -d "${RUNNER_TEMP%/}/cargo-deny-${version}.XXXXXX")"
archive="${install_dir}/${package}.tar.gz"
trap 'rm -f "$archive"' EXIT

curl --fail --silent --show-error --location --proto '=https' --tlsv1.2 \
  --connect-timeout 15 --max-time 120 \
  "https://github.com/EmbarkStudios/cargo-deny/releases/download/${version}/${package}.tar.gz" \
  --output "$archive"
printf '%s  %s\n' "$archive_sha256" "$archive" | sha256sum --check --strict
tar --extract --gzip --file "$archive" --directory "$install_dir" \
  --strip-components=1 "${package}/cargo-deny"
actual_version="$("${install_dir}/cargo-deny" --version)"
if [[ "$actual_version" != "cargo-deny ${version}" ]]; then
  printf 'Unexpected cargo-deny version: %s\n' "$actual_version" >&2
  exit 1
fi
printf '%s\n' "$actual_version"
printf '%s\n' "$install_dir" >> "$GITHUB_PATH"
