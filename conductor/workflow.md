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

0. Public repo infrastructure.
1. Product roadmap.
2. Legacy workflow audit.
3. Reference intake.
4. CSL canonical model.
5. Verification sidecar.
6. Academic providers.
7. Standardisation and cleaning.
8. Conflict resolution.
9. In-text citation reconciliation.
10. Manual agent review.
11. Export suite.
12. CLI and MCP.
13. Legal citations.
14. Claim/source/provenance roadmap.
15. Reference reporting.
16. Journal workflow integrations.

## Validation expectations

- Every extracted reference fixture needs an original input and expected CSL JSON.
- Every provider integration needs mocked or recorded responses and deterministic confidence output.
- CSL JSON stays clean; verification state lives in sidecar metadata.
- Every public command or MCP tool emits deterministic JSON when requested.
- Every export format has generation tests and reparse or structural checks where practical.
- Manual review queues preserve original extracted text, provider candidates, diffs, confidence, and decisions.
- Journal screening reports produce editor-facing summaries and author-facing checklists without asserting AI generation.
- Legal citation support is modelled separately from academic CSL where required.
