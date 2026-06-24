# AI Client Extension Submission Drafts (Not Submitted)

Date: 2026-06-09

**Approval gate:** All five surfaces have `external_submission_allowed: false` in
`conductor/submission-requirements.json`.

## Claude Desktop / Cowork (draft)

Subject: Sourceright MCP server for local reference verification

- Configure `mcpServers.sourceright` per `examples/mcp-clients/claude-desktop.json`
- Optional MCPB install path documented in Track 73 Smithery evidence
- **Not** a Claude plugin or Cowork marketplace listing

## Codex app (draft)

- Add Sourceright stdio MCP server per `examples/mcp-clients/codex-config.toml`
- **Not** a Codex app plugin package

## GitHub Copilot (draft)

- Enable repository instructions from `.github/copilot-instructions.md`
- Follow `examples/mcp-clients/github-copilot-coding-agent.md`
- Verify Copilot entitlement before "enabled" claims
- **Not** a GitHub Copilot extension listing

## Gemini CLI / Qwen CLI (draft)

Deferred: no `gemini-extension.json` or `qwen-extension.json` package is
shipped. Document MCP/CLI configuration only until host extension schema is
pinned and install smoke passes.

## Rollback

- Remove any public wording that implies host plugin acceptance from release notes.
- Revert release-status rows if client-specific "verified setup" is claimed without transcript evidence.
