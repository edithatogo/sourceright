# Track 87 - Implementation Plan

## Phase 1: Discover [x]

- [x] Identify the current track, registry, and evidence files that the platform workflow should read.
- [x] List the reusable discovery and validation steps that should become agent or skill building blocks.
- [x] Note where human approval must interrupt any automated suggestion.

## Phase 2: Design [x]

- [x] Define the registry entries for platform evidence, capabilities, blockers, and confidence.
- [x] Define the agent workflow that proposes track candidates from official sources and repo fixtures.
- [x] Define the skill/workflow hooks that can reuse the same contract across platform families.

## Phase 3: Lock [x]

- [x] Write the claim boundary so registry output cannot become an unsupported support claim.
- [x] Keep the self-improving loop readable in conductor docs and track notes.
- [x] Separate suggestion logic from approval logic.

## Phase 4: Implement [x]

- [x] Add `conductor/platform-registry.json` as the source of truth for platform opportunities.
- [x] Add `conductor/platform-registry.schema.json` for the internal registry contract.
- [x] Add `scripts/propose-platform-track-candidates.ps1` to emit structured candidate-track JSON without mutating the repo.
- [x] Add `tests/platform_registry_policy.rs` to lock structured candidates and human-review gates.

## Phase 5: Run checks [ ]

- [x] Validate the registry-facing docs and track metadata.
- [x] Confirm the evidence ledger still matches the new track boundaries.
- [x] Recheck that the machine-readable plan does not imply auto-publishing.
- [x] Run `scripts/propose-platform-track-candidates.ps1` and confirm `candidate_track_count` covers Janeway and the proprietary matrix platforms.
- [x] Run `cargo fmt --check` for the new policy test.
- [ ] Run `cargo test --locked --test platform_registry_policy` in an environment where the Windows target/linker can write build artifacts.

## Phase 6: conductor-review [ ]

- [ ] Review the registry design for runaway automation or missing approval gates.
- [ ] Tighten wording where the agent role could be read as autonomous support publication.

## Phase 7: Apply fixes [ ]

- [ ] Fix any schema, workflow, or boundary issues found in review.
- [ ] Re-run the checks after the fixes.

## Phase 8: Progress [x]

- [x] Turn the registry into the source of truth for future platform-track suggestions.
- [x] Keep human review as the last gate before a new platform track is opened.
