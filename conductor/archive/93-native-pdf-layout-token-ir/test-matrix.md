# Track 93 Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Layout serialization | Versioned JSON round-trips without CSL fields. | Rust tests and fixture. | Default-CI |
| Page-relative boxes | Token boxes remain within their page dimensions. | Policy test. | Default-CI |
| Reading order | Blocks sort deterministically by page, column, vertical, then horizontal position. | Rust test. | Default-CI |
| Ambiguous layout | Overlapping block bands emit an ambiguity diagnostic. | Rust test. | Default-CI |
| No-text scan | Empty text input reports OCR-required and emits no synthetic token. | Rust test. | Default-CI |
| Resource limits | Oversized input/page/token counts fail before unbounded allocation. | Rust test. | Default-CI |
| Backend boundary | Fixture adapter is not presented as a PDF backend or universal parser. | Docs/spec policy. | Default-CI |
| Candidate selection | Parser license/API/security evidence is recorded before dependency selection. | Conductor plan. | Opt-in |
