# Microsoft Word Add-In Packaging Decision

## Decision

Track 67 is closed as an explicit deferral with a stable future Office Add-in
contract. Sourceright does not currently ship an Office Add-in manifest,
taskpane package, sideloadable Word add-in, or Microsoft AppSource listing.

Current DOCX support is document extraction and file-level processing. That is
not Word add-in support and does not imply in-document editing.

## Future Add-In Contract

A future Word add-in must be a thin adapter over the Rust core. It must:

- call Sourceright CLI JSON commands, local MCP, or a service wrapper around the
  same contracts;
- map Sourceright diagnostics back to Word document ranges without mutating
  `references.csl.json` or `references.verification.json` silently;
- support paragraph, footnote/endnote, table-cell, and bibliography anchors;
- keep all in-document edits dry-run first with a visible change plan;
- apply changes only after explicit user action and write an audit log;
- preserve a reversible plan for every proposed edit; and
- document sideload and AppSource evidence separately from local scaffolding.

## Range Provenance

Initial range provenance should use this shape before any write path exists:

| Word surface | Required provenance | Sourceright source |
| --- | --- | --- |
| Body paragraph | document id, paragraph index, character span | citation reconciliation or report diagnostic |
| Footnote/endnote | note id, note type, character span | citation reconciliation diagnostic |
| Table cell | table index, row, column, character span | extraction provenance diagnostic |
| Bibliography entry | reference id plus range or paragraph anchor | CSL/reference report diagnostic |

If a range cannot be mapped deterministically, the add-in must show the issue as
document-level guidance rather than applying edits to an inferred location.

## Publication Boundary

Microsoft AppSource remains deferred until an Office Add-in manifest and
taskpane package exist, the add-in sideloads locally, fixture documents prove
range mapping and reversible write plans, and listing evidence is recorded in
Track 69.
