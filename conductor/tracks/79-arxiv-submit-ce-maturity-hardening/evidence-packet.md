# `arXiv/submit-ce` Evidence Packet

Date: 2026-05-18

## Local Contract

The current adapter accepts synthetic or exported `submit-ce` submission
metadata and emits `sourceright.journal_screening.v1`. It does not submit
papers, call arXiv credentials, write to `submit-ce`, or mutate canonical CSL.

## Existing Evidence

- `fixtures/journal/arxiv-submit-ce-submission.json`
- `fixtures/journal/arxiv-submit-ce-variants.json`
- `schemas/sourceright.arxiv-submission-fixture.schema.json`
- `plugins/manifests/journal.arxiv-submit-ce.toml`
- `tests/arxiv_platform_adapter_policy.rs`
- `tests/cli_end_to_end.rs`
- `docs/src/journal-integrations.md`

## Maintainer Draft

Title: Proposal: reference-screening integration contract for `submit-ce`

Body:

Sourceright has a local, fixture-backed journal-screening contract for arXiv
`submit-ce` submissions. The integration is read-only: it produces a
`sourceright.journal_screening.v1` screening report from submission metadata and
does not submit papers, mutate arXiv state, require live credentials, or alter
canonical CSL records.

Evidence included in the Sourceright repo:

- separate `journal.arxiv-submit-ce` manifest;
- synthetic source-bundle fixture;
- JSON schema for arXiv submission fixtures;
- CLI and policy tests covering platform registration, fixture parsing, schema
  docs, and no-overclaim boundaries;
- documentation separating `provider.arxiv` metadata evidence from submission
  platform screening.

Question for maintainers: would you prefer this to remain an external
integration contract, or should we prepare a small upstream module/PR that maps
`submit-ce` submission metadata into a read-only screening hook?

Rollback path: keep the integration external and remove any proposed upstream
hook without affecting arXiv submission state.

## Remaining before upstream submission

- Run any maintainer-requested `submit-ce` local test command once the upstream
  branch and dependency requirements are pinned (opt-in; Track 81 approval required).
- Schema drift snapshot: `submit-ce-contract-snapshot.json` (verified 2026-06-09).
- Security boundaries: `security-boundaries.md`.
