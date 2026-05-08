# Sourceright Workflow

## Operating model

Use Conductor tracks for all substantial work. Each track defines the user-facing capability, source material being relied on, implementation boundaries, data contracts, and validation gates before code changes begin.

## Track structure

Each track lives under `conductor/tracks/NN-name/` and contains:

- `spec.md`: user goal, interfaces, data contracts, and boundaries.
- `plan.md`: implementation sequence and migration notes.
- `test-matrix.md`: acceptance scenarios, fixtures, and validation gates.
- `metadata.json`: status, priority, owners, dependencies, and owned paths.

## Roadmap tracks

1. Public repo infrastructure.
2. Product roadmap.
3. Legacy workflow audit.
4. Reference intake.
5. CSL canonical model.
6. Verification sidecar.
7. Academic providers.
8. Standardisation and cleaning.
9. Conflict resolution.
10. In-text citation reconciliation.
11. Manual agent review.
12. Export suite.
13. CLI and MCP.
14. Legal citations.
15. Claim/source/provenance roadmap.

## Validation expectations

- Every extracted reference fixture needs an original input and expected CSL JSON.
- Every provider integration needs mocked or recorded responses and deterministic confidence output.
- CSL JSON stays clean; verification state lives in sidecar metadata.
- Every public command or MCP tool emits deterministic JSON when requested.
- Every export format has generation tests and reparse or structural checks where practical.
- Manual review queues preserve original extracted text, provider candidates, diffs, confidence, and decisions.
- Legal citation support is modelled separately from academic CSL where required.
