# AI Client MCP Smoke — Local Evidence

Date: 2026-06-09  
Version: 0.1.20

## MCP status smoke

```powershell
C:\tmp\sourceright-target-track73\x86_64-pc-windows-gnu\release\sourceright.exe mcp status --json
```

Result:

| Field | Value |
| --- | --- |
| server_mode | stdio |
| transport | stdio |
| available_tools | 14 |
| available_resources | 8 |
| available_prompts | 5 |

## Default-CI policy tests (2026-06-09)

```text
cargo test --test ai_client_mcp_packaging_policy
  ai_client_examples_cover_required_hosts_without_plugin_claims ... ok
  ai_client_release_status_keeps_config_separate_from_marketplace_acceptance ... ok
  ai_client_mcp_smoke_contract_is_documented ... ok
```

Target dir: `C:\tmp\sourceright-target-track76` (GNU toolchain, locked deps).

## Transcript contract (documented, manual)

`examples/mcp-clients/smoke-requests.jsonl` defines the stdio sequence:

- `initialize`
- `tools/list`
- `resources/list`
- `prompts/list`
- dry-run `workspace.init`

Per-host client transcript smoke (Claude Desktop UI, Codex app UI, Copilot
entitlement) remains opt-in and is not recorded in this slice.

## Claude MCPB cross-reference

Optional Claude-compatible MCPB path is hardened in Track 73:
`conductor/tracks/73-mcp-directory-submission-hardening/smithery-mcpb-build-2026-06-09.md`.

## Claim boundary

This evidence supports hardened local MCP configuration and protocol-level smoke
contracts. It does **not** claim Claude, Codex, Copilot, Gemini, or Qwen
marketplace extension acceptance.
