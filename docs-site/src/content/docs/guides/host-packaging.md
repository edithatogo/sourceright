---
title: Host Packaging
description: Host-specific plugin, client, editor, and office-suite packaging boundaries.
---

Sourceright should publish one Rust core and keep host integrations thin. Host
packages may call the CLI, local stdio MCP server, or stable JSON contracts, but
they must not reimplement reference verification or silently write canonical
CSL.

## Current host status

| Host | Track | Current state | Required proof before public plugin claim |
| --- | --- | --- | --- |
| Claude Desktop | 65 | MCP-compatible local stdio server; no Claude-specific package. | Client install snippet, MCP discovery smoke, dry-run write proof, and wording that says client configuration rather than Claude plugin. |
| Codex | 65 | CLI/MCP usable from repo workflows; no Codex-specific package. | Client guide or config example plus policy tests that keep Codex claims to CLI/MCP integration. |
| GitHub Copilot | 64, 65 | Repository coding-agent prep exists through instructions and setup workflow; no Copilot extension package. | Entitlement/settings evidence for coding-agent use, or a separate accepted Copilot extension package. |
| Generic MCP clients | 56, 57, 65 | Official MCP Registry accepted for `0.1.20`; Glama and Smithery are prepared. | Transcript smoke and separate accepted/prepared evidence per directory. |
| VS Code | 66 | Development settings exist; no VSIX product package. | VSIX or explicit deferral, Marketplace/Open VSX notes, install smoke, and diagnostics contract. |
| Microsoft Word | 67 | DOCX extraction is separate; no Office Add-in package. | Office Add-in manifest, sideload/AppSource notes, document-range provenance, reversible write plans, and fixture smoke. |
| LibreOffice Writer | 68 | No `.oxt`/UNO extension package. | `.oxt` or explicit deferral, UNO/adapter contract, local install smoke, and document-range provenance. |
| Zotero | 58 | Adapter track is in progress. | Package/install notes plus preview/apply/audit proof in a disposable library. |
| OJS/PKP | 60 | Plugin track is in progress. | Installable plugin package, permissions, fixture submission, and optional test-instance smoke. |

## Cross-host rules

- Read-only diagnostics may be exposed through CLI JSON or MCP resources.
- Write-capable actions must stay dry-run first, require explicit apply, and
  produce audit logs.
- Provider evidence must stay in `references.verification.json` and must not
  silently overwrite `references.csl.json`.
- Legal citation workflows must remain separate from academic CSL.
- A host entry is `accepted` only when the public marketplace or directory shows
  the artifact, version, date, and install metadata.

Prepared metadata, local configuration, and development settings are useful
inputs, but they are not marketplace acceptance.
