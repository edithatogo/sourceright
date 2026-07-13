#!/usr/bin/env bash
set -euo pipefail

target_directory="${CARGO_TARGET_DIR:-${TMPDIR:-/tmp}/sourceright-target}"
export CARGO_TARGET_DIR="$target_directory"
export RUSTUP_TOOLCHAIN="${RUSTUP_TOOLCHAIN:-stable}"
mkdir -p "$CARGO_TARGET_DIR"

cargo fmt --all --check
cargo test --locked
cargo clippy --locked --all-targets -- -D warnings
cargo check --locked

printf 'Cross-platform Rust gates passed using %s.\n' "$CARGO_TARGET_DIR"
