# AI Client Extension Publication Hardening Spec

## Goal

Harden each AI-client extension or package path into an explicit decision —
supported installable extension, documented no-package position, or deferred with
recorded blocker — so that every surface has a verified install/config path or a
recorded non-offering rationale backed by transcript smoke.

## User outcome

An AI client user (Claude Desktop, Codex, GitHub Copilot, Gemini CLI, or Qwen
CLI) can find a documented install and configuration path for Sourceright MCP
integration, or a recorded decision that the host does not support an
installable extension, with transcript smoke proving the documented path works.

## Scope

- **Claude Desktop extension/package decision**: Confirm the no-Claude-plugin
  position is documented and the local stdio config path is stable with
  transcript smoke. Record the decision in
  `package-decisions-2026-07-01.md` as "client-configured — no Claude
  marketplace package."

- **Codex extension/package decision**: Confirm the no-Codex-plugin position
  is documented and the repo-agent/MCP config path is stable with transcript
  smoke. Record the decision as "client-configured — no Codex marketplace
  package."

- **GitHub Copilot extension boundary**: Revalidate the "prepared" coding-agent
  prep status and confirm no Copilot extension package exists. Record the
  boundary decision as "prepared coding-agent prep — no Copilot marketplace
  extension."

- **Gemini CLI extension/package decision**: Research Gemini CLI MCP client
  configuration capability. If Gemini supports local MCP stdio clients, add
  config example and transcript smoke. If not, record a no-package decision
  with blocker rationale.

- **Qwen CLI extension/package decision**: Research Qwen CLI MCP client
  configuration capability. If Qwen supports local MCP stdio clients, add
  config example and transcript smoke. If not, record a no-package decision
  with blocker rationale.

- **Transcript smoke per client**: For each client with a documented config
  path, run a manual or fixture-backed MCP transcript smoke (initialize,
  tools/list, resources/list, prompts/list via the host config) and record
  the transcript as evidence.

- **Package decisions evidence file**: Write
  `conductor/tracks/76-ai-client-extension-publication-hardening/package-decisions-2026-07-01.md`
  recording the decision, install path (or rationale), transcript smoke
  reference, and claim boundary per client.

- **Submission packet**: Scaffold
  `conductor/submission-packets/ai-client-extensions.md` with per-surface
  rows and evidence references.

- **Documentation updates**: Update `docs/src/mcp.md` and
  `docs-site/src/content/docs/mcp.md` (create if absent) with the hardened
  per-client sections.

## Out of scope

- AI marketplace listings (Claude MCP directory, Copilot extension
  marketplace, Gemini extension directory, Qwen extension store) — this track
  records package decisions only; marketplace submission is owned by future
  tracks or explicit maintainer approval per Track 72.

- Hosted MCP HTTP/WebSocket endpoint packaging — this track covers local
  stdio only.

- Non-MCP integrations (REST API wrappers, SDK packages, IDE plugins other
  than Copilot/Codex).

- New AI clients beyond the five named surfaces.

- VS Code extension packaging (owned by Track 77).

- Word add-in packaging (owned by Track 67).

- LibreOffice extension packaging (owned by Track 68).


## Data contracts

| Contract | Source | Format |
|---|---|---|
| Package decisions evidence | `conductor/tracks/76-ai-client-extension-publication-hardening/package-decisions-2026-07-01.md` | Markdown table per client: decision, install path, config file, transcript smoke ref, claim boundary |
| Transcript smoke evidence | `conductor/tracks/76-ai-client-extension-publication-hardening/transcript-smoke-2026-07-01.md` | Markdown — per-client transcript of `initialize`, `tools/list`, `resources/list`, `prompts/list` |
| Claude Desktop config example | `examples/mcp-clients/claude-desktop.json` | JSON — `mcpServers.sourceright` local stdio shape |
| Codex config examples | `examples/mcp-clients/codex-config.toml`, `codex-mcp.json` | TOML/JSON — local CLI/MCP workflow configuration |
| GitHub Copilot coding-agent prep | `examples/mcp-clients/github-copilot-coding-agent.md` | Markdown — coding-agent prep with boundary notes |
| GitHub Copilot VS Code MCP config | `examples/mcp-clients/vscode-mcp.json` | JSON — VS Code `servers` shape for Copilot agent mode |
| Gemini CLI config example | `examples/mcp-clients/gemini-cli-mcp.json` (to be created if supported) | JSON — Gemini CLI MCP client configuration |
| Qwen CLI config example | `examples/mcp-clients/qwen-cli-mcp.json` (to be created if supported) | JSON — Qwen CLI MCP client configuration |
| Generic MCP client config | `examples/mcp-clients/generic-mcp-client.json`, `generic-stdio.json` | JSON — portable `mcpServers` launcher |
| Transcript smoke requests | `examples/mcp-clients/smoke-requests.jsonl` | JSONL — manual stdio smoke sequence |
| MCP documentation | `docs/src/mcp.md` | Markdown — MCP surface, config examples, claim boundaries |
| Docs-site MCP page | `docs-site/src/content/docs/mcp.md` | Starlight page mirroring or summarising `docs/src/mcp.md` |
| Host manifest status | `examples/mcp-clients/host-manifest.json` | JSON — Track 65 host status and claim boundary |
| Marketplace evidence ledger | `conductor/evidence-ledger.json` (or Track 69 `marketplace-evidence.md`) | JSON — per-surface listing status |
| AI client extensions submission packet | `conductor/submission-packets/ai-client-extensions.md` | Markdown — per-surface rows with requirements, evidence, gate status |

## Claim boundary

**"Client-configured" not "marketplace-published".** An AI client integration may
be described as "client-configured" when:

- The package decision is documented (installable extension, no-package, or
  deferred).
- A config example exists for the client with the local stdio `sourceright mcp`
  server.
- Transcript smoke (initialize, tools/list, resources/list, prompts/list) has
  been run and recorded for that client config.
- The marketplace/listing status is explicitly recorded (not-accepted,
  not-applicable, deferred, or for-information-only).

No AI client integration may be described as "Claude-approved", "Codex-listed",
"Copilot extension", "Gemini-published", or "Qwen-published" without a public
listing URL, version, date, and install metadata in the marketplace evidence
ledger.

## Evidence level target

`default-CI` for transcript smoke that can be run against the local stdio server
(fixture-backed). `opt-in-live` for Gemini CLI and Qwen CLI research results
(manual browser research of extension/package support), and for verification
that the host-config examples match real host documentation.

## Parallelization plan

- **Subagent A — Claude Desktop decision and transcript smoke**: Revalidate the
  no-Claude-plugin position. Run transcript smoke against `claude-desktop.json`
  config. Record decision and transcript evidence.

- **Subagent B — Codex decision and transcript smoke**: Revalidate the
  no-Codex-plugin position. Run transcript smoke against `codex-config.toml`
  config. Record decision and transcript evidence.

- **Subagent C — GitHub Copilot boundary revalidation**: Review coding-agent
  prep files. Confirm no Copilot extension package exists. Record boundary
  decision. Optionally run transcript smoke if Copilot agent-mode MCP is
  confirmed functional.

- **Subagent D — Gemini CLI research**: Research Gemini CLI extension/MCP
  client support. Produce config example or no-package decision. Record
  transcript smoke if supported.

- **Subagent E — Qwen CLI research**: Research Qwen CLI extension/MCP client
  support. Produce config example or no-package decision. Record transcript
  smoke if supported.

- **Subagent F — Documentation and evidence consolidation**: Update
  `docs/src/mcp.md`, create/update `docs-site/src/content/docs/mcp.md`,
  create `package-decisions-2026-07-01.md`, create
  `transcript-smoke-2026-07-01.md`, scaffold
  `conductor/submission-packets/ai-client-extensions.md`, update evidence
  ledger.

Subagents A, B, C, D, and E can run in parallel after the spec is locked.
Subagent F depends on all five — it consolidates their outputs into the
documentation and evidence files.
