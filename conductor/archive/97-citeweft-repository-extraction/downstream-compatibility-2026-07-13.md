# Downstream compatibility evidence

- Sourceright source commit: `17f090e`
- CiteWeft extraction commit: `f4a859e`
- CiteWeft governance commit: `351cb28`
- Compatibility script: `scripts/verify-citeweft-compatibility.ps1`
- Neutral module hashes: all five matched the published CiteWeft checkout.
- Sourceright extraction verifier: passed for five core modules.
- Sourceright CI for `17f090e`: core, security, docs, Pages, and robustness
  checks passed.

This is source compatibility evidence only. Sourceright has not yet consumed
an immutable packaged CiteWeft crate, so downstream dependency migration
remains gated.
