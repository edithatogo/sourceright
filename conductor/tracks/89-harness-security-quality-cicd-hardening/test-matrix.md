# Track 89 Test Matrix

## Workflow harness

- `./scripts/check-workflow-harness.ps1`
- `cargo test --test workflow_harness_policy`
- `actionlint`
- `zizmor --min-severity medium .github/workflows`

## Rust and contracts

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test`
- `cargo check --locked`
- `cargo deny check advisories bans sources`

## Deep verification

- Coverage, mutation, fuzz, stress, and benchmark workflows retain bounded
  evidence artifacts.
- Release dry-run and tagged release execute equivalent pre-publication gates.
- MCP server manifests, server card, tool discovery, installation, and registry
  metadata validate against their checked-in schemas and live listings.
