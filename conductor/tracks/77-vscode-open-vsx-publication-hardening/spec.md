# VS Code / Open VSX Publication Hardening Spec

## Goal

Turn the deferred VSIX contract from Track 66 into a package, install-smoke,
and public listing evidence pipeline — but only when external demand justifies
the effort.

## User Outcome

When demand triggers activation, VS Code users can install the Sourceright
extension from **VS Code Marketplace** or **Open VSX** using standard extension
commands. The extension surfaces CLI/MCP diagnostics inline, respects Workspace
Trust, keeps live providers opt-in, and keeps writes preview-only.

Until demand is confirmed, all VSIX artifacts, marketplace submissions, and
listing claims remain **explicitly deferred**. No VS Code extension package or
marketplace listing is claimed.

## Scope

- Demand assessment gate — evaluate whether VS Code extension is justified
  vs. continuing with CLI/MCP-only workflow.
- VSIX manifest and package build from the existing `examples/mcp-clients/`
  and CLI/MCP contracts.
- Local install and uninstall smoke test on a clean VS Code instance.
- VS Code Marketplace submission preparation (publisher account, manifest
  metadata, asset packaging, privacy/security notes).
- Open VSX submission preparation (namespace, publishing token, listing
  metadata).
- Cross-reference with Track 69 marketplace evidence model and Track 72
  submission requirements contracts.
- Docs updated: `docs/src/vscode-extension.md` and
  `docs-site/src/content/docs/guides/vscode.md`.
- Policy test that enforces the deferral until activation and prohibits
  extension claims before a verified listing.

## Out of Scope

- Reimplementing core reference verification logic in TypeScript. The
  extension must invoke the CLI, MCP server, or a thin LSP wrapper around
  the Rust core.
- Office add-ins, Zotero plugins, OJS/PKP plugins, LibreOffice extensions,
  or other host packages.
- Non-VS Code editor integrations (e.g., IntelliJ, Emacs, Vim).
- VS Code extension features beyond diagnostics display (e.g., inline
  fixes, code actions, custom editor views).
- Silent writes to `references.csl.json` — all write-capable actions must
  stay dry-run first, require explicit apply, and produce audit logs.

## Data Contracts

| Contract | Source | Consumer |
|---|---|---|
| VSIX manifest metadata | Track 66 deferral, `examples/mcp-clients/vscode-mcp.json` | Marketplace / Open VSX |
| CLI JSON diagnostic format | `sourceright validate-csl --json`, `sourceright report --json` | Extension diagnostics |
| MCP tool/resource protocol | `sourceright mcp` tools, resources, prompts | Extension MCP adapter |
| Workspace Trust policy | VS Code Workspace Trust API | Extension activation |
| Marketplace evidence row | Track 69 evidence model | Release notes, README |
| Submission requirements | Track 72 downstream-requirements-handoff | Marketplace / Open VSX submission |

## Claim Boundary

- "Listed" — not "widely adopted" or "production-ready institutional
  platform."
- A VS Code extension is **claimed** only when a verified listing exists on
  VS Code Marketplace **or** Open VSX with a recorded URL, version, date,
  and install metadata.
- Prepared metadata, local configuration, and development settings
  (`.vscode/settings.json`, `examples/mcp-clients/vscode-mcp.json`) are
  **not** public extension claims.
- Before activation, all extension claims must be explicitly deferred in
  docs and policy tests.

## Evidence Level Target

**Fixture-backed**

- VSIX package built from CI (`vsce package` or `npx @vscode/vsce package`).
- Install/uninstall smoke captured as a log or transcript from a clean VS
  Code instance (or headless `code-server`).
- Marketplace listing probe (opt-in-live) confirming the extension page is
  accessible.
- Open VSX listing probe (opt-in-live) confirming the extension page is
  accessible.
- Policy test (`tests/vscode_publication_policy.rs`) that enforces the
  deferral when no verified listing exists, and that extension claims are
  accompanied by listing evidence.

## Parallelization Plan

This track is the last stage in the VS Code pipeline and depends on three
completed tracks. Within the track:

1. **Demand assessment** blocks all downstream work — do not build VSIX or
   prepare listings until demand is confirmed.
2. **VSIX build, install smoke, and listing preparation** can run in
   parallel once activated, as they target separate systems (local package,
   Marketplace, Open VSX).
3. **Docs and policy test** can be updated concurrently with the build and
   listing preparation.
4. **conductor-review** gates any public listing submission — do not submit
   to Marketplace or Open VSX until review passes.
