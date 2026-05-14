# LibreOffice Extension Packaging Test Matrix

| Area | Required check | Evidence |
| --- | --- | --- |
| Packaging | `.oxt` packaging exists, or the track records an explicit deferral. | Explicit deferral recorded in `packaging-decision.md`; no `.oxt` or LibreOffice Extensions listing is claimed. |
| Adapter | UNO/Python or command adapter calls the Rust core instead of reimplementing verification. | Contract requires CLI/MCP or small UNO/Python command adapter over core outputs. |
| Range mapping | Writer document anchors map to stable Sourceright diagnostics. | Writer range-provenance table is documented in `packaging-decision.md`. |
| Write safety | Writer edits are reversible, previewed, audited, and explicit on apply. | Contract requires dry-run-first visible change plans, audit logs, explicit apply, and reversible plans. |
| Install smoke | Local install and uninstall smoke are documented before extension-site claims. | Install/uninstall smoke remains deferred until a `.oxt` package exists. |
| Review | `$conductor-review` checks that file processing is not described as LibreOffice extension support. | `tests/libreoffice_extension_packaging_policy.rs` enforces the claim boundary. |
