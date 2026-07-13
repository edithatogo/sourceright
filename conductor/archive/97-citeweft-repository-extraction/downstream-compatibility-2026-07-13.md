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

This is source compatibility evidence only. Sourceright has not yet consumed
the package as a committed Cargo dependency, so downstream dependency
migration remains gated. The disposable consumer smoke proves the candidate
crate can compile and execute outside its source repository.
