# mdBook Fallback Retirement Plan

## Purpose

The mdBook build (`book.toml`, `docs/src/`) was the original public docs surface.
This plan defines when and how it can be fully retired, leaving only the
Astro/Starlight site as the public documentation.

## Current State

| Surface              | Status                                        |
|----------------------|-----------------------------------------------|
| Astro/Starlight site | **Live** — canonical public docs              |
| mdBook build         | **Archival** — not deployed; source preserved |
| `book.toml`          | Kept for reference                            |
| `docs/src/`          | Archival Markdown source; used in parity tests|

## Retirement Conditions

The mdBook source tree (`docs/src/`) and `book.toml` can be retired when **all**
of the following conditions are met:

### 1. Content Parity Achieved

- [ ] Every page in `docs/src/SUMMARY.md` has a counterpart in the Astro site
  sidebar (currently tested by `docs_site_parity.rs`).
- [ ] The `docs-parity.ps1` script reports zero missing files.
- [ ] Content differences between archival and site pages are reviewed and
  resolved (either backport archival improvements to the site, or accept that
  the site version is intentionally shorter/longer).

### 2. Redirect Coverage

- [ ] A `404.html` redirect page is deployed that maps all known mdBook URLs to
  Astro URLs, following the policy in `redirect-canonical-policy.md`.
- [ ] The redirect mapping covers every page listed in `docs/src/SUMMARY.md`.

### 3. No Active mdBook References

- [ ] `README.md` does not link to any mdBook-specific URL.
- [ ] No CI/CD workflow references the mdBook build as a deployable artifact.
- [ ] `docs/src/publishing.md` clearly states the archival-only status.
- [ ] All internal repo links (CONTRIBUTING.md, etc.) point to the Astro site.

### 4. Search and Discovery Validated

- [ ] Starlight's Pagefind search indexes all guide and reference pages.
- [ ] A post-deploy check confirms the sitemap is complete.
- [ ] Search results return the Astro site pages, not stale mdBook URLs.

### 5. Observation Period

- [ ] The Astro site has been the sole public surface for at least **30 days**
  without reported issues.
- [ ] No user reports of broken links or missing content.
- [ ] GitHub Pages deployment has been stable across multiple pushes.

## Retirement Procedure

When all conditions are met, execute the following steps:

### Step 1: Announce

- Add a note to `docs-site/src/content/docs/guides/release-notes.md` indicating
  the archival mdBook source is being retired.
- Tag the commit with a `docs-retire-mdbook-source` annotation.

### Step 2: Archive the mdBook Source

- Move `docs/src/` to `docs/archival-src/` (preserves git history).
- Remove `book.toml` or move it to `docs/archival-src/book.toml`.
- Update `.gitattributes` or `.gitignore` as needed.

### Step 3: Update Parity Tests

- Update `tests/docs_site_parity.rs` to point at `docs/archival-src/` instead of
  `docs/src/`, or convert it to a historical snapshot test.
- Update `tests/docs_cutover_policy.rs` to reference the new archival location.
- Ensure `cargo test` passes.

### Step 4: Update CI/CD

- Remove any remaining mdBook build steps from CI workflows.
- The `.github/workflows/pages.yml` should only build and deploy the Astro site.

### Step 5: Update Documentation

- Update `docs-site/src/content/docs/guides/docs-cutover.md` to note the
  archival directory change.
- Update `docs-site/src/content/docs/guides/publishing.md` accordingly.
- Update `README.md` if it referenced the mdBook source.

### Step 6: Final Verification

- [ ] `cargo test` passes with all policy, parity, and site tests green.
- [ ] Astro site builds and deploys successfully.
- [ ] All redirects function correctly.
- [ ] No broken internal links anywhere in the repo.

## Post-Retirement Verification

After retirement, run these checks periodically (suggested: on each release):

1. **Parity check:** Run `scripts/docs-parity.ps1` (updated to scan the archival
   directory) to confirm no drift.
2. **Link check:** Crawl the deployed Astro site for broken links.
3. **Redirect check:** Verify the 404.html redirect map is still valid by
   testing a sample of old mdBook URLs.
4. **Search check:** Query Starlight Pagefind for key terms and confirm results
   are correct.

## Rollback Plan

If the Astro site becomes unavailable or critically broken:

1. The mdBook source in `docs/archival-src/` can be rebuilt with `mdbook build`.
2. Deploy the mdBook output as a temporary fallback via GitHub Pages.
3. Document the rollback in `docs-cutover.md` with a timestamp and reason.
4. Once the Astro site is fixed, revert to it and remove the temporary fallback.

This rollback should be treated as an emergency measure, not a routine option.
