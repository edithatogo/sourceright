# Track 68 — LibreOffice Extension Packaging Review

## Status

Completed as explicit deferral with contract.

## Evidence

| Area | Result |
| --- | --- |
| Current repo assets | ODT/DOCX file processing is separate; no `.oxt` or UNO package exists. |
| Package decision | `packaging-decision.md` records no `.oxt`, UNO bridge, install smoke, or LibreOffice Extensions listing. |
| Future contract | Extension must reuse CLI/MCP/UNO-command-adapter outputs and must not reimplement verification logic. |
| Writer range provenance | Paragraph, footnote/endnote, table-cell, text-frame, and bibliography anchors are specified. |
| Write safety | Writer edits must be previewed, reversible, audited, and explicit on apply. |
| Marketplace boundary | LibreOffice Extensions acceptance is deferred to Track 69 evidence. |

## Remaining External Work

- Create a `.oxt` or UNO/Python bridge only after range provenance and
  reversible write plans are stable.
- Add local install/uninstall smoke against a disposable LibreOffice profile.
- Add fixture documents covering paragraphs, footnotes/endnotes, tables, text
  frames, and bibliography ranges.
- Record LibreOffice Extensions listing evidence only after acceptance.

## Review Outcome

Track 68 should not claim LibreOffice extension support from ODT/DOCX file
processing. The repo now has the explicit deferral and future contract needed
to prevent overclaiming while preserving the implementation path.
