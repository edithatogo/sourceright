---
title: Installation
description: Install and run Sourceright locally for development and validation.
---

Use Cargo for the Rust CLI and keep the repo checked out cleanly before release
or registry work.

- Build with `cargo build --locked`.
- Run the CLI with `cargo run --locked -- --help`.
- Keep `CARGO_REGISTRY_TOKEN` set only for publish jobs.
- Use the docs site build separately from the Rust package so the documentation
  surface can evolve without changing the crate release path.
