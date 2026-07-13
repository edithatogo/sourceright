# AI Client Package Decisions

Date: 2026-05-18

## Claude Cowork / Claude Desktop

Decision: use local MCP configuration and the Smithery/Claude-compatible MCPB
bundle path where supported. Do not claim a Claude plugin unless a
host-specific package/listing is submitted and accepted.

Local package evidence: Track 73 Smithery MCPB build
(`conductor/tracks/73-mcp-directory-submission-hardening/smithery-mcpb-build-2026-06-09.md`).

## Codex App

Decision: use MCP server configuration. No Codex app plugin/package claim is
made because no Sourceright-specific Codex package registry path is proven in
this repo.

## GitHub Copilot

Decision: use repository instructions and coding-agent workflow contracts. No
GitHub Copilot extension claim is made from repository instructions alone.

## Gemini CLI Extensions

Decision: create a host-specific extension scaffold only after the Gemini CLI
extension schema is pinned and a local install smoke can run. Until then,
Sourceright remains MCP/CLI-configurable for Gemini workflows, not a published
Gemini CLI extension.

## Qwen CLI Extensions

Decision: create a host-specific extension scaffold only after the Qwen Code
extension schema is pinned and a local install smoke can run. Until then,
Sourceright remains MCP/CLI-configurable for Qwen workflows, not a published
Qwen CLI extension.

## Claim Boundary

Local MCP configuration is not a host plugin. Host plugin claims require a
host-specific package artifact, install smoke, and listing or submission
evidence.
