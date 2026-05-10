# Docs Cutover

The public docs path is now the Starlight/Astro site in `docs-site/`, deployed
through GitHub Pages.

## Current Shape

- `docs-site/` is the public docs build target.
- `docs/src/` remains the fallback and archival Markdown source.
- CI validates both the docs-site build and the Rust-side documentation checks.

## Cutover Rules

- Keep mdBook only until the new site proves content parity and stable Pages
  deployment.
- Keep canonical links and navigation aligned with the public site.
- Retire the fallback docs surface only after the public site is proven in CI
  and release review.

## Launch Checklist

- Confirm the docs site builds.
- Confirm Pages uploads `docs-site/dist`.
- Confirm the README and publishing guidance point to the public site.
- Confirm the fallback surface still renders for regression comparison.
