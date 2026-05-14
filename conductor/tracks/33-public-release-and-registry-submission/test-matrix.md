# Public Release And Registry Submission Test Matrix

| Area | Check | Current Evidence |
| --- | --- | --- |
| Crate packaging | `cargo package --locked` | Passed on 2026-05-15 with `cargo +stable-x86_64-pc-windows-gnu package --locked` and `CARGO_TARGET_DIR=C:\tmp\sourceright-track33-gnu-target`. |
| Crate dry run | `cargo publish --dry-run --locked` | Reached package verification on 2026-05-15, then blocked by local `C:\` disk exhaustion (`os error 112`). `crates.io` already lists `sourceright@0.1.20`; future version dry-run remains required before publishing. |
| Release workflow | Validate tag-triggered release gating and artifact upload steps | `v0.1.20` GitHub Release accepted with binaries and SHA-256 checksums. Workflow policy covered by `tests/publish_policy.rs` and `tests/release_status_policy.rs`. |
| MCP registry path | Validate server metadata, image labels, and submission preflight | Official MCP Registry accepted `0.1.20`; `server.json`, Dockerfile labels, and Glama/Smithery prepared boundaries covered by MCP distribution policy tests. |
| Checksums | Verify artifact checksum generation and naming | GitHub Release `v0.1.20` records platform binaries plus SHA-256 checksums. |
| Docs | Confirm release guidance matches the live publication sequence | `docs/src/release-status.md`, `docs/src/release-runbook.md`, and `docs/src/publishing.md` classify accepted, prepared, deferred, and n/a surfaces. |
