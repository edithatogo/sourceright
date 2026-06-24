# Citation-Manager Submission Packet

## Surfaces

- Zotero
- EndNote

## Requirements Evidence

| Surface | Source | Retrieved | Local impact |
| --- | --- | --- | --- |
| Zotero | <https://www.zotero.org/support/dev/client_coding/plugin_development> and <https://www.zotero.org/support/dev/zotero_7_for_developers> | 2026-05-18 | Zotero 7 plugin work needs a real plugin package decision; current Sourceright support remains CLI/Web API adapter until `.xpi` packaging exists. |
| EndNote | <https://docs.endnote.com/docs/endnote/2025/macos/v1/content/08import/importing_ref_data_intoen.htm> and <https://docs.endnote.com/docs/endnote/2025/v1/windows/en/content/08import/importing_refs_othbibsftwr.htm> | 2026-05-18 | EndNote supports reference import through correctly formatted data files such as RIS; Sourceright should harden ENW/RIS handoff rather than claim an EndNote plugin. |

## Local Gates

- Zotero package decision: adapter package, `.xpi`, or both.
- Disposable Zotero library smoke for preview/apply/audit.
- EndNote ENW/RIS reparse and import guide.
- Release notes that distinguish adapter/file handoff from plugin listing.

## Blockers

None.

## Local package evidence (Track 74)

| Surface | Evidence file | Verified |
| --- | --- | --- |
| Zotero | `conductor/tracks/74-citation-manager-publication-hardening/zotero-adapter-hardening-2026-06-09.md` | 2026-06-09 |
| EndNote | `conductor/tracks/74-citation-manager-publication-hardening/endnote-reparse-verification-2026-06-09.md` | 2026-06-09 |

Package decisions: `conductor/tracks/74-citation-manager-publication-hardening/publication-decision-2026-05-18.md`.

## Draft Submission Body

Local package decisions are complete. Zotero is a CLI/Web API adapter shipped
through the Sourceright binary, not a `.xpi`; EndNote is ENW/RIS file handoff,
not an EndNote plugin. External announcements or package submissions still need
approval and must use that claim boundary.

## Approval Gate

No Zotero or EndNote external listing, forum post, or package submission without
explicit approval.
