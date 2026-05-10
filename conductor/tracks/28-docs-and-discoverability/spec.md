# Docs And Discoverability Spec

## Goal

Keep Sourceright's documentation reliable, discoverable, and aligned with the public CLI and package surfaces.

## Scope

- Keep mdBook working as the primary docs surface unless a Starlight/Astro product site becomes necessary.
- Add install, command, MCP, and release documentation.
- Validate docs in CI.
- Keep README and docs aligned with actual command behavior.

## Outputs

- Updated README and mdBook pages.
- CI docs build and smoke validation.
- Documented mdBook versus Starlight/Astro decision.

## Boundaries

Do not migrate docs stacks in this track unless the implementation includes a complete working site, CI build, navigation, and deployment replacement.
