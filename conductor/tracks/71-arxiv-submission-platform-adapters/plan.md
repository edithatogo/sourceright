# arXiv Submission Platform Adapters Plan

1. [x] Discover existing OJS, provider.arxiv, plugin registry, schema, and journal-screen contracts.
2. [x] Address dependencies early by updating shared platform enum, CLI/MCP parsing, journal-screening schema, and registry manifests before platform-specific fixtures.
3. [x] Implement the `submit-ce` lane with an owned manifest and synthetic fixture.
4. [x] Implement the `arxiv-submission-core` lane with an owned manifest and synthetic fixture.
5. [x] Add policy and end-to-end tests that prove both lanes reuse `sourceright.journal_screening.v1`.
6. [x] Document the provider/platform boundary and the parallelization model.
7. [x] Add and document the arXiv fixture schema so fixture `$schema` pointers resolve inside the repo.
8. [x] Add the Windows GNU local validation wrapper so MSVC/linker and OneDrive target-lock issues are handled by a documented command.
9. [x] Run targeted checks, complete a local `$conductor-review` pass, apply local review fixes automatically, and defer only live/upstream work.

## Validation

- [x] `cargo fmt --check`
- [x] `Get-Content schemas\sourceright.arxiv-submission-fixture.schema.json | ConvertFrom-Json`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test arxiv_platform_adapter_policy --target-dir C:\tmp\sourceright-target-track71`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test cli_end_to_end arxiv_platform_fixtures_screen_to_shared_journal_contract --target-dir C:\tmp\sourceright-target-track71`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test schema_contract_inventory --target-dir C:\tmp\sourceright-target-track71`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --test docs_site_parity --test requirements_contract_policy --target-dir C:\tmp\sourceright-target-track71`
- [x] `cargo +stable-x86_64-pc-windows-gnu clippy --all-targets --target-dir C:\tmp\sourceright-target-track71 -- -D warnings`
- [x] `cargo +stable-x86_64-pc-windows-gnu check --locked --target-dir C:\tmp\sourceright-target-track71`
- [x] `cargo +stable-x86_64-pc-windows-gnu test --target-dir C:\tmp\sourceright-target-track71`
- [x] `powershell -NoProfile -ExecutionPolicy Bypass -File scripts\verify-local-windows-gnu.ps1 -TargetDir C:\tmp\sourceright-target-local -SkipBench -SkipReportSmoke`
- [x] `cargo +stable-x86_64-pc-windows-gnu run --bin sourceright --target-dir C:\tmp\sourceright-target-track71 -- report --json examples\workspace`
- [x] `npm run build` from `docs-site/`
- [x] `powershell -NoProfile -ExecutionPolicy Bypass -File scripts\release-check.ps1 -ReleaseTag v0.1.20`
- [x] `git diff --check` (only Windows line-ending warning for `scripts/release-check.ps1`)
