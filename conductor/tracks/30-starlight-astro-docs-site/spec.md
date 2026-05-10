# Starlight Astro Docs Site Spec

## Goal

Move Sourceright documentation from mdBook to a Starlight/Astro site that is
more product-like, more discoverable, and better suited to public installation
and publishing guidance.

## Scope

- Create a Starlight/Astro documentation site with content parity for the current docs surface.
- Keep navigation, search, and pages deployment equivalent or better than mdBook.
- Preserve the CLI, MCP, benchmarks, publishing, and contributing references.
- Keep a migration path that allows mdBook to remain the fallback until the new site is live and validated.

## Outputs

- Astro/Starlight configuration and content tree.
- Pages deployment workflow for the new docs site.
- Content parity checklist and redirect/retention policy.

## Boundaries

The migration must not silently drop existing docs content. The old mdBook surface can only be retired once the new site has CI, preview, and deployment parity.
