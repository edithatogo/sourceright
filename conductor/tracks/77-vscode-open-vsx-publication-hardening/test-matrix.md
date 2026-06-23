# VS Code / Open VSX Publication Hardening Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
|---|---|---|---|
| **VSIX package build** | `vsce package` (or `npx @vscode/vsce package`) exits 0 and produces a `.vsix` file with valid manifest metadata (publisher, name, version, engines, categories, activation events, contributes). | CI log showing `vsce package` output and `.vsix` file listing. | Default-CI |
| **Install smoke** | Extension installs via `code --install-extension <vsix>` on a clean VS Code instance (or headless `code-server`). Extension activates when a CSL workspace is opened. Diagnostics appear in the Problems panel. | Transcript of install command and activation log. | opt-in-live |
| **Uninstall smoke** | Extension uninstalls cleanly via `code --uninstall-extension <publisher>.<name>` with no residual files or errors. | Transcript of uninstall command. | Default-CI |
| **Workspace Trust boundary** | Extension does not activate in untrusted workspaces; or surfaces a clear trust-required notification. | Log entry or notification capture showing trust gate. | Default-CI |
| **Live-provider opt-in** | Extension does not auto-enable any live provider on first activation; provider opt-in requires explicit user action (e.g., command palette, settings toggle). | Settings JSON or UI capture showing provider opt-in defaulting to disabled. | Default-CI |
| **Write preview-only** | All write-capable operations (e.g., fix application, CSL updates) produce a preview plan or audit log; no write silently modifies `references.csl.json`. | CLI/MCP dry-run output with `applied: false`. | Default-CI |
| **CLI reuse boundary policy** | Extension shells out to `sourceright` CLI, MCP server, or a thin LSP wrapper. No TypeScript reimplementation of reference verification logic exists. | Policy test `tests/vscode_publication_policy.rs` passes. | Default-CI |
| **Marketplace listing probe** | VS Code Marketplace listing is accessible at the expected URL and shows the correct publisher, version, and extension name. | HTTP probe log or screenshot of the Marketplace page. | opt-in-live |
| **Open VSX listing probe** | Open VSX listing is accessible at the expected URL and shows the correct namespace, version, and extension name. | HTTP probe log or screenshot of the Open VSX page. | opt-in-live |
| **Deferral enforcement** | Before activation, all extension-related docs and policy tests explicitly state that no VS Code Marketplace or Open VSX listing is claimed. No public-facing docs imply installability. | `tests/vscode_publication_policy.rs` passes; `docs/src/vscode-extension.md` and `docs-site` guide show deferral statement. | Default-CI |
| **Demand gate** | A `demand-decision.md` exists in the track directory recording the assessment outcome, criteria used, and decision timestamp. | `conductor/tracks/77-vscode-openvsx-publication-hardening/demand-decision.md` exists with populated content. | Default-CI |
