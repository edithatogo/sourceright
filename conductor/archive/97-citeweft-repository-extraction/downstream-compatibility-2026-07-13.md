# Downstream compatibility evidence

- Sourceright source commit: `17f090e`
- CiteWeft extraction commit: `f4a859e`
- CiteWeft governance commit: `351cb28`
- Compatibility script: `scripts/verify-citeweft-compatibility.ps1`
- Neutral module hashes: all five matched the published CiteWeft checkout.
- Candidate package checksum: `827c1cebe0a2fb00a8cf567b1287f9a3756cab4d36b6b27561af013a271eaa7e`.
- Packaged-crate consumer smoke: passed from the verified `citeweft-0.1.0`
  package directory.
- Sourceright extraction verifier: passed for five core modules.
- Sourceright CI for `17f090e`: core, security, docs, Pages, and robustness
  checks passed.

Sourceright now consumes the CiteWeft candidate through an exact Git revision
in `Cargo.toml`, recorded in commit `ad1d675`; the duplicated local neutral
modules were removed. The disposable consumer smoke and full Sourceright Rust
gates pass after migration. Rollback remains available by reverting the
dependency/wiring commit and is still tracked as an explicit release gate.
