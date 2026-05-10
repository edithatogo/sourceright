# CI Supply-Chain Hardening Test Matrix

| Scenario | Expected result |
| --- | --- |
| Matrix Rust CI | Linux, macOS, and Windows run format, clippy, tests, and locked checks. |
| Docs build | mdBook builds in CI without broken summary references. |
| Dependency policy | `cargo-deny` checks advisories, banned duplicate policy, and sources. |
| CLI smoke | Version, help, MCP status, plugin validation, and benchmark commands run in CI. |
| Release dry run | Release workflow validates build, package, and publish dry-run. |
| Least privilege | Workflows keep permissions scoped to the job needs. |
