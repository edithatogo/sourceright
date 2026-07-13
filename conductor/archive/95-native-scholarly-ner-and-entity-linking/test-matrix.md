# Track 95 Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Vocabulary mapping | Original label, class, and mapping relation are preserved. | Mapping tests. | Default-CI |
| Span preservation | Entity offsets and text remain source-grounded. | Fixture tests. | Default-CI |
| Linking separation | Link candidates carry registry/version/query/method/score and do not rewrite mentions. | Unit test. | Default-CI |
| Input safety | Inputs above 25 MiB and empty patterns are rejected before matching. | Unit test. | Default-CI |
| Ledger validation | Source, license, access, redistribution, domain, language, and split fields are present. | JSON policy test. | Default-CI |
| Citation independence | Entity module has no CSL/provider dependency and existing reference path remains unchanged. | Module boundary/policy tests. | Default-CI |
| Domain packs | General fixture does not imply biomedical/legal coverage. | Manifest and docs. | Default-CI |
| GROBID-NER audit | Java/API/version, 27-class scope, bibliography exclusion, and mixed data access are recorded. | Audit ledger. | Default-CI |
| Native NER metrics | Span/class metrics and calibration are measured on independent cohorts before claims. | Deferred benchmark. | Opt-in |
| Compatibility bridge | Any bridge is separately versioned, pinned, and disabled by default. | Deferred decision. | Opt-in |
