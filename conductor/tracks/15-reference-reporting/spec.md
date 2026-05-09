# Reference Reporting Spec

## Goal

Make Sourceright useful as a reference integrity audit tool, including for identifying AI-related citation errors, not only as a tool that repairs references.

## Scope

- Generate a human-readable reference report from canonical CSL JSON and verification sidecar metadata.
- Surface incomplete, unverified, conflicting, queued, and unresolved references.
- Mark issue types that are useful AI-risk signals, such as plausible but unverified references, missing identifiers, duplicate IDs, and CSL/sidecar boundary violations.

## Outputs

- Markdown report from `sourceright report`.
- Later JSON report for CI, MCP clients, and downstream dashboards.

## Boundaries

The report does not claim a reference is AI-generated. It identifies citation integrity risks and patterns commonly associated with AI-assisted citation errors.
