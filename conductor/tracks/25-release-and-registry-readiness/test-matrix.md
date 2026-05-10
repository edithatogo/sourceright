# Release And Registry Readiness Test Matrix

| Scenario | Expected result |
| --- | --- |
| Package metadata | Cargo metadata includes repository, homepage, docs, license, keywords, categories, and MSRV. |
| Package contents | Crate package excludes legacy/provenance-only material and includes files needed by runtime tests. |
| Package validation | `cargo package --locked` passes from a clean tree. |
| Publish dry run | `cargo publish --dry-run --locked` passes before real publication. |
| GitHub release | Tagged release builds platform binaries and checksums. |
| No accidental publish | Real crates.io publication is manual and environment-gated. |
