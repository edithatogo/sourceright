# Track 83 - Implementation Plan

## Phase 1: Discover [ ]

- [ ] Inspect Janeway official docs for plugin hooks, package structure, and install flow.
- [ ] Identify what Sourceright should integrate through: plugin, sidecar, or hybrid bridge.
- [ ] Record any Janeway-specific constraints that affect command invocation, permissions, or report handoff.

## Phase 2: Lock spec [ ]

- [ ] Freeze the integration shapes and evidence requirements.
- [ ] Lock the claim boundary: Janeway is reconnaissance scoped, not supported or published.
- [ ] Confirm the docs update shape for `docs/src/journal-integrations.md` and the docs-site mirror.

## Phase 3: Map [ ]

- [ ] Update the journal-integrations docs with a Janeway section and a self-improving registry note.
- [ ] Capture the platform capability matrix in the test matrix.
- [ ] Keep the Janeway contract separate from the OJS plugin contract.

## Phase 4: Run checks [ ]

- [ ] Validate that the docs site mirror matches the source doc.
- [ ] Run targeted repo checks for the edited markdown and roadmap files.
- [ ] Verify the new track entries and evidence-ledger entries stay internally consistent.

## Phase 5: conductor-review [ ]

- [ ] Run `$conductor-review` against the Janeway reconnaissance track.
- [ ] Record any findings about platform scope, evidence wording, or boundary drift.

## Phase 6: Apply fixes [ ]

- [ ] Fix any wording, scope, or doc-parity issues found in review.
- [ ] Re-run the targeted checks after fixes.

## Phase 7: Progress [ ]

- [ ] Promote the Janeway integration contract into the packaging track only after the reconnaissance evidence is stable.
- [ ] Keep the claim boundary at reconnaissance until the packaging and smoke tracks pass.
