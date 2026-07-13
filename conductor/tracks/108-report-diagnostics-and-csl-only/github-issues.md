# GitHub issue evidence

- #18: malformed or legacy verification sidecars now return an error naming `references.verification.json` and advising repair or quarantine before rerunning `report`.
- #19: CSL-only workspaces now produce a report with an informational `report.missing_verification_sidecar` diagnostic and explicitly degraded verification coverage.
- Verification: `cargo test --locked`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo check --locked`, workflow harness, release parity, and focused policy tests passed on 2026-07-13.
