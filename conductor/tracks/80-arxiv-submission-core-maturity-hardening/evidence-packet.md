# `arXiv/arxiv-submission-core` Evidence Packet

Date: 2026-05-18

## Local Contract

The legacy adapter maps submission-core style event/domain evidence into
`sourceright.journal_screening.v1`. It does not change legacy submission state,
call live credentials, write to arXiv systems, or mutate canonical CSL.

## Existing Evidence

- `fixtures/journal/arxiv-submission-core-submission.json`
- `fixtures/journal/arxiv-submission-core-variants.json`
- `schemas/sourceright.arxiv-submission-fixture.schema.json`
- `plugins/manifests/journal.arxiv-submission-core.toml`
- `tests/arxiv_platform_adapter_policy.rs`
- `tests/cli_end_to_end.rs`
- `docs/src/journal-integrations.md`

## Maintainer Draft

Title: Proposal: read-only reference-screening contract for `arxiv-submission-core`

Body:

Sourceright has a local, fixture-backed screening contract for legacy
`arxiv-submission-core` submission workflows. The integration maps legacy
submission evidence into a read-only `sourceright.journal_screening.v1` report.
It does not mutate submission-core state, require live credentials in default
CI, submit papers, or alter canonical CSL records.

Evidence included in the Sourceright repo:

- separate `journal.arxiv-submission-core` manifest;
- synthetic legacy-domain fixture;
- shared JSON schema for arXiv submission fixtures;
- CLI and policy tests covering platform registration, fixture parsing, schema
  docs, and no-overclaim boundaries;
- documentation separating `provider.arxiv` metadata evidence from submission
  platform screening.

Question for maintainers: would a read-only external integration contract be
useful for legacy workflows, or should this remain outside the archived/legacy
submission-core code path?

Rollback path: keep the integration external and avoid any change to legacy
domain models.

## Remaining before upstream submission

- Run maintainer-requested legacy tests once the upstream dependency contract is
  pinned (opt-in; Track 81 approval required).
- Migration mapping snapshot: `submission-core-contract-snapshot.json` (verified 2026-06-09).
- Security boundaries: `security-boundaries.md`.
