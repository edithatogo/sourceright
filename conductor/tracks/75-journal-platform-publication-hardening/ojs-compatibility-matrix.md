# OJS Compatibility Matrix

Date: 2026-06-09  
Plugin package: `sourceright-ojs-generic-plugin-0.1.0`

## Supported targets (declared)

| OJS release line | Plugin type | Evidence | Live smoke |
| --- | --- | --- | --- |
| OJS 3.3+ | `plugins.generic` generic plugin | `version.xml` (`application=ojs`), install docs | Not run in default CI |
| OJS 3.x (current) | Same | PKP plugin guide alignment | Opt-in via Docker script |

## Server prerequisites

| Requirement | Documented in | Default CI |
| --- | --- | --- |
| PHP on OJS host (plugin registration) | `plugins/ojs/sourceright/README.md` | PHP lint skipped when PHP not on PATH |
| Sourceright CLI on server or reachable host | Track 60 installation guide | CLI fixture e2e passes |
| Workspace with CSL + verification sidecar | Plugin README | Fixture-backed screening test |

## Permissions and settings

| Setting / boundary | Behavior |
| --- | --- |
| `sourcerightCliPath` | Configurable CLI path; defaults to `sourceright` on PATH |
| `allowExplicitWrites` | Write-capable flows gated; preview/default export only |
| Canonical CSL | Plugin must not silently overwrite `references.csl.json` |
| Screening command | `sourceright journal-screen --platform ojs` |

## Not verified in this slice

- Live OJS admin UI enable/disable cycle
- PKP Plugin Gallery listing or review outcome
- Multi-version matrix across specific OJS patch releases

Live/disposable instance proof remains opt-in and approval-gated.
