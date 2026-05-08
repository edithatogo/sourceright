# Export Suite Spec

## Goal

Produce downstream reference files from verified CSL JSON.

## Scope

- XML, ENW, RIS, BibLaTeX, and YAML exports.
- Stable export ordering.
- Structural validation and reparse checks where practical.

## Boundaries

Exports use clean reference data and do not expose internal verification sidecar fields unless a diagnostic export is explicitly requested.
