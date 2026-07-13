# Host Packaging

Sourceright should publish one Rust core and keep host integrations thin. Host
packages may call the CLI, local stdio MCP server, or stable JSON contracts, but
they must not reimplement reference verification or silently write canonical
CSL.

## Current Host Status

| Host | Track | Current state | Required proof before public plugin claim |
| --- | --- | --- | --- |
| Claude Desktop | 65 | MCP-compatible local stdio server with `examples/mcp-clients/claude-desktop.json`; no Claude-specific package. | MCP discovery smoke, dry-run write proof, and wording that says client configuration rather than Claude plugin. |
| Codex | 65 | CLI/MCP usable from repo workflows with `examples/mcp-clients/codex-config.toml`; no Codex-specific package. | Client smoke plus policy tests that keep Codex claims to CLI/MCP integration. |
| GitHub Copilot | 64, 65 | Repository coding-agent prep exists through instructions and setup workflow; `examples/mcp-clients/vscode-mcp.json` covers VS Code MCP settings; no Copilot extension package. | Entitlement/settings evidence for coding-agent use, or a separate accepted Copilot extension package. |
| Gemini CLI extensions | 76 | No extension package is currently claimed; requirements and package path are planned. | Official extension requirements, manifest/package validation, install smoke, MCP transcript proof, and submission evidence. |
| Qwen CLI extensions | 76 | No extension package is currently claimed; requirements and package path are planned. | Official extension requirements, manifest/package validation, install smoke, MCP transcript proof, and submission evidence. |
| Generic MCP clients | 56, 57, 65 | Official MCP Registry accepted for `0.1.20`; Glama and Smithery are prepared; `examples/mcp-clients/generic-mcp-client.json` documents local stdio launch. | Transcript smoke and separate accepted/prepared evidence per directory. |
| VS Code | 66, 77 | Local VSIX scaffold builds and passes isolated install/uninstall smoke; `.vscode` settings and `examples/mcp-clients/vscode-mcp.json` remain development/client configuration, not Marketplace acceptance. | Marketplace/Open VSX submission evidence and accepted listing evidence before public plugin acceptance claims. |
| Microsoft Word | 67 | Explicitly deferred with a future Office Add-in contract; DOCX extraction is separate and no Office Add-in package exists. | Office Add-in manifest, sideload/AppSource notes, document-range provenance, reversible write plans, and fixture smoke. |
| LibreOffice Writer | 68 | Explicitly deferred with a future Writer extension contract; ODT/DOCX processing is separate. No `.oxt`/UNO extension package exists. | `.oxt` package, UNO/adapter contract, local install/uninstall smoke, Writer range provenance, reversible write plans, and listing evidence. |
| Zotero | 58 | CLI/Web API adapter is fixture-backed with preview/apply/audit semantics; no Zotero `.xpi` or Plugin Gallery listing is claimed. | Package decision, install proof, disposable-library smoke, and accepted listing evidence before plugin marketplace claims. |
| OJS/PKP | 60 | Generic-plugin source skeleton and fixture-backed screening are present; PKP Plugin Gallery acceptance is not claimed. | Live OJS handler/settings-form/workflow-template wiring, compatibility testing, and Gallery listing evidence. |

## Cross-Host Rules

- Read-only diagnostics may be exposed through CLI JSON or MCP resources.
- Write-capable actions must stay dry-run first, require explicit apply, and
  produce audit logs.
- Provider evidence must stay in `references.verification.json` and must not
  silently overwrite `references.csl.json`.
- Legal citation workflows must remain separate from academic CSL.
- A host entry is `accepted` only when the public marketplace or directory shows
  the artifact, version, date, and install metadata.

## Submission Evidence

| Evidence level | Meaning |
| --- | --- |
| Contracted | Track exists with host scope, owner paths, and claim boundary. |
| Scaffolded | Config, manifest, or docs exist but no installable package is proven. |
| Package-built | A local package builds and has install/uninstall smoke proof. |
| Publicly accepted | External marketplace or directory listing is verified with URL, version, and date. |

Prepared metadata, local configuration, and development settings are useful
inputs, but they are not marketplace acceptance.

These host entries are an explicit deferral of marketplace/plugin claims until
the matching package artifact, install proof, and listing evidence exist.

## Track 65 Client Artifacts

Track 65 keeps host examples in `examples/mcp-clients/` and uses
`examples/mcp-clients/host-manifest.json` as the local inventory of client
status. Every example launches the same server command:

```text
sourceright mcp
```

The smoke path is intentionally host-neutral:

```bash
sourceright --version
sourceright mcp status
sourceright mcp tools --json
sourceright mcp resources --json
sourceright mcp prompts --json
```

For stdio-level proof, run `sourceright mcp` and send the JSON-RPC lines from
`examples/mcp-clients/smoke-requests.jsonl`. Passing smoke means the client can
initialize the server, discover tools/resources/prompts, and receive a dry-run
`workspace.init` plan with `applied: false`. It does not prove public directory
acceptance or host-specific extension packaging.

No Claude, Codex, or Copilot marketplace package exists in this repository.
Those hosts stay at prepared client-configuration or coding-agent-prep status
until a separate package artifact, listing URL, version, date, and install
metadata are recorded.

Track 69 records the cross-host marketplace evidence model in
`conductor/tracks/69-marketplace-submission-evidence/marketplace-evidence.md`.
Track 72 records the cross-surface submission contract in
`conductor/submission-contracts.md`; Tracks 73-77 harden MCP directories,
citation managers, journal platforms, AI clients, and VS Code/Open VSX toward
submission evidence.
