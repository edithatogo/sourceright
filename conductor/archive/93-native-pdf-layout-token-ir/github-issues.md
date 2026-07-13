# GitHub Issue Ledger

## Issue #38

Track 93 native PDF layout and token IR. This implementation establishes the
stable backend-neutral layout contract and fixture adapter. Native PDF parser
selection remains gated on Track 92 cohort evidence plus license, security,
malformed-input, resource-limit, and cross-platform checks.

Candidate inventory: `conductor/third-party/native-pdf-layout.json`. No parser
dependency is selected or presented as supported by this contract slice.

Review fixes applied: PDF-looking input is rejected by the fixture adapter,
token source IDs include page identity, and serialization/overlap ambiguity
tests now cover the documented contract.
