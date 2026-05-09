# Provider Strategy

Academic verification should be built as an ordered provider roadmap. Each provider adds candidates, provenance, and confidence signals; provider data must never silently overwrite the canonical reference record.

## Implementation Order

1. Crossref: first source for DOI metadata and bibliographic matching. Use it to establish the provider contract, normalized candidate shape, confidence scoring inputs, and provenance fields.
2. DOI resolution: validate DOI reachability and final landing-page targets separately from metadata lookup. Resolution confirms identifier usability; it does not by itself prove bibliographic correctness.
3. DataCite: add coverage for datasets, software, preprints, and non-journal scholarly outputs that Crossref may not describe well.
4. OpenAlex: add broad graph enrichment, including work identifiers, venues, authorship, concepts, and citation context. Treat OpenAlex as enrichment unless its evidence independently supports a verification decision.
5. PubMed/NCBI: add biomedical lookup and PMID/PMCID enrichment for health and life-science references. Use NCBI identifiers as additional provenance rather than replacements for DOI or canonical citation fields.
6. ORCID: add author identity enrichment after work-level matching is reliable. ORCID should link candidate authors to persistent person identifiers, not create work matches on its own.

## Fixture and Mocking Expectations

Provider work must be testable without live network access. Each provider should define fixture-backed responses for successful lookup, no-match, ambiguous-match, rate-limit, outage, malformed response, and conflicting metadata cases. Fixtures should preserve enough raw provider payload to verify parsing and provenance, while tests assert against normalized candidate output.

HTTP clients should be mockable behind the provider boundary. Unit tests should use local fixtures or explicit mock responses; live provider checks, if added later, should be opt-in and excluded from the default test path.

## Merge Rule

Provider results are candidates with provenance. They must not silently overwrite canonical references. Any proposed change to canonical fields must be represented as a sidecar candidate, conflict, or explicit merge decision with source attribution.
