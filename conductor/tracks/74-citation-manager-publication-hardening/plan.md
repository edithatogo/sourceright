# Plan

## Phases

### 1. [ ] Discover
- Audit current Zotero plugin state:
  - Review `conductor/tracks/58-mature-zotero-plugin/packaging-decision.md` for
    current packaging decision and revalidation date.
  - Check `plugins/manifests/citation-manager.zotero.toml` for manifest status.
  - Check `docs/src/zotero-plugin-install.md` for install smoke proof evidence.
  - Check `docs/src/citation-manager-integrations.md` for Zotero section currency.
  - Verify `.xpi` decision is still current (no new demand signals or Zotero API
    deprecations).
  - Record revalidation finding: decision stands or revisit required.
- Audit EndNote handoff proof:
  - Check `plugins/manifests/citation-manager.endnote.toml` for manifest.
  - Run `sourceright export --format enw` against `examples/csl-examples.csla.json`
    or representative fixture.
  - Verify generated ENW parses with a structural validator or known parser.
- Audit RIS export status:
  - Check `docs/src/exports.md` RIS section for current spec.
  - Run `sourceright export --format ris` against representative fixture.
  - Verify generated RIS parses with a structural validator.
- Check marketplace evidence from Track 69 for citation-manager entries.
- Check submission requirements from Track 72 for citation-manager hosts.

### 2. [ ] Lock spec
- Document package format per citation manager:
  - Zotero: CLI/Web API binary (no `.xpi`) — confirm or revise.
  - EndNote: ENW/RIS file handoff (no live API adapter).
- Document distribution channel per manager:
  - Zotero: GitHub Releases (binary) + crates.io (crate).
  - EndNote: File export only (no package distribution).
- Document smoke gates per manager:
  - Zotero: `sourceright citation-sync --preview --remote-fixture <fixture>` must
    exit 0 and produce valid report JSON.
  - EndNote: `sourceright export --format enw` must produce valid ENW that
    roundtrips through a parser.
  - RIS: `sourceright export --format ris` must produce valid RIS that
    roundtrips through a parser.
- Record claim boundary: "package-ready" not "marketplace-accepted".
- Lock data contracts, parallelization plan, and evidence targets in `spec.md`.
- Share locked spec with Subagents A-D.

### 3. [ ] Implement
- **Subagent A — Zotero packaging decision audit**:
  - Write revalidation finding to `plugins/zotero/packaging-revalidation-<date>.md`.
  - If `.xpi` is still deferred, record finding: "Deferred per Track 58 —
    CLI/Web API binary remains the intended package."
  - If `.xpi` is now required (e.g., Zotero API deprecation), produce `.xpi`
    build script and install smoke evidence.
  - Verify install smoke: `sourceright citation-sync --preview --remote-fixture
    fixtures/providers/zotero/preview-exact-match.json` exits 0 with valid JSON.
- **Subagent B — EndNote ENW handoff validation**:
  - Add or verify `tests/export_roundtrip_enw.rs` with structural validation
    that re-parses generated ENW and checks record count + key identifiers.
  - Run against `fixtures/export/*.json` or `examples/csl-examples.csla.json`.
  - Record results in `test-matrix.md`.
- **Subagent C — RIS import/export roundtrip**:
  - Add or verify `tests/export_roundtrip_ris.rs` with structural validation
    that re-parses generated RIS and checks record count, DOI/URL preservation,
    and author counts.
  - Run against representative fixtures.
  - Record results in `test-matrix.md`.
- **Subagent D — Distribution notes and listing probes**:
  - Update `docs/src/citation-manager-integration.md` with per-manager:
    - Package format and distribution channel.
    - Marketplace listing status (accepted/prepared/deferred/not-applicable).
    - Smoke gate evidence references.
  - Create `docs-site/src/content/docs/guides/citation-manager.md` as a
    Starlight guide page (mirror or summary).
  - Record listing probe results:
    - Zotero Plugin Gallery: not-applicable (no `.xpi`).
    - Zotero Forums: deferred (no browser plugin to announce).
    - EndNote: not-applicable (file-format only, no live adapter).
    - Other citation-manager directories: deferred or not-applicable per Track 59.

### 4. [ ] Run checks
- **Import/export roundtrip tests**:
  ```bash
  cargo test export_roundtrip_enw
  cargo test export_roundtrip_ris
  cargo test export_roundtrip_biblatex
  ```
  All pass against `fixtures/export/` and `examples/csl-examples.csla.json`.
- **Zotero install smoke** (fixture-backed):
  ```bash
  cargo test citation_sync -- --nocapture
  ```
  Verifies `sourceright citation-sync --preview --remote-fixture` produces valid
  report JSON without live Zotero credentials.
- **Structural validation tests**:
  ```bash
  cargo test --test export_schema_contract
  ```
  Verifies generated exports match schema contracts.
- **CLI smoke**:
  ```bash
  sourceright export --format ris --preview .sourceright
  sourceright export --format enw --preview .sourceright
  ```
  Both exit 0 and print valid `sourceright.export_manifest.v1` JSON.

### 5. [ ] conductor-review
- Run `$conductor-review` gate:
  - Check all `owned_paths` are present and consistent.
  - Verify packaging decision revalidation is documented.
  - Verify ENW and RIS roundtrip tests exist and pass.
  - Verify distribution notes are updated in `docs/src/` and `docs-site/`.
  - Verify marketplace listing probes are recorded.
  - Verify claim boundary is enforced in all documentation (no "marketplace-accepted"
    claims without listing evidence).
  - Record review finding in `conductor/tracks/74-citation-manager-publication-hardening/review.md`.

### 6. [ ] Apply fixes
- Apply any findings from `$conductor-review`:
  - Missing test coverage — add roundtrip or smoke tests.
  - Outdated docs — update `docs/src/citation-manager-integration.md` and
    `docs-site/src/content/docs/guides/citation-manager.md`.
  - Claim boundary violations — reword any "accepted" or "listed" language to
    "package-ready" or "prepared."
  - Missing packaging revalidation — write the finding document.
  - Missing listing probe — run probe and record result.
- Re-run affected tests after fixes.

### 7. [ ] Progress — Advance to listing evidence only after review passes
- After `$conductor-review` passes and all fixes are applied:
  - Update `conductor/evidence-ledger.json` with citation-manager hardening
    evidence entries.
  - Update `docs/src/release-status.md` and docs-site mirror to reflect
    per-manager listing status.
  - Mark citation-manager entries in the marketplace evidence table as
    `package_hardened` or equivalent.
  - Record track completion metadata.
  - Only promote any manager from `prepared` to `accepted` if a public listing
    URL, version, date, and install metadata are verified in the evidence ledger.
