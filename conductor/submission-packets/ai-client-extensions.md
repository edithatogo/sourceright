# AI Client Extension Submission Packet

## Surfaces

- Claude Cowork / Claude Desktop
- Codex app
- GitHub Copilot
- Gemini CLI extensions
- Qwen CLI extensions
- OpenCode plugins/MCP

## Requirements Evidence

| Surface | Source | Retrieved | Local impact |
| --- | --- | --- | --- |
| Claude Desktop / Cowork | <https://support.anthropic.com/en/articles/10949351-getting-started-with-local-mcp-servers-on-claude-desktop> and <https://claude.com/docs/connectors/building/mcpb> | 2026-05-18 | Current state is local MCP config; Claude Desktop package claims require MCPB or other host-specific package evidence. |
| Codex app | <https://platform.openai.com/docs/docs-mcp> | 2026-05-18 | Codex supports MCP server configuration; no Sourceright Codex plugin/package claim exists until a supported package path is documented. |
| GitHub Copilot | <https://docs.github.com/en/copilot/how-tos/configure-custom-instructions/add-repository-instructions> | 2026-05-18 | Current support is repository instructions/coding-agent preparation; extension claims need separate package/listing evidence. |
| Gemini CLI extensions | <https://github.com/google-gemini/gemini-cli/blob/main/docs/extensions/reference.md> | 2026-05-18 | Gemini CLI extensions use `gemini-extension.json` and install from Git/local paths; Sourceright needs a real extension package or no-package decision. |
| Qwen CLI extensions | <https://qwenlm.github.io/qwen-code-docs/en/users/extension/introduction/> | 2026-05-18 | Qwen Code supports extensions and npm extensions with `qwen-extension.json`; package path must be proven before claims. |
| OpenCode | <https://open-code.ai/en/docs/plugins> and MCP docs | 2026-06-10 | OpenCode supports npm plugins, local plugins, and `mcp` config blocks; Track 89 owns publish path. |

## Submission tracks (83–90)

Tracks **84–89** own feature-complete packages and approval-gated submissions for
Claude, Codex, Copilot, Gemini, Qwen, and OpenCode. Track **83** owns VS Code/Open
VSX (see `vscode-open-vsx` packet). Track **90** owns Cline MCP Marketplace
(see `mcp-directories` packet).

## Local Gates

- No-package decision or package scaffold per host.
- MCP transcript smoke.
- Install smoke for Gemini/Qwen extension package if created.
- Entitlement/settings proof for Copilot enabled claims.
- No host-plugin claim from local config alone.

## Blockers

None.

## Local package evidence (Track 76)

| Surface | Evidence file | Verified |
| --- | --- | --- |
| Claude / Codex / Copilot / Generic MCP | `mcp-client-smoke-2026-06-09.md` | 2026-06-09 |
| All hosts | `package-decisions-2026-05-18.md` | 2026-05-18 |
| Submission drafts | `submission-drafts.md` | 2026-06-09 |

## Draft Submission Body

Local package/no-package decisions are hardened. Claude may use the MCPB/MCP
configuration path where supported; Codex and Copilot remain configuration/workflow
surfaces; Gemini and Qwen extension packages are deferred until host-specific
schema pinning and install smoke.

## Approval Gate

No host marketplace/package submission without explicit approval.
