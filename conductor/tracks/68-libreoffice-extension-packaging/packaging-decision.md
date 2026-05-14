# LibreOffice Extension Packaging Decision

## Decision

Track 68 is closed as an explicit deferral with a stable future LibreOffice
Writer extension contract. Sourceright does not currently ship a `.oxt`
package, UNO bridge, macro package, local install smoke, or LibreOffice
Extensions listing.

Current ODT/DOCX support is file-level processing, not LibreOffice Writer extension support, and does not imply in-document editing.

## Future Extension Contract

A future LibreOffice package must be a thin adapter over the Rust core. It must:

- call Sourceright CLI JSON commands, local MCP, or a small UNO/Python command
  adapter over those same contracts;
- map Sourceright diagnostics back to Writer ranges without mutating
  `references.csl.json` or `references.verification.json` silently;
- support paragraph, footnote/endnote, table-cell, text-frame, and bibliography
  anchors;
- keep Writer edits dry-run first with a visible change plan;
- apply changes only after explicit user action and write an audit log;
- preserve a reversible plan for every proposed edit; and
- document LibreOffice Extensions publication evidence separately from local
  `.oxt` scaffolding.

## Writer Range Provenance

Initial Writer provenance should use this shape before any write path exists:

| Writer surface | Required provenance | Sourceright source |
| --- | --- | --- |
| Body paragraph | document id, paragraph index, character span | citation reconciliation or report diagnostic |
| Footnote/endnote | note id, note type, character span | citation reconciliation diagnostic |
| Table cell | table index, row, column, character span | extraction provenance diagnostic |
| Text frame | frame id, paragraph index, character span | extraction provenance diagnostic |
| Bibliography entry | reference id plus range or paragraph anchor | CSL/reference report diagnostic |

If a Writer range cannot be mapped deterministically, the extension must show
document-level guidance rather than applying edits to an inferred location.

## Publication Boundary

LibreOffice Extensions publication remains deferred until a `.oxt` package
exists, installs and uninstalls locally, fixture documents prove range mapping
and reversible write plans, and listing evidence is recorded in Track 69.
