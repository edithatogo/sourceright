# Export Suite Spec

## Goal

Produce downstream reference files from verified CSL JSON in formats expected by
citation managers, static review workflows, and human audit steps.

## Scope

- XML, ENW, RIS, BibLaTeX, and YAML exports.
- Stable export ordering.
- Structural validation and round-trip or reparse checks where practical.
- Clean export defaults with diagnostic output explicitly separated.

## Boundaries

Exports use clean reference data and do not expose internal verification sidecar fields unless a diagnostic export is explicitly requested.

No Rust source, workflow, or CI behavior is specified by this planning pass.

## Shared requirements

- CSL JSON remains the canonical source. Exporters do not invent bibliographic
  facts that are missing from the canonical record.
- Export order is deterministic across all formats for the same input.
- Optional fields are omitted when absent.
- Person-name order is preserved from the canonical record.
- Identifiers are preserved without lossy normalization unless the target format
  requires a documented representation.
- Clean outputs exclude internal-only fields, including verification sidecars,
  extraction traces, confidence scores, review decisions, and runtime metadata.
- Format-specific escaping is part of the exporter contract and is covered by
  fixtures with punctuation, XML-sensitive characters, BibLaTeX-sensitive
  characters, and non-ASCII names.

## Format requirements

### XML

- Emits one well-formed UTF-8 XML document.
- Uses a single root element and one reference element per CSL item.
- Represents stable source id, title, contributors, date, container fields,
  publisher fields, volume, issue, pages, DOI, URL, ISBN, ISSN, and PMID where
  present.
- Escapes XML-sensitive characters and reparses successfully with a standard XML
  parser.
- Validation checks root shape, record count, stable ids, and selected nested
  contributor/date fields.

### ENW

- Emits one EndNote-style record block per CSL item.
- Starts each record with a predictable reference type tag and terminates record
  blocks consistently.
- Emits repeated contributor tags in canonical order.
- Maps title, year/date, container title, volume, issue, pages, DOI, URL,
  ISBN/ISSN, PMID, and stable id where supported by the ENW field policy.
- Validation checks block count, required tags for representative item types,
  contributor counts, and absence of internal verification metadata.

### RIS

- Emits one `TY  -` to `ER  -` block per CSL item.
- Uses repeated RIS tags for repeated contributors and multi-value fields where
  expected by RIS consumers.
- Maps item type, title, contributors, date, container title, publisher fields,
  volume, issue, pages, DOI, URL, ISBN, ISSN, PMID, and stable id.
- Round-trip validation reparses RIS blocks and compares record count, required
  delimiters, stable ids, DOI/URL values, and contributor counts.

### BibLaTeX

- Emits one BibLaTeX entry per CSL item.
- Citation keys are unique and deterministic.
- CSL item types map to documented BibLaTeX entry types, with a conservative
  fallback for unsupported types.
- Maps title, author, editor, date/year, journaltitle/booktitle, publisher,
  volume, number, pages, DOI, URL, ISBN, ISSN, PMID, and stable id where
  supported by the field policy.
- Escaping and bracing preserve parser validity and title capitalization where
  required.
- Validation parses entries, checks unique keys, record count, representative
  required fields, and DOI/URL preservation.

### YAML

- Emits parseable YAML with a top-level export version marker and ordered
  `references` collection.
- Uses canonical CSL-style field names unless a documented export-only field is
  required.
- Preserves nested contributors, dates, identifiers, and source ids in a
  human-reviewable structure.
- Round-trip validation parses the YAML and compares version marker, record
  count, stable ids, DOI/URL values, selected contributors, and selected date
  parts with the source CSL JSON.
