# Marketplace Submission Evidence Test Matrix

| Area | Required check | Evidence |
| --- | --- | --- |
| Accepted | URL, version, date, install metadata, and external evidence are recorded. | `docs/src/release-status.md`, docs-site mirror, and `marketplace-evidence.md` record GitHub Release, crates.io, docs.rs, and Official MCP Registry accepted rows. |
| Prepared | Repository metadata and blocking requirements are recorded without availability claims. | GHCR, Glama, Smithery, AI clients, Zotero, and OJS/PKP are prepared rows with explicit blocking requirements. |
| Deferred | Revisit trigger and claim boundary are recorded. | VS Code/Open VSX, Microsoft AppSource, LibreOffice Extensions, package-manager wrappers, and launcher wrappers remain deferred. |
| Release docs | Release-status and publishing docs use the same evidence state. | `docs/src/release-status.md`, `docs-site/src/content/docs/release-status.md`, publishing docs, and host-packaging docs share the Track 69 model. |
| Policy tests | Public docs cannot claim installability without an accepted listing or validated local package. | `tests/marketplace_submission_evidence_policy.rs` enforces coverage and claim-boundary phrases. |
| Review | `$conductor-review` checks marketplace wording before release notes are published. | `review.md` records accepted/prepared/deferred findings and remaining external work. |
