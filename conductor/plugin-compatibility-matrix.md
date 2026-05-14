# Plugin Compatibility Matrix

This matrix records the target host, support status, and proof required before
public claims are allowed.

| Host or module | Target versions | Track | Current evidence | Required proof |
| --- | --- | --- | --- | --- |
| Zotero | Current supported desktop release plus one prior stable release | 58 | Fixture-backed CLI/Web API adapter; no `.xpi` or Plugin Gallery listing | Preview/apply/audit fixtures, disposable-library smoke, package/install notes, and accepted listing evidence before plugin marketplace claims. |
| EndNote | Current desktop export/import workflow | 59 | Contracted | ENW/RIS handoff, reparse checks, reference-checking guide. |
| Mendeley | TBD | 59 | Contracted | API/import decision and fixture proof or deferral. |
| Paperpile | TBD | 59 | Contracted | API/import decision and fixture proof or deferral. |
| JabRef | Current BibLaTeX/RIS workflow | 59 | Contracted | BibLaTeX/RIS round trip and limitations. |
| RefWorks | TBD | 59 | Contracted | Export/import proof or deferral. |
| OJS/PKP | Supported OJS LTS/current release | 60 | Fixture-backed source skeleton | Generic-plugin source skeleton, fixture submission, and policy tests exist; live OJS smoke and PKP Plugin Gallery acceptance remain separate evidence gates. |
| Streamlit | Current Python and Streamlit supported by `streamlit_app/requirements.txt` | 61 | Contracted | Server smoke, synthetic-data check, deployment docs. |
| MCP clients | Local stdio clients first | 56, 57, 65 | Official MCP Registry accepted for `0.1.20`; generic stdio snippets documented; Smithery prepared | Transcript smoke, registry listing, Smithery path validation, and named-client install evidence before named-client claims. |
| Claude Desktop | Current MCP-capable local client | 65 | Prepared as local stdio client configuration; no Claude package | Client config snippet, transcript smoke, dry-run write proof, and no Claude-plugin claim. |
| Codex | Current CLI/MCP-compatible agent workflow | 65 | Prepared as CLI/MCP repo-agent guidance; no Codex package | Client guide or explicit deferral, local stdio smoke, and no Codex-plugin claim. |
| GitHub Copilot | Copilot coding agent; extension/package TBD | 64, 65 | Prepared for coding-agent workflow only; no Copilot extension package | Entitlement verification or explicit admin-blocked note; separate package evidence before extension claims. |
| VS Code | Extension host current stable release | 66 | Explicitly deferred with contract | No VSIX or marketplace listing; future extension must reuse CLI/MCP diagnostics, respect Workspace Trust, and keep writes preview-only. |
| Microsoft Word | Microsoft 365 Word with Office Add-ins | 67 | Explicitly deferred with contract | No Office Add-in manifest, taskpane, sideload smoke, or AppSource listing; future add-in must prove range provenance and reversible audited writes. |
| LibreOffice Writer | Current LibreOffice stable release | 68 | Explicitly deferred with contract | No `.oxt`, UNO bridge, install smoke, or LibreOffice Extensions listing; future package must prove Writer range mapping and reversible audited writes. |
| Public providers | Provider-specific current APIs | 48, 62 | Contracted | Fixture and opt-in live smoke per provider. |
| Licensed providers | Provider-specific BYO-key APIs | 49, 62 | Contracted | Credential skip, redaction, fixture proof, optional live smoke. |
