# Academic Providers Plan

1. Define the provider contract and normalized candidate response model using Crossref as the first implementation target. Implemented through `AcademicProviderResult` and sidecar `ProviderCandidate` output.
2. Implement Crossref lookup by DOI and bibliographic query, including provenance fields, confidence inputs, and fixture-backed parsing tests. Initial fixture-backed Crossref work normalization is implemented; live lookup remains adapter work.
3. Add DOI resolution as a separate validation provider that records DOI reachability, redirect target, and resolution errors without treating reachability as bibliographic truth. Implemented as DOI resolution evidence records.
4. Add DataCite for datasets, software, preprints, and other scholarly records that are weakly covered by Crossref.
5. Add OpenAlex for graph enrichment, work identifiers, venue context, authorship context, and citation metadata.
6. Add PubMed/NCBI for PMID/PMCID enrichment and biomedical reference verification.
7. Add ORCID for author identity enrichment only after work-level matching is stable.
8. Record all provider matches, conflicts, raw-source references, and merge decisions in the verification sidecar.
9. Enforce the no-silent-overwrite rule: provider data can create candidates and explicit merge proposals, but it must not replace canonical reference fields without a recorded decision.
