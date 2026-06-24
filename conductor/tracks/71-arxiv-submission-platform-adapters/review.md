# arXiv Submission Platform Adapters Review

## Result

Local review passed for the fixture-backed adapter-contract slice.

## Findings

- `provider.arxiv` remains separate from the two planned journal platform adapters.
- `journal.arxiv-submit-ce` and `journal.arxiv-submission-core` have separate manifests and registry entries.
- `JournalPlatform`, CLI parsing, MCP parsing, and the journal-screening schema all include both arXiv platform IDs.
- Synthetic fixtures cover the current `submit-ce` source-bundle lane and the legacy `arxiv-submission-core` event/domain lane.
- Fixture `$schema` values now resolve to `schemas/sourceright.arxiv-submission-fixture.schema.json`, which is documented in both docs surfaces.
- The new policy and end-to-end tests prove both lanes reuse `sourceright.journal_screening.v1`.
- `scripts/verify-local-windows-gnu.ps1` captures the local Windows workaround for MSVC/linker failures and locked default target directories.
- The Windows GNU verifier report smoke uses `examples/workspace`, exercising the documented parent-directory workspace resolution instead of requiring callers to pass `examples/workspace/.sourceright`.
- No upstream arXiv module acceptance, live credential use, API integration, platform writeback, or canonical CSL mutation is claimed.

## Deferred Work

- Live arXiv test-environment work remains opt-in and requires explicit credentials or a public test target.
- Upstream module or pull-request work remains deferred until the target arXiv repository contract is reviewed.
