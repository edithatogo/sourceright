# Track 87 - Self-Improving Platform Registry

## Goal

Build the machine-readable registry, agent workflow, and skill hooks that keep Sourceright's platform roadmap current as new journals, APIs, plugin surfaces, and future-paper workflow requirements appear.

## User outcome

The platform roadmap can update itself from verified evidence, propose new track candidates, and keep human review in the loop before any support claim moves forward.

Track 88 supplies the strategic context for future scientific papers and agentic editorial workflows. Track 87 should use that context when deciding which platform evidence, workflow stages, and adapter capabilities need to be kept current.

## Scope

- Define the registry shape for platform evidence, capabilities, blockers, and confidence.
- Map the agent workflow that can discover new platforms, re-scan old ones, and propose deltas.
- Identify the skills or reusable workflows that should automate discovery, packaging, smoke checks, and review prompts.
- Keep the registry tied to human approval before any new track is opened or any claim is promoted.
- Include future-paper workflow capabilities such as structured submission packages, agentic triage, reviewer briefing, executable-object support, review provenance, and post-publication monitoring.

## Out of scope

- Autonomous publication of support claims.
- Unreviewed registry edits.
- Replacing the conductor workflow with a hidden agent.
- Any self-registration path that bypasses evidence review.
- Autonomous editorial decisions about submissions, reviews, or clinical use.

## Data contracts

- The platform registry schema used by conductor tracks and evidence notes.
- The Janeway and proprietary platform matrix tracks as the first consumers of the registry.
- Track 88 as the research context for future-paper and agentic workflow capability fields.
- Any future agent or skill manifests that encode repeatable discovery and validation steps.

## Claim boundary

> The registry can suggest, but humans still approve.

The track is complete when the feedback loop is explicit and bounded, not when it auto-publishes platform support or editorial decisions.

## Evidence level target

**contracted** - the registry is done enough when the schema, workflow, and approval boundary are written and reviewable.

## Parallelization plan

- Registry schema design can happen alongside platform matrix updates.
- Agent and skill workflow design can be drafted from the same evidence model.
- Future-paper workflow capability mapping can happen alongside Track 88 review.
- Human review gates should be specified before any automation is wired in.
