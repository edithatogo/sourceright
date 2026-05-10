# Public Docs Cutover And Launch Spec

## Goal

Finish the public docs migration by cutting over navigation, search, deployment,
and canonical links to the Starlight/Astro site while retaining any archive or
fallback surface that is still needed for regression comparison.

## Scope

- Make the Astro docs site the canonical public documentation target.
- Preserve redirects or equivalent navigation continuity from the mdBook
  surface.
- Keep CI and Pages deployment aligned with the public docs site.
- Retire the fallback mdBook surface only when the new site is stable.

## Outputs

- Public docs launch checklist.
- Redirect and canonical-link policy.
- Archive or fallback retirement plan.

## Boundaries

The cutover must not strand existing docs users. Legacy mdBook can remain as a
reference until the public site has the necessary content and link parity.
