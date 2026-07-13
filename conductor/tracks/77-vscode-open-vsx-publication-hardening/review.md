# Track 77 — VS Code / Open VSX Publication Hardening — Completion Review

## Review scope

Harden the local VSIX scaffold, Workspace Trust declaration, build scripts, and
isolated install/uninstall smoke for VS Code Marketplace / Open VSX publication
readiness. No external marketplace submission was performed.

## Files inspected

| Path | Status |
| --- | --- |
| requirements-evidence.md | Created |
| vsix-build-2026-05-18.md | Existing |
| vsix-smoke-2026-06-09.md | Created (refresh) |
| marketplace-metadata-draft.md | Existing |
| submission-drafts.md | Created |
| `extensions/vscode-sourceright/` | Validated via packaging policy tests |

## Test matrix verification

| Scenario | Result |
| --- | --- |
| Requirements search + inventory alignment | Pass |
| VSIX build (`build-vscode-vsix.ps1`) | Pass |
| Install/uninstall smoke (`smoke-vscode-vsix.ps1`) | Pass |
| Workspace Trust + preview-first surface | Pass |
| Marketplace metadata draft | Pass |
| No accepted listing overclaim | Pass |

## Findings

1. Local VSIX `edithatogo.sourceright-0.1.20` builds and passes isolated smoke.
2. Extension surface remains read-only (`report --json`) with untrusted-workspace
   restrictions declared.
3. Marketplace and Open VSX metadata are drafted but not submitted.
4. Plan step 6 (external submission) stays open until approval and listing
   evidence exist.

## Sign-off

Track 77 is complete at **hardened local package** evidence level. VS Code
Marketplace and Open VSX submitted/accepted claims remain blocked.
