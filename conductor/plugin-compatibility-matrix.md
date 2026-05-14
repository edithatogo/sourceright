# Plugin Compatibility Matrix

This matrix records the target host, support status, and proof required before
public claims are allowed.

| Host or module | Target versions | Track | Current evidence | Required proof |
| --- | --- | --- | --- | --- |
| Zotero | Current supported desktop release plus one prior stable release | 58 | Contracted | Preview/apply/audit fixtures, disposable-library smoke, package/install notes. |
| EndNote | Current desktop export/import workflow | 59 | Contracted | ENW/RIS handoff, reparse checks, reference-checking guide. |
| Mendeley | TBD | 59 | Contracted | API/import decision and fixture proof or deferral. |
| Paperpile | TBD | 59 | Contracted | API/import decision and fixture proof or deferral. |
| JabRef | Current BibLaTeX/RIS workflow | 59 | Contracted | BibLaTeX/RIS round trip and limitations. |
| RefWorks | TBD | 59 | Contracted | Export/import proof or deferral. |
| OJS/PKP | Supported OJS LTS/current release | 60 | Contracted | Installable plugin package, fixture submission, optional test-instance smoke. |
| Streamlit | Current Python and Streamlit supported by `streamlit_app/requirements.txt` | 61 | Contracted | Server smoke, synthetic-data check, deployment docs. |
| MCP clients | Local stdio clients first | 56, 57 | Fixture-backed for local metadata; Smithery contracted | Transcript smoke, registry listing, Smithery path validation. |
| Claude Desktop | Current MCP-capable local client | 65 | Contracted | Client config snippet, transcript smoke, dry-run write proof, and no Claude-plugin claim. |
| Codex | Current CLI/MCP-compatible agent workflow | 65 | Contracted | Client guide or explicit deferral, local stdio smoke, and no Codex-plugin claim. |
| GitHub Copilot | Copilot coding agent; extension/package TBD | 64, 65 | Prepared for coding-agent workflow | Entitlement verification or explicit admin-blocked note; separate package evidence before extension claims. |
| VS Code | Extension host current stable release | 66 | Contracted | VSIX package or deferral, diagnostics mapping, Marketplace/Open VSX notes, and install smoke. |
| Microsoft Word | Microsoft 365 Word with Office Add-ins | 67 | Contracted | Add-in manifest/taskpane or deferral, sideload/AppSource notes, range provenance, and reversible write proof. |
| LibreOffice Writer | Current LibreOffice stable release | 68 | Contracted | `.oxt`/UNO package or deferral, Writer range mapping, install smoke, and reversible write proof. |
| Public providers | Provider-specific current APIs | 48, 62 | Contracted | Fixture and opt-in live smoke per provider. |
| Licensed providers | Provider-specific BYO-key APIs | 49, 62 | Contracted | Credential skip, redaction, fixture proof, optional live smoke. |
