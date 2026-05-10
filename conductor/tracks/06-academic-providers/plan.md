# Academic Providers Plan

1. Define the provider contract and normalized candidate response model using Crossref as the first implementation target. Implemented through `AcademicProviderResult` and sidecar `ProviderCandidate` output.
2. Implement Crossref lookup by DOI and bibliographic query, including provenance fields, confidence inputs, and fixture-backed parsing tests. Initial fixture-backed Crossref work normalization is implemented; live lookup remains adapter work.
3. Add DOI resolution as a separate validation provider that records DOI reachability, redirect target, and resolution errors without treating reachability as bibliographic truth. Implemented as DOI resolution evidence records.
4. Add DataCite for datasets, software, preprints, and other scholarly records that are weakly covered by Crossref. Implemented as fixture-backed candidate normalization.
5. Add OpenAlex for graph enrichment, work identifiers, venue context, authorship context, and citation metadata. Implemented as fixture-backed candidate normalization.
6. Add PubMed/NCBI for PMID/PMCID enrichment and biomedical reference verification. Implemented as fixture-backed record normalization.
7. Add ORCID for author identity enrichment only after work-level matching is stable. Implemented as identity-only candidate evidence with conservative confidence.
8. Record all provider matches, conflicts, raw-source references, and merge decisions in the verification sidecar.
9. Enforce the no-silent-overwrite rule: provider data can create candidates and explicit merge proposals, but it must not replace canonical reference fields without a recorded decision.
