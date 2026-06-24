# Citation-Manager Publication Decision

Date: 2026-05-18

## Zotero

Sourceright ships Zotero support as the Sourceright CLI/Web API adapter, not as
a Zotero `.xpi` browser or desktop plugin.

Evidence:

- `conductor/tracks/58-mature-zotero-plugin/packaging-decision.md`
- `docs/src/zotero-plugin-install.md`
- `.github/workflows/zotero-live-smoke.yml`
- `.github/workflows/zotero-desktop-smoke.yml`

Package channel:

- GitHub Releases or crates.io for the Sourceright binary/crate.
- Zotero plugin directory is not a target unless a future track creates a real
  `.xpi` package and Zotero UI workflow.

## EndNote

Sourceright ships EndNote support as file handoff through deterministic ENW and
RIS exports, not as an EndNote plugin.

Evidence:

- `src/export.rs`
- `fixtures/export/endnote-export.enw`
- `fixtures/export/endnote-export.ris`
- unit tests `enw_export_matches_endnote_handoff_fixture` and
  `ris_export_matches_endnote_handoff_fixture`
- `docs/src/exports.md`
- `docs/src/citation-manager-integrations.md`
- `conductor/tracks/59-other-citation-manager-integrations/review.md`

## Claim Boundary

Allowed wording: "Zotero CLI/Web API adapter" and "EndNote ENW/RIS file
handoff".

Disallowed wording until separate evidence exists: "Zotero plugin accepted",
"EndNote plugin", "Zotero `.xpi`", or "EndNote live sync".
