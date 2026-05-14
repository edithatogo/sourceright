# Smithery Distribution Readiness Test Matrix

| Scenario | Acceptance |
| --- | --- |
| Path decision | Streamable HTTP or MCPB/local path is explicitly chosen. |
| Metadata | Required Smithery config/metadata validates. |
| Install smoke | Local install/run path works or skips with clear reason. |
| Public listing | Docs say accepted only after Smithery listing exists. |
| Review loop | `$conductor-review` runs and local fixes are applied. |

## Evidence

| Scenario | Result |
| --- | --- |
| Path decision | MCPB/local is selected because Smithery URL publishing requires Streamable HTTP and Sourceright currently implements stdio MCP. |
| Metadata | `tests/smithery_distribution_policy.rs` validates `smithery/mcpb/manifest.template.json` against the local stdio runtime contract. |
| Package builder | `scripts/build-smithery-mcpb.ps1` stages a `.mcpb` bundle from an existing release binary and rewrites platform-specific manifest fields. |
| Install smoke | Skipped until a release binary is supplied to the builder; the skip reason is recorded in `review.md` and the docs. |
| Public listing | `docs/src/release-status.md` and docs-site release status classify Smithery as `prepared`, not `accepted`. |
| Review loop | Local policy test and standard Rust/docs checks are the review gate for this track. |
