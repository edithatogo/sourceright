# Test Matrix

## Zotero packaging and install smoke

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Packaging decision revalidation | `plugins/zotero/packaging-revalidation-<date>.md` exists with finding: decision stands or revisit required | Revalidation document mentions Track 58 decision date, Zotero API status, demand signals | default-CI |
| Install smoke (fixture-backed) | `sourceright citation-sync --preview --remote-fixture fixtures/providers/zotero/preview-exact-match.json` exits 0 and prints valid `sourceright.citation_sync.v1` JSON | Test run log or CI trace showing zero exit and valid JSON schema | default-CI |
| Zotero Plugin Gallery listing probe | Zotero Plugin Gallery listing status is recorded as "not-applicable" (no `.xpi` package) | `docs/src/citation-manager-integration.md` row with Zotero listing status = not-applicable | opt-in-live |
| Zotero Forums listing probe | Zotero Forums listing status is recorded as "deferred" (no browser plugin to announce) | `docs/src/citation-manager-integration.md` row with Zotero Forums status = deferred | opt-in-live |
| Distribution notes separate shareable package from acceptance | `docs/src/citation-manager-integration.md` and docs-site guide state "package-ready" not "marketplace-accepted" for Zotero | Docs review confirms no "Zotero Plugin Gallery accepted" or "Zotero-listed" claims | default-CI |

## EndNote ENW handoff validation

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| ENW export produces valid tagged records | `sourceright export --format enw` against `examples/csl-examples.csla.json` exits 0 and writes `references.enw` with record count matching source | Generated `references.enw` file with structural validation log | default-CI |
| ENW structural validation | Parsed ENW contains same record count and key identifiers (DOI, title) as source CSL JSON | `tests/export_roundtrip_enw.rs` test passes | default-CI |
| ENW author/editor fidelity | Each ENW record preserves person count (author/editor tags) from source CSL item | Roundtrip test asserts author count per record | default-CI |
| EndNote listing probe | EndNote listing status is recorded as "not-applicable" (file-format only, no live adapter) | `docs/src/citation-manager-integration.md` row with EndNote listing status = not-applicable | opt-in-live |
| EndNote distribution notes | Documentation separates shareable ENW file export from official EndNote marketplace acceptance | Docs review confirms no "EndNote-accepted" or "EndNote-listed" claims | default-CI |

## RIS import/export roundtrip

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| RIS export produces valid blocks | `sourceright export --format ris` against representative fixture exits 0 and writes `references.ris` with `TY  -`/`ER  -` blocks matching source record count | Generated `references.ris` file with structural validation log | default-CI |
| RIS roundtrip — record count | Parsed RIS block count matches source CSL record count | `tests/export_roundtrip_ris.rs` test passes | default-CI |
| RIS roundtrip — DOI/URL preservation | DOI and URL identifiers survive export/reparse unchanged | Roundtrip test asserts DOI and URL fields match source | default-CI |
| RIS roundtrip — author/editor counts | Each RIS block preserves person count from source CSL item | Roundtrip test asserts author count per block | default-CI |
| RIS format edge cases | Ampersands, angle brackets, non-ASCII names, and multi-value fields (authors, keywords) survive roundtrip | Test with fixture containing special characters and multi-value fields | default-CI |

## Marketplace listing probes

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Zotero Plugin Gallery status recorded | Listing status field populated in `docs/src/citation-manager-integration.md` (accepted/prepared/deferred/not-applicable) | Docs review confirms status field per citation manager | opt-in-live |
| EndNote listing status recorded | Listing status field populated for EndNote | Docs review confirms status field | opt-in-live |
| Mendeley listing status recorded | Listing status field populated for Mendeley (deferred/not-applicable per Track 59) | Docs review confirms status field | opt-in-live |
| Paperpile listing status recorded | Listing status field populated for Paperpile (deferred/not-applicable per Track 59) | Docs review confirms status field | opt-in-live |
| JabRef listing status recorded | Listing status field populated for JabRef (deferred/not-applicable per Track 59) | Docs review confirms status field | opt-in-live |
| RefWorks listing status recorded | Listing status field populated for RefWorks (deferred/not-applicable per Track 59) | Docs review confirms status field | opt-in-live |
| Marketplace evidence ledger consistent | `conductor/evidence-ledger.json` or Track 69 evidence table matches listing status in docs | Cross-reference between docs and evidence ledger | default-CI |

## Documentation and claim boundary

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| `docs/src/citation-manager-integration.md` updated | Document contains per-manager sections with package format, distribution channel, listing status, and smoke gate references | File review against spec.md requirements | default-CI |
| `docs-site/src/content/docs/guides/citation-manager.md` created | Starlight guide page exists with citation-manager integration overview, listing status table, and install/smoke references | File existence check and content review | default-CI |
| No "marketplace-accepted" overclaims | All docs use "package-ready", "prepared", "deferred", or "not-applicable" — never "accepted" without listing evidence | grep for "accepted" in citation-manager docs confirms no false claims | default-CI |
| Release-status table updated | `docs/src/release-status.md` and docs-site mirror reflect per-manager listing status | Docs build and review | default-CI |
