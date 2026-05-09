# Exports

CSL JSON is canonical. Sourceright exports should be generated only from
verified, cleaned reference data. The export suite should make the generated
files useful to citation managers and review workflows without leaking internal
verification sidecar fields into clean outputs.

## Shared expectations

- `sourceright export` accepts a Sourceright workspace and writes one or more named
  output files.
- Records are emitted in a stable order, using the same deterministic ordering
  for every format.
- Clean exports include bibliographic data only. Internal review notes,
  confidence scores, raw extraction traces, and verification sidecar fields are
  excluded unless a diagnostic export explicitly requests them.
- Missing optional values are omitted rather than represented as empty strings,
  placeholder tokens, or format-invalid fields.
- Escaping and encoding are format-specific and deterministic. Unicode text is
  preserved where the target format supports it.
- Each generated file has a structural validation check. Where a practical
  parser exists, tests should reparse the generated file and compare the parsed
  record count and key identifiers with the source CSL JSON.

The implemented Rust exporter writes:

- `references.yaml`
- `references.xml`
- `references.ris`
- `references.enw`
- `references.bib`

`sourceright export --all [.sourceright-directory]` writes the full suite. `sourceright export --format <yaml|xml|ris|enw|biblatex> [.sourceright-directory]` writes a single format.

## Format criteria

### XML

- XML output is well-formed UTF-8 and has one root element containing one child
  element per exported reference.
- Required bibliographic fields are represented with predictable element names
  or attributes, including title, author/editor names where present, issued
  date, container title, volume, issue, pages, DOI, URL, ISBN, ISSN, PMID, and
  source identifier.
- XML escaping is validated for ampersands, angle brackets, quotes, apostrophes,
  and non-ASCII names.
- Structural validation reparses the file with an XML parser, verifies exactly
  one root, verifies the expected record count, and verifies that each record can
  be matched back to a CSL item id or stable derived key.

### ENW

- ENW output uses one EndNote record block per reference and terminates each
  record predictably.
- Core tags are mapped consistently, including reference type, title, authors,
  year, journal or container, volume, issue, pages, DOI, URL, ISBN/ISSN, notes
  where clean notes are allowed, and stable record id.
- Author and editor names emit one tag per person in source order.
- Structural validation checks that each record begins with an EndNote type tag,
  contains a title when the source has one, preserves person counts, and does
  not emit internal verification metadata.

### RIS

- RIS output uses one `TY  -`/`ER  -` block per reference.
- Tags are mapped consistently for type, title, authors/editors, publication
  year/date, journal or book title, volume, issue, pages, DOI, URL, ISBN, ISSN,
  PMID, and stable id.
- Multi-value fields emit repeated RIS tags rather than joined strings where RIS
  conventions expect repetition.
- Round-trip validation reparses RIS blocks, verifies block count, required
  start/end tags, source identifiers, DOI/URL preservation, and author/editor
  counts for representative fixtures.

### BibLaTeX

- BibLaTeX output emits one entry per reference with a deterministic citation
  key derived from the canonical record id or a documented stable fallback.
- Entry types map predictably from CSL item types, with a conservative fallback
  for unsupported types.
- Required and common fields are mapped to BibLaTeX names, including `title`,
  `author`, `editor`, `date` or `year`, `journaltitle`, `booktitle`,
  `publisher`, `volume`, `number`, `pages`, `doi`, `url`, `isbn`, `issn`, and
  `pmid` where supported by the chosen field policy.
- Special characters are escaped or braced so generated entries parse and do
  not lose capitalization-sensitive title text.
- Structural validation parses entries, verifies unique keys, verifies record
  count, checks required fields for representative item types, and confirms
  DOI/URL identifiers survive unchanged.

### YAML

- YAML output is a transparent diagnostic export of the clean canonical data,
  not a dump of private runtime state.
- The top level contains a stable version marker and an ordered list of
  references.
- Field names follow canonical CSL-style names unless a documented export field
  is needed for clarity.
- YAML preserves nested contributors, dates, identifiers, and source ids in a
  human-reviewable structure.
- Round-trip validation parses the YAML back into structured data, verifies the
  version marker, record count, key identifiers, and selected nested contributor
  and date fields against the source CSL JSON.
