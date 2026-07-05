# Track 78 — Test Matrix

## Requirements documentation — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| `arXiv/submit-ce` requirements documented | `requirements-matrix.md` contains Contribution docs, CI/CD config, Schema/contracts, Testing patterns, Security boundaries, and Submission process for `arXiv/submit-ce` | `requirements-matrix.md` — submit-ce section has findings in all 6 dimensions | Default-CI |
| `arXiv/arxiv-submission-core` requirements documented | `requirements-matrix.md` contains same 6 dimensions for `arXiv/arxiv-submission-core` | `requirements-matrix.md` — submission-core section has findings in all 6 dimensions | Default-CI |
| Active vs legacy status correctly noted | `arXiv/submit-ce` marked as active; `arXiv/arxiv-submission-core` marked as legacy/inactive per maintainer | `requirements-matrix.md` — status row for each repo | Default-CI |
| All 6 contract areas covered | CI, Schema, Testing, Security, Submission process, Contribution docs all present for both repos | Review of `requirements-matrix.md` headers and content for each dimension | Default-CI |
| Claim boundary present | `requirements-matrix.md` contains a claim boundary section stating "requirements-documented" not "arXiv-compatible" | File contains claim boundary text | Default-CI |
| URLs are valid GitHub paths | All referenced GitHub repo paths (`https://github.com/arXiv/submit-ce`, `https://github.com/arXiv/arxiv-submission-core`) are valid | URL inspection | Default-CI |
| Handoff links for downstream tracks | `requirements-matrix.md` references Track 79 (submit-ce maturity hardening) and Track 80 (submission-core maturity hardening) | Handoff section or link present in `requirements-matrix.md` | Default-CI |

## Spec integrity — default CI

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Goal references maturity, stability, testing | `spec.md` contains the words "maturity", "stability", and "testing" as keywords describing the track's purpose | `spec.md` review — Goal and Scope sections contain these terms | Default-CI |
| Plan forbids upstream submission | `plan.md` contains "Do not submit upstream" phrase and does not contain "acceptance gates pass" | `plan.md` review — Phase 3 has the prohibition | Default-CI |
| Evidence level target is `contracted` | `spec.md` Evidence level target section states `contracted` | `spec.md` section review | Default-CI |
| Claim boundary documented in spec | `spec.md` has a Claim boundary section with "requirements-documented" not "arXiv-compatible" | `spec.md` section review | Default-CI |
| Metadata dependencies listed | `metadata.json` has `dependencies` array with `72-submission-requirements-contracts` and `16-journal-workflow-integrations` | `metadata.json` readback | Default-CI |

## Handoff completeness — opt-in live

| Scenario | Acceptance | Evidence | CI mode |
|---|---|---|---|
| Track 79 implementer can read matrix and understand submit-ce requirements | Track 79 implementer confirms `requirements-matrix.md` provides enough detail to begin hardening | Track 79 evidence packet references `requirements-matrix.md` | opt-in-live |
| Track 80 implementer can read matrix and understand submission-core requirements | Track 80 implementer confirms `requirements-matrix.md` provides enough detail to begin migration hardening | Track 80 evidence packet references `requirements-matrix.md` | opt-in-live |
| Requirements matrix referenced in `arxiv-upstream.md` packet | `conductor/submission-packets/arxiv-upstream.md` contains reference to `conductor/tracks/78-arxiv-upstream-requirements-reconnaissance/requirements-matrix.md` | `arxiv-upstream.md` readback | opt-in-live |
