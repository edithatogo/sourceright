# MCP Server Runtime Spec

## Goal

Implement an actual local MCP server transport for Sourceright's existing read-only MCP contracts.

## Scope

- Add a `sourceright mcp` runtime that starts a local MCP server.
- Expose the existing read-only tools, resources, and prompts defined by the CLI/MCP contract.
- Keep existing status and manifest commands working without behavior regressions.
- Support deterministic local workspace operation with no default live provider calls.
- Document server startup, supported contracts, and client integration expectations.

## Outputs

- Local MCP server runtime.
- Read-only tool/resource/prompt exposure matching existing contracts.
- Startup and capability manifest output.
- Updated MCP documentation.
- Fixture-backed runtime tests.

## Boundaries

This track must not introduce write-capable MCP tools. Writes remain deferred until dry-run semantics, schema validation, audit logs, and explicit apply behavior are stable.

The runtime must preserve the separation between canonical `references.csl.json` and verification/provenance sidecar data.
