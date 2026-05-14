# Microsoft Word Add-In Packaging Test Matrix

| Area | Required check | Evidence |
| --- | --- | --- |
| Packaging | Office Add-in manifest/taskpane exists, or the track records an explicit deferral. | Explicit deferral recorded in `packaging-decision.md`; no Office Add-in or AppSource listing is claimed. |
| Sideload | Local sideload instructions and manifest validation are documented before AppSource claims. | Sideload is deferred until a manifest/taskpane package exists. |
| Range mapping | Paragraph, footnote/endnote, table-cell, and bibliography anchors map to diagnostics. | Future range-provenance table is documented in `packaging-decision.md`. |
| Write safety | In-document edits are reversible, previewed, audited, and explicit on apply. | Contract requires dry-run-first visible change plans, audit logs, explicit apply, and reversible plans. |
| Privacy | Document handling and local/remote boundaries are documented. | Contract keeps adapter thin over local CLI/MCP/service-wrapper outputs. |
| Review | `$conductor-review` checks that DOCX extraction is not described as Word add-in support. | `tests/word_addin_packaging_policy.rs` enforces the claim boundary. |
