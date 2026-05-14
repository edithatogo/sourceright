# Fallback Retirement Plan

## When to Retire
mdBook surface retires when:
1. Astro site has full content parity verified
2. Pages workflow deploys successfully for 2+ consecutive releases
3. No external links reference mdBook URLs

## How to Retire
1. Remove book.toml
2. Archive docs/src/ as git history
3. Update all docs-site references to Astro as canonical
4. Remove mdBook build from CI

## Post-Retirement Verification
- Run tests/docs_site_parity.rs passes
- Pages workflow deploys cleanly
- No broken links in Astro site
