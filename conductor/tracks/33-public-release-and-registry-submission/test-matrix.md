# Public Release And Registry Submission Test Matrix

| Area | Check | Current Evidence |
| --- | --- | --- |
| Crate packaging | `cargo package --locked` | Passed on 2026-05-17 clean-tree re-validation; package verified successfully for `sourceright v0.1.20`. |
| Crate dry run | `cargo publish --dry-run --locked` | Passed on 2026-05-17 clean-tree re-validation for the already-published `sourceright@0.1.20`; crates.io reported the expected existing-version warning, package verification reached upload, and upload was aborted due to dry run. Future version dry-run remains required before publishing. |
| Release workflow | Validate tag-triggered release gating and artifact upload steps | `v0.1.20` GitHub Release accepted with binaries and SHA-256 checksums. Workflow policy covered by `tests/publish_policy.rs` and `tests/release_status_policy.rs`. |
| MCP registry path | Validate server metadata, image labels, and submission preflight | Official MCP Registry accepted `0.1.20`; `server.json`, Dockerfile labels, and Glama/Smithery prepared boundaries covered by MCP distribution policy tests. |
| Checksums | Verify artifact checksum generation and naming | GitHub Release `v0.1.20` records platform binaries plus SHA-256 checksums. |
| Docs | Confirm release guidance matches the live publication sequence | `docs/src/release-status.md`, `docs/src/release-runbook.md`, and `docs/src/publishing.md` classify accepted, prepared, deferred, and n/a surfaces. |
