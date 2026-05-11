# Docs Cutover

The public docs path is now the Starlight/Astro site in `docs-site/`, deployed
through GitHub Pages.

## Current Shape

- `docs-site/` is the public docs build target.
- `docs/src/` remains the archival Markdown source.
- CI validates the docs-site build and the Rust-side documentation checks.

## Cutover Rules

- Keep the archival Markdown source for regression and source-history review.
- Keep canonical links and navigation aligned with the public site.
- Retire any alternate public docs surface only if one is reintroduced for a
  migration or rollback exercise.

## Launch Checklist

- Confirm the docs site builds.
- Confirm Pages uploads `docs-site/dist`.
- Confirm the README and publishing guidance point to the public site.
- Confirm the archival Markdown source still matches the public site content.
