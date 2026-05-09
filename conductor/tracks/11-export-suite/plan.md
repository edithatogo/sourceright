# Export Suite Plan

## Intent

Build the export suite from canonical, verified CSL JSON into XML, ENW, RIS,
BibLaTeX, and YAML. Planning assumes clean bibliographic exports by default and
diagnostic data only where explicitly requested.

## Work plan

1. Define the export command contract.
   - Accept canonical CSL JSON input.
   - Support selecting one format or writing the full suite.
   - Use deterministic filenames, stable record ordering, and explicit
     overwrite behavior.
   - Document clean versus diagnostic output behavior before implementation.

2. Establish shared export invariants.
   - Emit one output record per canonical CSL item.
   - Preserve source ids, DOI, URL, ISBN, ISSN, PMID, titles, contributors, and
     dates where present.
   - Omit missing optional fields rather than emitting empty placeholders.
   - Exclude verification sidecar fields from clean exports.
   - Keep Unicode text unless a target format requires explicit escaping.

3. Implement YAML first as the transparent diagnostic baseline.
   - Include an export schema/version marker.
   - Emit an ordered `references` collection.
   - Preserve nested contributors, dates, and identifiers in a structure that can
     be parsed back and compared with the source CSL JSON.
   - Use this output to clarify expected field policy for the other exporters.

4. Implement RIS and BibLaTeX as citation-manager exports.
   - RIS uses complete `TY  -` to `ER  -` blocks with repeated tags for repeated
     contributors and identifiers.
   - BibLaTeX uses deterministic citation keys, conservative entry-type mapping,
     and parser-safe escaping/bracing.
   - Both formats preserve DOI/URL and contributor counts in representative
     fixtures.

5. Implement ENW and XML.
   - ENW maps CSL fields to predictable EndNote tags, one tag per contributor,
     and one clearly terminated record block per reference.
   - XML is well-formed UTF-8 with a single root element and one child element
     per reference.
   - XML element and attribute naming should be stable and documented before the
     exporter is treated as public.

6. Add validation gates.
   - Every format has a structural validation test.
   - YAML, XML, RIS, and BibLaTeX should be reparsed where practical.
   - ENW should receive block/tag structural checks unless a reliable parser is
     adopted.
   - Validation compares record count, stable ids or keys, DOI/URL preservation,
     and contributor counts against the source fixture.

## Initial acceptance criteria

- Given a representative canonical CSL JSON fixture, the export suite generates
  XML, ENW, RIS, BibLaTeX, and YAML files with stable names and stable record
  ordering.
- Clean exports do not include verification sidecar fields, raw extraction
  traces, confidence scores, or internal review notes.
- Each format has a documented field mapping for title, contributors, dates,
  container/publisher fields, pages, DOI, URL, ISBN, ISSN, PMID, and source id.
- Generated files pass their format-specific validation checks.
- Round-trip or reparse validation proves that record count and key identifiers
  survive export for YAML, XML, RIS, and BibLaTeX.
- ENW validation proves one correctly delimited EndNote-style record per source
  item and predictable tag mapping for representative references.
