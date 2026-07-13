# Test Matrix

| Area | Required evidence |
| --- | --- |
| Fixture governance | Every fixture has original input, expected semantic assertions, source revision, license, and provenance. |
| Canonical boundary | Oracle output cannot write canonical CSL or verification sidecars. |
| Determinism | Repeated comparisons emit byte-identical JSON reports. |
| BibLaTeX round trip | Names, dates, macros, casing, inheritance, identifiers, and unknown-field loss are classified. |
| RIS round trip | Supported fields survive or produce explicit loss diagnostics. |
| Parser safety | Malformed and oversized inputs fail within declared resource limits without panic. |
| Citation.js | Pinned runner produces normalized CSL comparisons from checked-in inputs. |
| biblatex-csl-converter | Loss-aware mapping is compared through an isolated optional runner. |
| Better BibTeX parser | Protected casing, TeX markup, macros, and malformed input have differential cases. |
| JabRef/JabKit | Optional disposable import/export smoke has retained artifacts and version evidence. |
| citeproc-js | License-cleared rendering cases compare normalized bibliography output. |
| Legal separation | CSL-M observations never force legal records into academic CSL. |
| CI security | External jobs have pinned dependencies, no secrets, bounded time/output, and temporary write scope. |
| Core gates | `cargo fmt --check`, clippy, tests, and `cargo check --locked` remain independently runnable. |
