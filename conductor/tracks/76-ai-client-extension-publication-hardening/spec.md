# Track 76: AI Client Extension Publication Hardening

## Goal

Define and harden publication paths for Claude Cowork/Claude Desktop, Codex app,
GitHub Copilot, Gemini CLI extensions, and Qwen CLI extensions without
overclaiming local MCP configuration as host plugins.

## User Outcome

Each AI client has either a mature extension/package submission path or a
documented no-package decision with local MCP configuration proof.

## Scope

- Requirements discovery for Claude Cowork, Codex app, GitHub Copilot, Gemini
  CLI extensions, and Qwen CLI extensions.
- Package contract, manifest schema, install smoke, transcript smoke, and
  submission evidence plan per host.
- No-package decisions where no registry or extension system is available.

## Out Of Scope

- Claiming host plugin acceptance from local stdio MCP configs.
- Creating external marketplace submissions without approval.

## Data Contracts

All clients call `sourceright mcp`, CLI JSON, or stable dry-run contracts.
Write-capable tools remain preview-first and auditable.

## Claim Boundary

Claude/Codex/Copilot/Gemini/Qwen plugin claims require a host-specific package
artifact and listing evidence. Otherwise the claim is local MCP configuration
only.

## Evidence Level Target

Contracted per client, hardened local package where host packaging exists, then
submitted/accepted only with external evidence.

## Parallelization Plan

Each client requirements lane can run in parallel after shared MCP smoke commands
and dry-run write semantics are frozen.

## Maturity, Stability, And Testing

Maturity requires official requirements search, package schema validation,
client install smoke, MCP transcript smoke, entitlement checks where relevant,
and policy tests that block host-plugin overclaims.
