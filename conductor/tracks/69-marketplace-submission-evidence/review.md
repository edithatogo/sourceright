# Track 69 Marketplace Submission Evidence Review

## Decision

Track 69 is completed as a repo-local marketplace evidence model. It records
accepted, prepared, deferred, and not-applicable states across core registries,
MCP directories, AI client configs, Zotero, OJS/PKP, editor extensions,
office-suite add-ins, and package-manager channels.

## Evidence Checked

| Surface | Result |
| --- | --- |
| Accepted registries | GitHub Release, crates.io, docs.rs, and Official MCP Registry have URL, version, date, and install metadata in `docs/src/release-status.md` and the docs-site mirror. |
| Prepared channels | GHCR, Glama, Smithery, Claude Desktop, Codex, GitHub Copilot, generic MCP clients, Zotero, and OJS/PKP are recorded as prepared without accepted-listing claims. |
| Deferred channels | VS Code/Open VSX, Microsoft AppSource/Word, LibreOffice Extensions, Homebrew, Scoop, winget, npm, and PyPI are recorded with blocking requirements and revisit triggers. |
| Host packaging | `docs/src/host-packaging.md` and the public docs mirror keep local config, source skeletons, and package metadata separate from marketplace acceptance. |
| Policy tests | `tests/marketplace_submission_evidence_policy.rs` enforces host coverage and claim boundaries. |

## Remaining External Work

- Verify direct GHCR package-page visibility if package permissions allow it.
- Verify Glama and Smithery listings only after the relevant external listings
  or built package artifacts exist.
- Submit VS Code, Word, LibreOffice, Zotero `.xpi`, or PKP Gallery packages
  only after their owning package tracks produce installable artifacts and smoke
  evidence.
- Keep private account submissions outside release notes until public listing
  evidence exists.
