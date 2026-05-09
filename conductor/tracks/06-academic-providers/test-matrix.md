# Academic Providers Test Matrix

| Scenario | Expected result |
| --- | --- |
| Crossref DOI lookup | Exact provider candidate is recorded with DOI metadata, confidence inputs, and Crossref provenance. |
| Crossref bibliographic lookup | Initial fixture-backed work normalization records ranked-candidate-ready confidence and source detail; live query adapter remains pending. |
| DOI resolution success | DOI reachability, redirect chain, and final target are recorded without replacing canonical citation fields. |
| DOI resolution failure | Deterministic error metadata is recorded as validation evidence. |
| DataCite lookup | Pending provider adapter reusing `AcademicProviderResult`. |
| OpenAlex enrichment | Pending provider adapter reusing `AcademicProviderResult`. |
| PubMed/NCBI lookup | Pending provider adapter reusing `AcademicProviderResult`. |
| ORCID enrichment | Pending provider adapter reusing `AcademicProviderResult`. |
| Provider no-match | A no-match result is recorded deterministically without creating an empty canonical update. |
| Ambiguous provider match | Multiple candidates are preserved with ranking/confidence instead of choosing silently. |
| Provider rate-limit or outage | Deterministic retry/error metadata is recorded from fixture-backed tests. |
| Malformed provider response | Parser failure is surfaced as provider error evidence and does not alter canonical data. |
| Conflicting provider data | Conflict evidence is recorded and no silent overwrite occurs. |
| Fixture-backed unit test | Provider behavior is validated from local fixtures or explicit mocks without live network access. |
