# VS Code / Open VSX Publication Hardening Plan

## Phase 1 — Discover

- [ ] Review Track 66 deferral decision and the VSIX contract recorded in
      `conductor/tracks/66-vscode-extension-packaging/`.
- [ ] Review Track 69 marketplace evidence model (`marketplace-evidence.md`).
- [ ] Review Track 72 submission requirements and downstream handoff
      (`downstream-requirements-handoff.md`).
- [ ] Assess demand: Evaluate whether VS Code extension is justified vs.
      continuing with CLI/MCP-only workflow. Criteria:
  - Inbound user requests or feature votes.
  - Partner/university pilot that requires VS Code installation.
  - Revenue or adoption data that justifies the packaging and maintenance
    cost.
- [ ] Record demand decision in `conductor/tracks/77-vscode-openvsx-publication-hardening/demand-decision.md`.

## Phase 2 — Lock Spec

- [ ] Document the VSIX contract: manifest metadata, workspace trust
      boundary, diagnostics wiring, live-provider opt-in, write preview.
- [ ] Define install smoke requirements: clean VS Code instance, extension
      install via VSIX, extension activation, diagnostics display.
- [ ] Define listing evidence format per Track 69: URL, version, date,
      install metadata.
- [ ] Link to Track 72 submission requirements for Marketplace and Open VSX.

## Phase 3 — Implement

- [ ] Create VSIX manifest (`package.json`) referencing CLI/MCP contracts.
- [ ] Create extension entry point (TypeScript) that shells out to CLI JSON
      or MCP server.
- [ ] Create diagnostic mapping from CLI JSON output to VS Code Problem
      ranges.
- [ ] Ensure workspace trust check gates extension activation.
- [ ] Ensure live providers are opt-in (not auto-enabled).
- [ ] Ensure all write-capable actions are preview-only with explicit apply.
- [ ] Add `docs/src/vscode-extension.md` documenting extension architecture,
      installation, and configuration.
- [ ] Add `docs-site/src/content/docs/guides/vscode.md` for the published
      docs site.
- [ ] Create `tests/vscode_publication_policy.rs` enforcing deferral when no
      verified listing exists.

## Phase 4 — Run Checks

- [ ] Build VSIX with `vsce package` (or `npx @vscode/vsce package`).
- [ ] Run install smoke: install VSIX on clean VS Code instance, verify
      extension activates and displays diagnostics.
- [ ] Run uninstall smoke: verify clean removal.
- [ ] Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`,
      `cargo test`, `cargo check --locked`.
- [ ] Run `tests/vscode_publication_policy.rs` to verify no premature claims.
- [ ] (opt-in-live) Probe Marketplace listing URL.
- [ ] (opt-in-live) Probe Open VSX listing URL.

## Phase 5 — conductor-review

- [ ] Run `conductor-review` on all track files.
- [ ] Check that public wording does not claim adoption or production-readiness.
- [ ] Check that claim boundary is respected: "listed" not "widely adopted".
- [ ] Check that no TypeScript reimplementation of core verification logic exists.
- [ ] Check that write-capable actions are preview-only.

## Phase 6 — Apply Fixes

- [ ] Automatically apply all review findings.
- [ ] Re-run checks and policy tests after fixes.
- [ ] Update docs to reflect any contract changes.

## Phase 7 — Progress

- [ ] Submit to VS Code Marketplace only after:
  - Review passes.
  - Demand is confirmed.
  - Publisher account is set up.
- [ ] Submit to Open VSX only after:
  - Review passes.
  - Demand is confirmed.
  - Namespace and publishing token are configured.
- [ ] Record listing evidence (URL, version, date, install metadata) per
      Track 69 model.
- [ ] Update `docs/src/vscode-extension.md` with listing status.
- [ ] Update `docs-site/src/content/docs/guides/vscode.md` with listing
      status.
- [ ] Update policy test to reflect verified listing.
- [ ] Until demand is confirmed, mark all VSIX and listing items as
      **deferred** and keep the policy test enforcing deferral.
