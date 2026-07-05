# Track 87 - Self-Improving Platform Registry

## Goal

Build the machine-readable registry, agent workflow, and skill hooks that keep Sourceright's platform roadmap current as new journals, APIs, and plugin surfaces appear.

## User outcome

The platform roadmap can update itself from verified evidence, propose new structured candidate tracks, and keep human review in the loop before any support claim moves forward.

## Scope

- Maintain `conductor/platform-registry.json` as the source of truth for platform evidence, capabilities, blockers, confidence, approval state, and candidate-track suggestions.
- Keep `conductor/platform-registry.schema.json` as the internal Conductor schema for the registry contract.
- Use `scripts/propose-platform-track-candidates.ps1` to validate the registry and emit candidate-track JSON without writing files, creating issues, committing, pushing, or opening tracks.
- Identify the skills or reusable workflows that should automate discovery, packaging, smoke checks, and review prompts.
- Keep the registry tied to human approval before any new track is opened or any claim is promoted.

## Out of scope

- Autonomous publication of support claims.
- Unreviewed registry edits.
- Replacing the conductor workflow with a hidden agent.
- Any self-registration path that bypasses evidence review.
- Auto-opening track directories, GitHub issues, pull requests, commits, or pushes from registry output.

## Data contracts

- **`conductor/platform-registry.json`** - internal registry of platform opportunities. Each entry must include platform metadata, evidence sources, capabilities, blockers, confidence, approval state, and a structured candidate track.
- **`conductor/platform-registry.schema.json`** - JSON Schema-style internal contract for the registry shape.
- **`scripts/propose-platform-track-candidates.ps1`** - validation and export script that emits `{ pass, candidate_track_count, errors, candidate_tracks }` or candidate tracks only with `-CandidatesOnly`.
- **`tests/platform_registry_policy.rs`** - policy test that ensures every platform has a structured candidate track and that output remains review-gated and non-mutating.

## Candidate-track contract

Every future platform opportunity must be represented as a structured `candidate_track` object before it can enter planning. Required fields are:

- `track_slug`
- `title`
- `proposed_status`
- `trigger`
- `scope`
- `acceptance_gates`
- `human_review_required`
- `auto_open`

The registry policy sets `ad_hoc_notes_allowed` to `false`, `candidate_track_required` to `true`, `human_review_required` to `true`, and `auto_open_tracks` to `false`. This means the registry can produce reviewable track candidates, but it cannot authorize platform support or mutate Conductor state.

## Initial consumers

- Track 83 Janeway reconnaissance supplies the first open-source journal-platform opportunity.
- Track 86 proprietary journal-platform matrix supplies ScholarOne, Editorial Manager, eJournalPress, and Manuscript Manager as contract-only candidate opportunities.

## Claim boundary

> The registry can suggest, but humans still approve.

The track is complete when the feedback loop is explicit and bounded, not when it auto-publishes platform support. Registry output is a candidate-track proposal, not a support announcement.

## Evidence level target

**contracted** - the registry is done enough when the schema, workflow, candidate-track output, policy test, and approval boundary are written and reviewable.

## Parallelization plan

- Registry schema design can happen alongside platform matrix updates.
- Agent and skill workflow design can be drafted from the same evidence model.
- Human review gates should be specified before any automation is wired in.
