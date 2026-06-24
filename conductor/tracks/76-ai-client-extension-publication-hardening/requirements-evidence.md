# Track 76 — AI Client Extension Requirements Evidence

Date: 2026-06-09

## Inventory alignment

| Surface | Inventory id | Submission packet |
| --- | --- | --- |
| Claude Cowork / Claude Desktop | `claude-cowork` | `conductor/submission-packets/ai-client-extensions.md` |
| Codex app | `codex-app` | same |
| GitHub Copilot | `github-copilot` | same |
| Gemini CLI extensions | `gemini-cli-extensions` | same |
| Qwen CLI extensions | `qwen-cli-extensions` | same |

## Official requirements sources (searched 2026-05-18)

| Surface | Source |
| --- | --- |
| Claude Desktop / Cowork | Anthropic MCP local server docs; MCPB connector path |
| Codex app | OpenAI MCP documentation |
| GitHub Copilot | GitHub Copilot custom instructions / coding agent docs |
| Gemini CLI | `google-gemini/gemini-cli` extension reference |
| Qwen CLI | Qwen Code extension introduction |

## Package decisions

Frozen in `package-decisions-2026-05-18.md`:

| Host | Decision |
| --- | --- |
| Claude | Local MCP config + optional MCPB bundle path (Track 73) |
| Codex | MCP server configuration only |
| Copilot | Repository instructions / coding-agent prep only |
| Gemini CLI | No-package until extension schema pinned + install smoke |
| Qwen CLI | No-package until extension schema pinned + install smoke |

## Claim boundary

Local MCP JSON/TOML configuration is not a host plugin. Host plugin or
marketplace claims require a host-specific package artifact, install smoke, and
accepted listing evidence.
