# Track 78 — arXiv Upstream Requirements Reconnaissance Spec

## Goal

Search and document the contribution, schema, testing, security, and submission
requirements of `arXiv/submit-ce` and `arXiv/arxiv-submission-core` so that
Tracks 79 and 80 have a documented requirements baseline for **maturity**
hardening, **stability** evaluation, and **testing** coverage before any
upstream submission is proposed.

## User outcome

Track 79 (submit-ce maturity hardening) and Track 80 (submission-core maturity
hardening) implementers can read `requirements-matrix.md` and immediately
understand:

- Which repo is active vs legacy, and where to file issues.
- What CI/CD pipeline, code owners, license, and contribution guidelines each
  repo expects.
- What schema/contract surfaces exist (OpenAPI, PyPI packages, Docker images).
- What testing patterns (pytest, nose2, coverage thresholds, lint tools) are
  used and what quality gates are expected.
- What security boundaries (OAuth2, authentication/authorization, secret
  management) are in place.
- What the submission process looks like (API endpoints, Docker Compose dev
  environment, deployment targets).

## Scope

- **`arXiv/submit-ce` inspect**: Contribution docs (CODEOWNERS, CONTRIBUTING),
  CI config (pytest, ruff, Docker multi-stage build), schema contracts
  (openapi.json with 15+ endpoints), testing patterns (pytest with coverage,
  hypothesis, polyfactory), security boundaries (OAuth2PasswordBearer, Google
  Cloud Secret Manager), and submission process (REST API, uv/uv.lock build
  system, Docker/GCR deployment).
- **`arXiv/arxiv-submission-core` inspect**: Contribution guidelines (Gitflow
  branching, PEP8, NumPy docstrings), CI config (Travis CI with Docker
  Compose, nose2, coveralls), schema contracts (submission API types, MariaDB
  schema, OpenAPI spec), testing patterns (nose2 with coverage >=90%, pylint
  score >=8/10, mypy static checks), security boundaries (OAuth2/JWT client
  credentials flow), and submission process (Pipenv/Python 3.6, Docker Compose
  local dev, PyPI deployment).
- **`requirements-matrix.md`**: A structured table comparing both repos across
  six requirement dimensions — Contribution docs, CI/CD config, Schema/contracts,
  Testing patterns, Security boundaries, Submission process.
- **Handoff readiness**: Documented findings ready for Track 79 and Track 80
  implementers and referenced in `conductor/submission-packets/arxiv-upstream.md`.

## Out of scope

- Implementation work or code changes in Sourceright's arXiv adapter.
- Live arXiv API access, credentials, or account setup.
- Opening issues or PRs on arXiv repositories (handled by Track 81).
- Accepting or rejecting arXiv upstream PRs.
- Operational monitoring of arXiv infrastructure.
- Claiming arXiv compatibility or certification.

## Data contracts

| Contract | Source | Format | Track 78 contribution |
|---|---|---|---|
| `arXiv/submit-ce` repo root | `https://github.com/arXiv/submit-ce` | GitHub repository | Inspected for contribution/CI/schema/security/process docs |
| `arXiv/arxiv-submission-core` repo root | `https://github.com/arXiv/arxiv-submission-core` | GitHub repository | Inspected for contribution/CI/schema/security/process docs |
| Requirements matrix | `conductor/tracks/78-arxiv-upstream-requirements-reconnaissance/requirements-matrix.md` | Markdown table | Structured comparison across 6 requirement dimensions |
| Handoff note | `conductor/submission-packets/arxiv-upstream.md` | Markdown | References requirements-matrix.md and downstream tracks |

## Claim boundary

**"requirements-documented" not "arXiv-compatible".** This track claims that
the upstream requirements of `arXiv/submit-ce` and `arXiv/arxiv-submission-core`
have been inspected and documented in `requirements-matrix.md`. It does not
claim that Sourceright's adapter is compatible with, accepted by, or reviewed
by arXiv maintainers. Compatibility and acceptance are separate concerns for
Tracks 79/80 (hardening) and Track 81 (submission).

All evidence docs must include the disclaimer: *fixture-backed, not arXiv-reviewed*.

## Evidence level target

**contracted** — All evidence is derived from reference-able GitHub repository
files (README, CONTRIBUTING, CI config files, OpenAPI schemas, Dockerfiles,
pyproject.toml, Pipfile, .travis.yml, etc.). No live arXiv API calls, no
credentials, and no upstream acceptance evidence are required.

Track 81 is responsible for elevating from `contracted` to `opt-in-live-proven`
by recording upstream issue/PR submission and maintainer response.

## Parallelization plan

- **Subagent A**: Inspect `arXiv/submit-ce` repo — contribution, CI, schema,
  testing, security, process. Document findings for the "submit-ce" rows in
  `requirements-matrix.md`.
- **Subagent B**: Inspect `arXiv/arxiv-submission-core` repo — contribution, CI,
  schema, testing, security, process. Document findings for the
  "submission-core" rows in `requirements-matrix.md`.
- **Subagent C**: Assemble final `requirements-matrix.md` from Subagent A + B
  findings, add contract dimension headers, handoff links, and claim boundary
  section.

Subagents A and B can run in parallel (independent repos). Subagent C must
wait for both A and B to finish.
