# Academic Providers Spec

## Goal

Verify and enrich academic references using reliable public citation APIs.

## Scope

- Crossref first, covering DOI metadata lookup and bibliographic query matching.
- DOI resolution second, covering DOI reachability, redirect behavior, and landing target capture.
- DataCite third, covering dataset, software, preprint, and other non-journal scholarly metadata.
- OpenAlex fourth, covering work graph enrichment, venues, authorship, concepts, citations, and alternate identifiers.
- PubMed/NCBI fifth, covering PMID/PMCID lookup and biomedical reference enrichment.
- ORCID sixth, covering author identity enrichment after work-level candidates are established.
- Provider clients behind mockable boundaries with fixture-based tests for success, no-match, ambiguous-match, rate-limit, outage, malformed response, and conflicting metadata cases.
- Normalized provider output that keeps provider identifier, raw-source reference, retrieved-at timestamp, candidate fields, confidence inputs, and provenance.

## Boundaries

Provider results create candidates and provenance; they do not silently overwrite canonical references.

Canonical reference fields may change only through an explicit merge decision recorded in the verification sidecar. Conflicting provider data must remain visible as conflict evidence until resolved.

Default tests must not require live provider access. Any live API checks must be opt-in, clearly marked, and independent of the normal local test path.
