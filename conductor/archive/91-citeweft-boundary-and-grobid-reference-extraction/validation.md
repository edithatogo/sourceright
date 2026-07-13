# Validation Evidence

Validated on 2026-07-11 with `cargo +stable-x86_64-pc-windows-gnu` and a clean
track-specific target directory.

Passed:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- full `cargo test`: 151 passed, 1 intentionally ignored live Zotero test
- `cargo check --locked`
- `cargo run --bin sourceright -- bench`: 13/13 tasks passed
- targeted GROBID tests: 8 passed
- manifest-backed fixture test: 1 passed
- CLI, documentation parity, requirements, plugin, schema, workflow harness,
  and repository policy tests
- `cargo package --locked --allow-dirty`
- `cargo publish --dry-run --locked --allow-dirty`
- `git diff --check`
- workflow harness: 18/18 workflows

Environment-gated:

- `cargo deny` is not installed on this workstation; the repository security
  workflow installs and runs Cargo Audit in CI.
- `cargo llvm-cov` is not installed; coverage remains enforced by repository
  CI policy and is not claimed as locally executed.
- Docker/GROBID runtime is unavailable; no live GROBID smoke is claimed.

The MSVC route is unsuitable on this workstation because Git's `link.exe`
shadows the Visual Studio linker and mixed Rust target artifacts were present.
The stable GNU toolchain completed the gates above.
