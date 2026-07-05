# Track 84 - Implementation Plan

## Phase 1: Discover [ ]

- [ ] Confirm the Janeway package format and extension entry points discovered in Track 83.
- [ ] Map the Sourceright command boundary into the host package without embedding screening logic in the host.
- [ ] Identify the minimum package metadata needed to install and preview the adapter.

## Phase 2: Lock spec [ ]

- [ ] Freeze the adapter skeleton contract, preview-first behavior, and permission boundary.
- [ ] Define the fixture-backed test cases that prove the package shape.
- [ ] Lock the claim boundary: packageable, not deployed.

## Phase 3: Implement [ ]

- [ ] Create the Janeway adapter skeleton or package scaffold if the extension model is stable enough.
- [ ] Keep the actual screening logic in the Rust core and call it through a thin boundary.
- [ ] Add any read-only metadata or docs needed for local validation.

## Phase 4: Run checks [ ]

- [ ] Run package structure checks and targeted linting.
- [ ] Run the fixture-backed adapter tests.
- [ ] Verify that the package skeleton still preserves the claim boundary.

## Phase 5: conductor-review [ ]

- [ ] Run `$conductor-review` on the Janeway package contract.
- [ ] Record any findings about host permissions, packaging shape, or boundary drift.

## Phase 6: Apply fixes [ ]

- [ ] Fix any packaging or contract issues uncovered in review.
- [ ] Re-run the targeted checks after the fix set lands.

## Phase 7: Progress [ ]

- [ ] Promote to install smoke only after the package contract and checks are stable.
- [ ] Keep the Janeway package track separate from live-instance evidence.
