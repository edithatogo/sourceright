# Track 83 - Janeway Platform Reconnaissance

## Goal

Map Janeway's plugin and extension surface so Sourceright can be integrated as a plugin, a sidecar service, or a hybrid bridge without overclaiming support.

## User outcome

Maintainers can see whether Janeway is a viable second open-source journal target after OJS, what integration shapes are realistic, and what evidence would be required to promote a claim beyond reconnaissance.

## Scope

- Inspect Janeway official docs, plugin guidance, and packaging story.
- Map Janeway install, extension, and test surfaces to Sourceright integration options.
- Update the journal-integrations docs to name Janeway explicitly and explain the integration shapes.
- Define a platform capability matrix that separates plugin-first, sidecar, and hybrid options.
- Capture fixture or smoke evidence requirements before any package work begins.

## Out of scope

- Janeway upstream PRs or acceptance claims.
- Live Janeway deployment claims.
- Vendor-only or proprietary platform integrations.
- Plugin implementation code.
- Automatic publication or self-registration.

## Data contracts

- `docs/src/journal-integrations.md` and `docs-site/src/content/docs/journal-integrations.md` for the human-readable platform map.
- Janeway documentation and plugin metadata pages for evidence notes.
- `conductor/tracks/83-janeway-platform-reconnaissance/test-matrix.md` for default-CI checks and evidence targets.

## Claim boundary

> Janeway is "reconnaissance scoped", not "supported" or "published".

All evidence notes for this track must preserve the boundary that Janeway is the next candidate platform, not a shipped integration.

## Evidence level target

**contracted** - the track is complete enough when the Janeway integration shapes, docs updates, and evidence requirements are explicit and reviewable.

## Parallelization plan

- Docs discovery and integration-shape mapping can run in parallel.
- The platform matrix and claim boundary should be locked before any package or smoke work begins.
- The journal-integrations docs update can be reviewed independently once the Janeway contract is written.
