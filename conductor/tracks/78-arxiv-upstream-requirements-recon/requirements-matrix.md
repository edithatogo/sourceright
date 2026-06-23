# arXiv Upstream Requirements Matrix

> **Claim boundary**: This matrix documents upstream repository requirements for
> reference. It does **not** claim that Sourceright's adapter is compatible
> with, accepted by, or reviewed by arXiv maintainers.
>
> Evidence level: **contracted** — all findings are derived from reference-able
> GitHub repository files, not from live API calls or maintainer interviews.

## Repo status overview

| Property | `arXiv/submit-ce` | `arXiv/arxiv-submission-core` |
|---|---|---|
| **Status** | ✅ **Active** — current intake surface | ⛔ **Legacy/Inactive** — per maintainer @dginev on issue #88 |
| **URL** | <https://github.com/arXiv/submit-ce> | <https://github.com/arXiv/arxiv-submission-core> |
| **Description** | arXiv paper submission system | arXiv-NG submission system (predecessor) |
| **Last release** | Active development (97 tags, frequent commits) | v0.6.1 (Jan 30, 2019) |
| **Stars / Forks** | 4 / 0 | 14 / 6 |
| **License** | MIT (2024 arXiv) | MIT (2017 Cornell University Library) |
| **Code owners** | `@jweiskoff @bdc34 @bmaltzan @DavidLFielding @norbusan` | Not specified in CODEOWNERS |
| **Open issues** | 1 | 8 |
| **Open PRs** | 5 | 6 |

---

## 1. Contribution docs

### `arXiv/submit-ce` (active)

| Requirement | Finding |
|---|---|
| CONTRIBUTING.md | Not present in repo root |
| Code of conduct | Present (referenced in sidebar) |
| PR requirements | Not formally documented; inferred from PR history |
| Commit message format | Not specified |
| Branch naming | Not specified |
| Review process | CODEOWNERS file defines reviewers; at least one review expected |
| DCO / CLA | Not documented |

**Notes**: submit-ce relies on GitHub's default PR workflow. CODEOWNERS
(`* @jweiskoff @bdc34 @bmaltzan @DavidLFielding @norbusan`) routes review
requests automatically. No formal CONTRIBUTING.md was found in the develop
branch.

### `arXiv/arxiv-submission-core` (legacy/inactive)

| Requirement | Finding |
|---|---|
| CONTRIBUTING.md | ✅ Present — detailed contributing guide |
| Code of conduct | Present (referenced in sidebar) |
| PR requirements | ✅ Documented: at least one code review required, all tests must pass before merge |
| Commit message format | ✅ Specified: short message with ticket reference (e.g. `ARXIVNG-42`) |
| Branch naming | ✅ Gitflow model: `story/ARXIVNG-108`, `task/issue-51` |
| Review process | ✅ Documented: PR → code review → merge by author; significant changes require separate PR |
| DCO / CLA | Not documented |

**Notes**: submission-core has a thorough CONTRIBUTING.md following the Gitflow
branching model. Uses JIRA tickets with `ARXIVNG-` prefix. Semantic versioning
per subsystem.

---

## 2. CI/CD config

### `arXiv/submit-ce` (active)

| Requirement | Finding |
|---|---|
| CI provider | GitHub Actions (`.github/workflows/` directory present) |
| Python version | 3.11 (via `uv:python3.11-bookworm` Docker image) |
| Build system | `uv` with `pyproject.toml` and `uv.lock` |
| Linting | Ruff (`lint.sh`: `uv run ruff check submit_ce`), Flake8 (`.flake8` with line length 88) |
| Testing | Pytest (`test.sh`: `uv run pytest --cov=submit_ce submit_ce/api submit_ce/implementations submit_ce/domain submit_ce/ui`) |
| Coverage | 70% minimum (`fail_under = 70` in `pyproject.toml`) |
| Coverage service | Coveralls |
| Docker build | Multi-stage Dockerfile: `builder` → `with-dev-venv` → `run-tests` → `production` |
| Deploy target | Google Container Registry (`gcr.io/arxiv-development/submit-ce/submit-ce-ui`) |
| Dependabot | Not detected |
| Pre-commit | Not detected |

**Notes**: `test.sh` excludes `submit_ce/implementations/pubsub` from coverage.
Dockerfile has a commented-out pubsub emulator test stage. Production image
uses `gunicorn` on port 8080.

### `arXiv/arxiv-submission-core` (legacy/inactive)

| Requirement | Finding |
|---|---|
| CI provider | Travis CI (`.travis.yml`) |
| Python version | 3.6 |
| Build system | Pipenv (`Pipfile` + `Pipfile.lock`) |
| Linting | Pylint (`.pylintrc`, target score >=8/10), pydocstyle (`tests/docstyle.sh`), mypy (`mypy.ini`) |
| Testing | Nose2 (`unittest.cfg`), pytest for agent/core |
| Coverage | >=90% target (`.coveragerc`) |
| Coverage service | Coveralls |
| Docker build | Docker Compose (`docker-compose.yml`) with MariaDB, metadata API, gateway |
| Deploy target | PyPI (`deploy` section in `.travis.yml`) |
| Integration tests | `WITH_INTEGRATION=1` env var for Docker Compose-based integration tests |
| Type checking | mypy with `mypy.ini` config |

**Notes**: Travis CI runs `nstest.py` + pytest with coverage. Deploys to PyPI
on tags. Uses `pipenv sync --dev` for dependency installation. The CI pipeline
is designed for the arXiv-NG microservices architecture with Docker Compose.

---

## 3. Schema / contracts

### `arXiv/submit-ce` (active)

| Requirement | Finding |
|---|---|
| API schema | ✅ OpenAPI 3.1.0 (`openapi.json`) — 15+ endpoints |
| API endpoints | `/v1/start`, `/v1/user_submissions`, `/v1/submission/{id}`, `/v1/submission/{id}/acceptPolicy`, `/v1/submission/{id}/setLicense`, `/v1/submission/{id}/assertAuthorship`, `/v1/submission/{id}/files`, `/v1/submission/{id}/setCategories`, `/v1/submission/{id}/setMetadata`, `/v1/submission/{id}/markDeposited`, `/v1/submission/{id}/markProcessingForDeposit`, `/v1/submission/{id}/unmarkProcessingForDeposit`, `/v1/status` |
| Auth | OAuth2PasswordBearer (OAuth2 with password flow) |
| Data models | Submission, SubmissionContent, SubmissionMetadata, Author, Category, License, SetMetadata, SetCategories, Agent, etc. |
| Validation | Pydantic v2 (pydantic>=2.9 in dependencies) |
| JSON Schema | `jsonschema==4.23.0` in dependencies |
| DB ORM | SQLAlchemy 2.0 |
| Category catalog | Comprehensive arXiv category enum (astro-ph, cond-mat, cs.*, math.*, physics.*, q-bio, q-fin, stat.*, etc.) |
| Submission statuses | `working`, `submitted`, `scheduled`, `announced`, `deleted`, `error`, `withdrawn` |

### `arXiv/arxiv-submission-core` (legacy/inactive)

| Requirement | Finding |
|---|---|
| API schema | REST API with JSON submission object (documented in README) |
| API endpoints | `POST /submission/`, `POST /submission/{id}/` (update), token endpoint |
| Auth | OAuth2 client credentials flow (client ID + secret → access token) |
| Data models | Submission (with metadata, classifications, source_content, compilations, etc.) |
| DB | MariaDB (via SQLAlchemy) |
| Validation | jsonschema==2.6.0 |
| Event model | Kinesis-based event system for submission state changes |
| Submission statuses | `working` (default), plus event-driven lifecycle |
| Category catalog | 150 categories loaded by default |
| Example fixture | `metadata/examples/complete_submission.json` |


---

## 4. Testing patterns

### `arXiv/submit-ce` (active)

| Requirement | Finding |
|---|---|
| Test framework | Pytest 8.3.3 |
| Coverage tool | pytest-cov 5.0.0 |
| Coverage threshold | 70% minimum (`fail_under = 70`) |
| Test directories | `submit_ce/api/`, `submit_ce/implementations/`, `submit_ce/domain/`, `submit_ce/ui/` |
| Property-based testing | Hypothesis >=6.131.21 + hypothesis-jsonschema >=0.23.1 |
| Factory/generation | Polyfactory >=2.22.0 + mimesis 18.0.0 |
| Mocking | pytest-mock >=3.14.0 |
| Dev dependencies | 40+ dev packages including orjson, paramiko, rapidfuzz, watchdog |
| Test filter warnings | Configured in `pyproject.toml` (pydantic deprecation, arxiv-base CSRF, fire/pipes) |
| Docker test stage | `run-tests` stage in Dockerfile runs pytest (excluding pubsub) |

### `arXiv/arxiv-submission-core` (legacy/inactive)

| Requirement | Finding |
|---|---|
| Test framework | nose2 (primary), pytest (secondary) |
| Coverage tool | coverage 4.5 |
| Coverage target | >=90% |
| Test config | `unittest.cfg` (nose2) |
| Test commands | `nose2 --with-coverage`, `pytest --cov=agent/agent --cov=core/arxiv` |
| Lint checks | `lint.sh`, `docstyle.sh`, `static.sh` (static analysis) |
| Type checking | mypy 0.720 with `mypy.ini` |
| Integration testing | Docker Compose-based (`docker-compose.yml`, `start_sim.sh`) |
| Test structure | `tests/` directory + `nstest.py` (smoke test) |
| Dev dependencies | sphinx, sphinx-autodoc-typehints, docker, openapi-spec-validator |

---

## 5. Security boundaries

### `arXiv/submit-ce` (active)

| Requirement | Finding |
|---|---|
| Auth mechanism | OAuth2PasswordBearer (password flow) |
| Token management | PyJWT 2.9.0 |
| Secret management | Google Cloud Secret Manager |
| Encryption | cryptography 43.0.1 |
| Security policy file | Not detected in repo root |
| Input validation | Pydantic v2 models, jsonschema validation |
| Upload handling | `python-multipart`, file upload via multipart/form-data |
| Dependencies pinned | Yes — most deps pinned to exact versions in `pyproject.toml` |
| Docker security | Non-root `e-prints` user in Docker image |
| CI/CD secrets | Not visible (GitHub secrets) |

### `arXiv/arxiv-submission-core` (legacy/inactive)

| Requirement | Finding |
|---|---|
| Auth mechanism | OAuth2 client credentials flow |
| Token management | PyJWT 1.6.4 |
| Client registry | OAuth2 client registry service (`arxiv/registry`) |
| Encryption | cryptography (via arxiv-base dependency chain) |
| Security policy file | Not detected |
| Input validation | jsonschema 2.6.0 |
| SQL injection protection | SQLAlchemy ORM |
| Dependency scanning | Not detected |
| Docker Compose security | Services on custom Docker network, gateway on port 8000 |

---

## 6. Submission process

### `arXiv/submit-ce` (active)

| Requirement | Finding |
|---|---|
| Submission API | RESTful — `POST /v1/start` to begin, then sequential steps |
| Submission types | `new`, `replacement`, `withdrawal`, `cross`, `jref` |
| Required steps | 1. Start (get submission ID), 2. Accept policy, 3. Assert authorship, 4. Set license, 5. Upload files, 6. Set categories, 7. Set metadata, 8. Confirm preview |
| File upload | Single file, zip, or tar.gz (auto-unpacked) |
| Source formats | `tex`, `pdftex`, `ps`, `html`, `pdf` |
| Local dev | `local_dev.py` — runs on localhost:8000 with test auth token |
| Test DB bootstrap | `python submit_ce/make_test_db.py bootstrap_db` |
| Production deploy | Docker → GCR → gunicorn on port 8080 |
| Configuration | Environment variables, Google Cloud services (PubSub, Secret Manager) |
| Browser testing | ModHeader extension for local auth token injection |

### `arXiv/arxiv-submission-core` (legacy/inactive)

| Requirement | Finding |
|---|---|
| Submission API | RESTful — `POST /submission/` to create, `POST /submission/{id}/` to update |
| Required fields | Minimal (empty `{}` creates a working submission) |
| Finalization | `POST /submission/{id}/` with `finalized: true` requires all mandatory fields |
| Local dev | Docker Compose — `docker-compose build` + `docker-compose up` |
| Dependencies | MariaDB, Redis, multiple microservices (metadata, gateway, registry) |
| Client setup | `docker run arxiv/registry:0.1 create_client.py` for OAuth2 credentials |
| Token acquisition | `POST /api/token` with client credentials grant |
| Submission lifecycle | `working` → `submitted` → `scheduled` → `announced` (event-driven via Kinesis) |
| Docs site | <https://arxiv.github.io/arxiv-submission-core/> (Sphinx docs) |
| Example | `metadata/examples/complete_submission.json` |



---

## Handoff for downstream tracks

| Downstream track | Purpose | Key source sections |
|---|---|---|
| **[Track 79](../79-arxiv-submit-ce-maturity-hardening/)** — submit-ce maturity hardening | Raise submit-ce adapter to upstream-ready maturity, stability, and testing evidence | Sections 1–6 for `arXiv/submit-ce` |
| **[Track 80](../80-arxiv-submission-core-maturity-hardening/)** — submission-core maturity hardening | Raise legacy submission-core adapter to migration-safe maturity, stability, and testing evidence | Sections 1–6 for `arXiv/arxiv-submission-core` |
| **[Track 81](../81-arxiv-upstream-submission-and-acceptance/)** — upstream submission & acceptance | File issues, monitor responses, record acceptance | Both repos' submission process + contact surfaces |

> **Note**: `arXiv/submit-ce` is the **active** intake surface. All hardening
> effort should prioritise submit-ce. `arXiv/arxiv-submission-core` is
> **legacy/inactive** per maintainer @dginev on issue #88; Track 80 covers
> migration hardening only.

---

## Claim boundary

> **"requirements-documented" not "arXiv-compatible".** This matrix documents
> what each upstream repository requires for contribution, CI, schema conformance,
> testing, security, and submission. It does **not** claim that Sourceright's
> adapter meets these requirements. Compatibility verification and acceptance
> are the responsibility of Tracks 79/80 (hardening) and Track 81 (submission).
>
> **Evidence level**: `contracted` — all claims are derived from reference-able
> GitHub files at the URLs listed above. No live API calls, credentials, or
> upstream acceptance evidence were used.
