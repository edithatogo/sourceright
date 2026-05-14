# Track 62 Review — Expanded Normaliser Provider Catalogue

## Summary

Track 62 assessed and documented 25 normaliser provider candidates across all
major scholarly domains. The expanded catalogue is published in
`docs/src/providers.md` as a single reference table with column-level metadata
(access model, status, track ownership, evidence level, fixtures, default-CI
behaviour, and overclaim guard). A cross-reference was added to
`docs/src/plugin-registry.md`.

## Test Matrix Results

| Scenario | Acceptance | Result |
|---|---|---|
| **Catalogue** | Candidate provider table covers scholarly, economics, grey literature, repositories, and search. | ✅ Table covers 25 providers across all required domains. |
| **Access model** | Each provider has public/licensed/search/deferred status. | ✅ Every row includes explicit access model and status field. |
| **Google Scholar** | No scraping or unsupported automation added; compliant path documented or deferred. | ✅ ADR 0005 documented. Permanently prohibited. No manifest, no fixtures, no CI path. |
| **Economics** | RePEc/SSRN/NBER/EconLit decisions are recorded. | ✅ Economics Decision Log documents deferral rationale for all five candidates. |
| **Registry** | New supported candidates have manifests and tests. | ✅ Existing manifests cover all `planned` and `core_normalizer` providers. Deferred providers explicitly documented as assessment-only with no manifest. |

## Files Modified

| File | Change |
|---|---|
| `docs/src/providers.md` | Added Expanded Provider Catalogue table (25 rows), Economics Decision Log, Google Scholar Decision Log (ADR 0005), Grey Literature/Repository Notes, Biomedical Preprint Notes. |
| `docs/src/plugin-registry.md` | Added cross-reference section pointing to the expanded catalogue. |
| `conductor/tracks/62-expanded-normaliser-provider-catalogue/metadata.json` | Status changed from `planned` to `completed`. |
| `conductor/tracks/62-expanded-normaliser-provider-catalogue/plan.md` | Added completion note with key decisions summary. |
| `conductor/evidence-ledger.json` | Added fixture-backed entry for track 62. |

## Files NOT Modified

- `plugins/manifests/*` — unchanged (owned by tracks 48, 49, 50).
- `.github/` — unchanged.
- `src/` — unchanged.

## Overclaim Guards

Every row in the catalogue includes an explicit overclaim guard string that
prevents unsupported market-readiness claims. Deferred providers (economics,
institutional repositories, clinical trial registries) additionally include
explicit "not supported" language in their guard column. Google Scholar is
marked as `deferred — prohibited` and includes the ADR 0005 reference.

## Open Items

None. This track is purely documentation and assessment — no implementation
work is expected from this catalogue effort. Implementation of `planned`
providers is owned by tracks 48, 49, 50, and 58.
