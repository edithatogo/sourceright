## Summary

## Track

Conductor track:

## Validation

- [ ] `cargo fmt --check`
- [ ] `cargo clippy --all-targets -- -D warnings`
- [ ] `cargo test`
- [ ] `cargo check --locked`
- [ ] Relevant docs-site build or schema/demo validation was run, if touched.
- [ ] `$conductor-review` was run for the relevant track or an explicit reason is recorded.

## Data and provenance

- [ ] CSL JSON remains clean.
- [ ] Verification/provenance metadata is in the sidecar boundary.
- [ ] Fixtures contain no private or proprietary data.

## Contract and claims

- [ ] The change maps to `conductor/requirements.md` and a Conductor track.
- [ ] Public wording does not claim production readiness, final verification, AI detection, legal compliance, or registry/platform support without evidence.
- [ ] Release-surface wording changes ran `scripts/verify-release-surface-refresh.ps1` and kept accepted/prepared/deferred states aligned.
- [ ] Plugin/module changes preserve the no-submodule default unless a separate release lifecycle is documented.

## Governance

- [ ] Branch protection requirements (`CI`, `Security`, `Pages`, release checks) are preserved or a settings-task note is recorded.
- [ ] Code scanning (CodeQL, Scorecard, Dependabot) and cargo/npm audit gaps are addressed or tracked.
- [ ] Renovate handles dependency updates; no Dependabot PR config was added.
- [ ] Coverage decision is documented: `cargo llvm-cov` summary-only (no Codecov/Coveralls service).
- [ ] Evidence ledger entry for the owning track is updated with accurate allowed_claims and evidence_level.
