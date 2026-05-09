# Legacy Audit

This audit covers the imported `legacy/humanizer-next` material listed in `docs/import-manifest.md`. The import is useful as provenance and fixture source material, but it should remain non-production until behavior is deliberately ported into the Rust implementation.

## Imported Surfaces

All manifest-listed surfaces are present under `legacy/humanizer-next`.

| Surface | Audit finding |
| --- | --- |
| `experiments/citation_ref_manager/` | Main legacy behavior source. Exposes citation-key extraction, CSL-JSON checks, manuscript/reference reconciliation, confidence scoring, CrossRef enrichment, URL/DOI checks, and CSL export conversions. |
| `skills/humanizer-cite/` | Stub CommonJS package. `CitationNormalizer.assess` reports `status: "stub"` and `fix` is a no-op. Its tests mostly cover formatting helpers, not citation verification. |
| Archived Conductor tracks | Useful planning/provenance only. They document earlier citation/source-verification intent, not production-ready behavior. |
| `docs/citation-manager-boundary.md` | Important boundary decision: citation manager was explicitly moved to `experiments/` and removed from the maintained Humanizer skill surface. |
| `docs/SOURCE_REFRESH_COMMANDS.md` | Reusable operational notes for source refresh, but commands assume the old repo layout and network access. |
| `scripts/research/citation-normalize.js` | Small JSON normalizer for non-CSL research citation records. Useful as schema inspiration, not as runtime code. |
| `src/references.json` | Best CSL-JSON fixture candidate. Contains varied academic, software, standards, report, dataset, and webpage records. Some `type` values, such as `software` and `standard`, are outside the legacy validator's own accepted CSL type list. |
| `src/research_references.md` and `src/ai_features_sources_table.md` | Human-readable source inventories. Useful for provenance examples and source-quality tests. |
| `test/sample-citations.json` | Compact non-CSL fixture with confidence/status fields. Useful for import-normalization tests. |
| `archive/sources_manifest.json` and `archive/sources/` | Source archive manifest and markdown source notes. The manifest includes one archived source with an empty-content SHA-256 hash and one pending source with blank hash/date fields, so it is not a clean verification baseline. |

## Reusable Fixture Candidates

| Candidate | Use |
| --- | --- |
| `legacy/humanizer-next/src/references.json` | Canonical positive fixture for parsing reference arrays, preserving CSL-like fields, and detecting unsupported/extended type values. |
| `legacy/humanizer-next/test/sample-citations.json` | Normalization fixture for legacy research-citation records with `authors`, `year`, `confidence`, `claimSummary`, `reasoningCategory`, and `status`. |
| Phase 6 sample records in `experiments/citation_ref_manager/phase6_test.js` | Small in-memory fixtures for missing citation, unused citation, YAML/RIS/BibLaTeX conversion, and URL/DOI verification scenarios. |
| `archive/sources_manifest.json` | Negative/edge fixture for source-manifest validation: pending records, blank hash/date fields, and suspicious empty-content hash handling. |
| `archive/sources/*.md` | Provenance text fixtures for source notes and later claim/source workflows. |
| `docs/citation-manager-boundary.md` | Boundary fixture for documentation tests that assert legacy citation-manager code is treated as experimental source material only. |

## Port, Replace, Discard Matrix

| Legacy surface or behavior | Decision | Rationale |
| --- | --- | --- |
| `findCitationKeysInManuscript` bracket-key extraction | Port with stronger parsing | The behavior is central, but the regex only recognizes simple `[key]` patterns and will over-match ordinary bracketed text. Rust should define supported citation syntaxes explicitly. |
| `verifyManuscriptCitations` missing/unused reconciliation | Port | This maps directly to Sourceright's reference workflow and should become deterministic Rust behavior. |
| CSL array validation and required-field checks | Replace | The legacy validator is hand-written, incomplete, and internally inconsistent with fixture data. Prefer a Rust schema/domain validator with explicit support for accepted CSL and Sourceright extension fields. |
| Duplicate and low-information checks in `validate_citations.js` | Port selectively | The checks are useful, but thresholds and severities should be product decisions rather than copied constants. |
| Confidence scoring | Replace | The heuristic is simple and undocumented. Keep the factors as examples, but implement confidence as transparent provider/provenance metadata. |
| CrossRef DOI enrichment | Replace | The idea is valid, but the JS implementation is single-provider, network-coupled, and lightly tested. Implement provider adapters with retries, provenance, caching, and review-state output. |
| URL/DOI accessibility checks | Port concept, replace implementation | Retain HEAD/DOI resolver verification as a capability, but make network behavior configurable and non-flaky in tests. |
| CSL-to-YAML/RIS/BibLaTeX/EndNote conversion | Replace or defer | Conversion code is useful for output expectations, but hand-rolled escaping and format mapping is risky. Prefer established crates or focused exporters with golden tests. |
| `CanonicalStorage` JSON load/save | Discard | Sourceright should own storage paths and file contracts directly; this JS class adds no durable design value. |
| `humanizeCitations` | Discard | It currently returns the original text and only warns about unsourced citations. |
| `experiments/citation_ref_manager/integration.js` | Discard | It references functions that are not imported in the module and should not be treated as runnable integration code. |
| `skills/humanizer-cite` | Discard as runtime, keep as provenance | The package is a stub and its formatting tests are unrelated to Sourceright's citation-verification core. |
| `scripts/research/citation-normalize.js` | Port fixture shape only | Useful for understanding legacy non-CSL records, but it mutates input files and uses a separate schema. |
| Archived Conductor tracks | Keep as provenance | Planning context only; do not port as implementation. |

## Immediate Follow-Up Recommendations

1. Create Rust fixture files from `src/references.json`, `test/sample-citations.json`, the Phase 6 inline sample records, and `archive/sources_manifest.json`.
2. Add deterministic tests for citation-key extraction, missing/unused citation reporting, duplicate IDs, unsupported/extended type handling, and manifest edge cases.
3. Define the Sourceright reference schema boundary: pure CSL fields, Sourceright extension fields, and validation severities.
4. Treat network verification and provider enrichment as later provider-adapter work with cached test responses, not as a direct JS port.
5. Keep legacy code out of runtime paths until a dedicated port track promotes specific behavior into Rust with tests.
