# Live Core Provider Verification Plan

1. Define opt-in environment/config controls for live core-provider queries.
2. Add or harden clients for Crossref, DataCite, OpenAlex, PubMed/NCBI, and DOI
   resolution.
3. Add deterministic response caching and recorded provider fixtures.
4. Normalize provider results into sidecar-only candidates with confidence and
   retrieval metadata.
5. Add rate-limit, malformed-response, no-match, and conflict tests.
6. Document how live verification changes the audit confidence level.

## Completion Signal

An opt-in live verification run can enrich a real CSL file with provider
evidence for core academic sources while default tests remain fixture-backed and
network-free.

## Progress Notes

- 2026-05-12: First deterministic evidence-quality slice landed. Fixture-backed
  Crossref, DataCite, OpenAlex, and PubMed normalizers now classify low
  confidence candidate evidence as `ambiguous` instead of `match`.
- 2026-05-12: Second evidence-quality slice landed. Provider assessment now
  distinguishes conflict-only payloads as `no_match` while keeping the raw
  provider payload as sidecar evidence, and mixed match/conflict payloads remain
  `ambiguous`.
- 2026-05-12: Third diagnostic slice landed. Provider results now expose
  deterministic diagnostic kinds for fixture-backed `no_match`,
  `ambiguous_match`, `malformed_response`, and outage-style failures while
  preserving sidecar-only candidate handling.
- 2026-05-13: Consolidation pass updated provider docs to describe the
  diagnostic fields and their sidecar-only evidence boundary.
