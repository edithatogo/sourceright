# Public Release And Registry Submission Test Matrix

| Area | Check |
| --- | --- |
| Crate packaging | `cargo package --locked` |
| Crate dry run | `cargo publish --dry-run --locked` |
| Release workflow | Validate tag-triggered release gating and artifact upload steps |
| MCP registry path | Validate server metadata, image labels, and submission preflight |
| Checksums | Verify artifact checksum generation and naming |
| Docs | Confirm release guidance matches the live publication sequence |
