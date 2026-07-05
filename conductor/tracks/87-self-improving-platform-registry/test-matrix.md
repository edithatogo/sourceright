# Track 87 - Test Matrix

| Area | Check | Expected |
| --- | --- | --- |
| registry schema | platform evidence fields exist | `conductor/platform-registry.json` and `conductor/platform-registry.schema.json` represent capabilities, blockers, confidence, approval state, and structured candidate tracks |
| workflow | suggestion path is explicit | `scripts/propose-platform-track-candidates.ps1` emits candidate-track JSON without writing files, commits, issues, or track directories |
| approval gate | human review remains required | Registry policy sets `human_review_required: true`, `auto_open_tracks: false`, and each candidate has `auto_open: false` |
| no ad hoc notes | platform opportunities are structured | Every platform entry contains a `candidate_track` object with required fields before planning can proceed |
| consumers | first consumers are named | Janeway and the vendor matrix are identified as initial inputs |
| docs | machine-readable and human-readable outputs align | Track 87 spec, plan, metadata, and policy test all reference the registry and candidate generator |
| boundary | no autonomous support claim | The track never implies the registry can self-authorize support |
