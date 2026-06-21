# Track 91 - Implementation Plan

## Phase 1: Source requirements [ ]

- [ ] Accept target journal and article type as inputs.
- [ ] Retrieve or ingest official author instructions and policies.
- [ ] Capture source provenance, retrieval date, and source category.
- [ ] Flag stale, conflicting, incomplete, or unofficial sources.

## Phase 2: Create contract [ ]

- [ ] Convert sourced requirements into `sourceright.journal_article_requirements.v1`.
- [ ] Assign category, machine-checkable status, assessment method, and default MoSCoW priority.
- [ ] Require human approval before using the contract as a gate.

## Phase 3: Assess submission [ ]

- [ ] Evaluate the research package against each requirement.
- [ ] Produce pass, fail, partial, not applicable, or unknown status.
- [ ] Attach evidence spans or package nodes to each finding.

## Phase 4: Recommend and plan [ ]

- [ ] Produce MoSCoW recommendations.
- [ ] Separate author tasks from automated dry-run actions.
- [ ] Generate a writeback plan without applying changes.

## Phase 5: Optional implementation [ ]

- [ ] Apply only approved, reversible changes.
- [ ] Log all changes, tool versions, and approvals.
- [ ] Reassess after implementation and compare before/after state.

## Phase 6: Integration [ ]

- [ ] Expose CLI and MCP surfaces.
- [ ] Add fixtures for at least one synthetic journal and one real-policy example when permitted.
- [ ] Add docs for authors, editors, and platform adapters.
