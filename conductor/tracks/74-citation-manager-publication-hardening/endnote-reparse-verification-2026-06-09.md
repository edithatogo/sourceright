# EndNote ENW/RIS Reparse Verification — Local Evidence

Date: 2026-06-09  
Version: 0.1.20

## Package decision

EndNote support is **ENW/RIS file handoff**, not an EndNote plugin.
Source: `publication-decision-2026-05-18.md`,
`conductor/tracks/59-other-citation-manager-integrations/review.md`.

## Manifest and fixtures

| Artifact | Role |
| --- | --- |
| `plugins/manifests/citation-manager.endnote.toml` | File-export manifest (`local_only`) |
| `fixtures/export/endnote-export.enw` | Golden ENW handoff fixture |
| `fixtures/export/endnote-export.ris` | Golden RIS handoff fixture |
| `docs/src/exports.md` | ENW/RIS mapping and structural validation notes |
| `docs/src/citation-manager-integrations.md` | EndNote file-handoff boundary |

## Default-CI checks (2026-06-09)

```text
cargo test endnote_handoff --lib
  export::tests::enw_export_matches_endnote_handoff_fixture ... ok
  export::tests::ris_export_matches_endnote_handoff_fixture ... ok
```

Target dir: `C:\tmp\sourceright-target-track74` (GNU toolchain, locked deps).

## Reparse scope

Golden fixtures prove deterministic export against canonical CSL input. RIS
structural validation in `export.rs` reparses blocks for tag counts, identifiers,
and author/editor preservation per `docs/src/exports.md`.

Full EndNote desktop import round-trip is **not** claimed in default CI. Manual
import verification in EndNote remains optional operator evidence.

## Claim boundary

This evidence supports hardened local ENW/RIS handoff. It does **not** claim an
EndNote plugin, live sync, or EndNote marketplace listing.
