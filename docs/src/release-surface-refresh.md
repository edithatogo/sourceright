# Release Surface Refresh

Use this guide when refreshing Sourceright's public release evidence after a tag, registry submission, marketplace review, or client-package change.

The source of truth is split deliberately:

- `conductor/tracks/69-marketplace-submission-evidence/marketplace-evidence.md` records detailed evidence rows.
- `docs/src/release-status.md` summarizes public release status for readers.
- `conductor/release-channels.md` records channel policy.
- `conductor/evidence-ledger.json` records allowed evidence levels and claim wording.

## Refresh Cadence

Run this review after each public release and at least before any public announcement that mentions availability in a marketplace, client, package manager, or registry.

1. Confirm the release version in `Cargo.toml`, GitHub Release evidence, crates.io, docs.rs, and MCP registry metadata.
2. For every `accepted` row, record a public URL, version or artifact id, verification date, and install metadata.
3. For every `prepared` row, confirm the repo-local manifest, config, package skeleton, or docs still exist and keep the blocking requirement explicit.
4. For every `deferred` row, confirm the missing artifact and revisit trigger still match the current roadmap.
5. Mirror reader-facing changes between `docs/src/release-status.md` and `docs-site/src/content/docs/release-status.md`.
6. Run `scripts/verify-release-surface-refresh.ps1` plus the marketplace and release-status policy tests before closing the refresh.

## Promotion Rules

Prepared metadata can move to `accepted` only when the public listing is visible and installable. Local config examples, package templates, source skeletons, or registry-ready metadata are not enough.

Deferred rows can move to `prepared` only when the missing package, wrapper, manifest, or installable artifact exists in the repo and has a documented smoke path.

## Current Watch List

These surfaces need explicit public proof before acceptance claims:

- Glama listing.
- Smithery listing or built MCPB/local package proof.
- Claude Desktop, Codex, GitHub Copilot, and generic MCP client transcript smokes.
- Zotero package or Plugin Gallery evidence.
- OJS/PKP Plugin Gallery evidence.
- VS Code Marketplace or Open VSX package evidence.
- Microsoft AppSource or sideloaded Word add-in proof.
- LibreOffice `.oxt` proof.
- Homebrew, Scoop, winget, npm, and PyPI wrapper decisions.

## Claim Boundary

Use "prepared", "deferred", "technical preview", "pilot-ready", "fixture-backed regression benchmark", and "deterministic benchmark scaffold" when that is what the evidence supports.

Do not use "accepted", "published", "available in marketplace", "production-ready institutional platform", "SOTA benchmarked performance", "legal filing compliance system", or host-plugin wording unless the matching evidence row proves it.

## Local Verification

The refresh script is deterministic and offline:

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\verify-release-surface-refresh.ps1
```

It checks the detailed evidence table, release-status mirror, docs-site mirror, and refresh guide for the current accepted/prepared/deferred boundaries.
