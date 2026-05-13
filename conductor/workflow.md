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

The complete track map is maintained in `conductor/tracks.md`. Keep this file
focused on process and validation expectations so the roadmap is not duplicated
across multiple Conductor surfaces.

The canonical feature requirements and repo contract are maintained in
`docs/src/feature-contract-matrix.md`, with architecture rationale in
`docs/src/design.md`. Track specs should cite those documents instead of
redefining global product boundaries.

## Validation expectations

- Every extracted reference fixture needs an original input and expected CSL JSON.
- Every provider integration needs mocked or recorded responses and deterministic confidence output.
- CSL JSON stays clean; verification state lives in sidecar metadata.
- Every public command or MCP tool emits deterministic JSON when requested.
- Every export format has generation tests and reparse or structural checks where practical.
- Manual review queues preserve original extracted text, provider candidates, diffs, confidence, and decisions.
- Journal screening reports produce editor-facing summaries and author-facing checklists without asserting AI generation.
- Legal citation support is modelled separately from academic CSL where required.
