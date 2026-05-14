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
| Generic MCP clients | 56, 57, 65 | Official MCP Registry accepted for `0.1.20`; Glama and Smithery are prepared; `examples/mcp-clients/generic-mcp-client.json` documents local stdio launch. | Transcript smoke and separate accepted/prepared evidence per directory. |
| VS Code | 66 | Explicitly deferred with a future VSIX contract; `.vscode` settings and `examples/mcp-clients/vscode-mcp.json` are not an extension package. | VSIX package, Workspace Trust implementation, diagnostics fixture, install smoke, and Marketplace/Open VSX listing evidence. |
| Microsoft Word | 67 | DOCX extraction is separate; no Office Add-in package. | Office Add-in manifest, sideload/AppSource notes, document-range provenance, reversible write plans, and fixture smoke. |
| LibreOffice Writer | 68 | No `.oxt`/UNO extension package. | `.oxt` or explicit deferral, UNO/adapter contract, local install smoke, and document-range provenance. |
| Zotero | 58 | Adapter track is in progress. | Package/install notes plus preview/apply/audit proof in a disposable library. |
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
