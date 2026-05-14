# Microsoft Word Add-In Packaging Test Matrix

| Area | Required check |
| --- | --- |
| Packaging | Office Add-in manifest/taskpane exists, or the track records an explicit deferral. |
| Sideload | Local sideload instructions and manifest validation are documented before AppSource claims. |
| Range mapping | Paragraph, footnote/endnote, table-cell, and bibliography anchors map to diagnostics. |
| Write safety | In-document edits are reversible, previewed, audited, and explicit on apply. |
| Privacy | Document handling and local/remote boundaries are documented. |
| Review | `$conductor-review` checks that DOCX extraction is not described as Word add-in support. |
