# External Proof Suites Plan

1. Create a proof-suite inventory and environment-variable contract.
2. Add deterministic fixture-backed tests first.
3. Add opt-in live smoke scripts with clear skip messages.
4. Add GitHub Actions manual workflows only when they are quiet and credential
   gated.
5. Update docs to separate fixture-backed, opt-in live, and unproven claims.
6. Run targeted checks for each proof family.
7. Run `$conductor-review`.
8. Apply review fixes automatically and progress proof families independently.
