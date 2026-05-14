# LibreOffice Extension Packaging Test Matrix

| Area | Required check |
| --- | --- |
| Packaging | `.oxt` packaging exists, or the track records an explicit deferral. |
| Adapter | UNO/Python or command adapter calls the Rust core instead of reimplementing verification. |
| Range mapping | Writer document anchors map to stable Sourceright diagnostics. |
| Write safety | Writer edits are reversible, previewed, audited, and explicit on apply. |
| Install smoke | Local install and uninstall smoke are documented before extension-site claims. |
| Review | `$conductor-review` checks that file processing is not described as LibreOffice extension support. |
