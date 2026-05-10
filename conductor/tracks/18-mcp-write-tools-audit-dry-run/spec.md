# MCP Write Tools Audit Dry-Run Spec

## Goal

Add MCP write tools only after Sourceright has dry-run change plans, schema validation, audit logs, and explicit apply semantics.

## Scope

- Define write-capable MCP tools for workspace updates.
- Make dry-run the default behavior for all write-capable tools.
- Emit auditable change plans before modifying files.
- Validate planned writes against canonical and sidecar schemas.
- Require explicit apply intent before any workspace mutation.
- Record audit logs for applied changes.

## Outputs

- Write-tool contract definitions.
- Dry-run change-plan schema.
- Apply semantics and audit-log schema.
- Fixture-backed validation tests.
- MCP documentation for safe write behavior.

## Boundaries

Provider data must never silently overwrite canonical CSL. Planned updates to `references.csl.json` must be explicit, reviewable, and auditable.

This track depends on the read-only MCP runtime. It must not bypass manual review queues, export validation, or verification sidecar boundaries.
