# arXiv Submission Platform Adapters Test Matrix

| Scenario | Acceptance | Evidence | Default-CI or opt-in-live |
| --- | --- | --- | --- |
| Shared dependency contract | `JournalPlatform`, CLI, MCP, schema, and registry know both arXiv platform IDs. | Rust unit tests and policy tests. | Default CI |
| `submit-ce` fixture lane | Synthetic current-platform fixture screens through `journal-screen` with `platform = arxiv_submit_ce`. | `tests/cli_end_to_end.rs`. | Default CI |
| Legacy platform fixture lane | Synthetic legacy fixture screens through `journal-screen` with `platform = arxiv_submission_core`. | `tests/cli_end_to_end.rs`. | Default CI |
| Fixture schema | arXiv fixture `$schema` values resolve to a repo schema documented in both docs surfaces. | `schemas/sourceright.arxiv-submission-fixture.schema.json`, schema inventory tests, and `tests/arxiv_platform_adapter_policy.rs`. | Default CI |
| Provider/platform boundary | Docs and policy distinguish `provider.arxiv` from arXiv submission-platform adapters. | `tests/arxiv_platform_adapter_policy.rs`. | Default CI |
| Parallelization guard | Track spec assigns shared contracts to one lane and platform fixtures/manifests to independent lanes. | This track spec and policy test. | Default CI |
| Upstream/live integration | No upstream arXiv patch or live arXiv write is claimed. | Claim boundary docs and evidence ledger blockers. | Opt-in live/future |
| Local review | Completed track has review notes for fixture scope, boundaries, validation, and deferred live work. | `conductor/tracks/71-arxiv-submission-platform-adapters/review.md`. | Default CI |
