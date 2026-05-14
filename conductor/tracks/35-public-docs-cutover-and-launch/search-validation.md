# Search Validation: Starlight / Pagefind

## How Starlight Search Indexing Works

Starlight (v0.39.2 used in this project) bundles **[Pagefind](https://pagefind.app/)** as its built-in search engine. Pagefind is a static full-text search library that runs at build time and requires no server-side components.

### Build-Time Indexing

During `astro build`, Starlight automatically invokes Pagefind after the static HTML pages are generated. Pagefind:

1. Crawls the built HTML output in `docs-site/dist/`
2. Extracts text content from each page, respecting headings, paragraphs, code blocks, and frontmatter
3. Builds a search index (fragment files, metadata, and a WASM-based runtime) placed in `docs-site/dist/pagefind/`
4. Generates `pagefind/pagefind.js` (the JavaScript bundle) and `pagefind/pagefind-entry.json` (index manifest)

### Runtime Search Behavior

When a visitor types into the Starlight search bar:

- `pagefind.js` loads the WASM index fragment files from `/sourceright/pagefind/`
- Queries are matched against the indexed content using substring/token matching with ranking
- Results display page title, excerpt snippet (with highlighted matches), and a link to the page

### Default Configuration

The current `astro.config.mjs` does **not** explicitly configure search, so Starlight applies its defaults:

- All pages in `src/content/docs/` are indexed automatically
- No pages are excluded from search
- Search UI is the built-in Starlight search dialog (triggered by Ctrl+K / Cmd+K or the search icon)
- No custom Pagefind options (e.g., ranking, language, indexing scope) are set

### Key Files Produced by Pagefind

| File / Directory | Purpose |
|---|---|
| `docs-site/dist/pagefind/pagefind.js` | Client-side search runtime |
| `docs-site/dist/pagefind/pagefind-entry.json` | Index entry manifest |
| `docs-site/dist/pagefind/fragment/` | Sharded index fragment files |
| `docs-site/dist/pagefind/pagefind-modular-ui.css` | Optional UI styling (if used) |

---

## How to Verify Search Works Post-Deployment

### Prerequisites

- The deployed site is live at `https://edithatogo.github.io/sourceright/`
- The Pages workflow has completed successfully (see `.github/workflows/pages.yml`)

### Verification Steps

#### 1. Confirm Pagefind Index Was Built

Check that the build output contains the `pagefind/` directory:

```bash
ls docs-site/dist/pagefind/
```

Expected: `pagefind.js`, `pagefind-entry.json`, and a `fragment/` subdirectory with `.pf` fragment files.

If `docs-site/dist/pagefind/` is missing or empty, the build either failed or Pagefind did not run (check for Starlight version or build warnings).

#### 2. Confirm the Search Endpoint Returns a 200

The Pagefind entry JSON must be served from the deployed site:

```bash
curl -sI https://edithatogo.github.io/sourceright/pagefind/pagefind-entry.json | head -n 1
# Expected: HTTP/2 200
```

#### 3. Confirm the Search JS Bundle Loads

```bash
curl -sI https://edithatogo.github.io/sourceright/pagefind/pagefind.js | head -n 1
# Expected: HTTP/2 200
```

#### 4. Verify Index Contains Pages

Fetch the entry manifest and check that it lists known pages:

```bash
curl -s https://edithatogo.github.io/sourceright/pagefind/pagefind-entry.json | jq '.version'
# Expected: non-null version string (e.g., "0.12.5")
```

The entry JSON lists the fragment shards. Each shard contains metadata about indexed pages.

#### 5. Perform a Search Query via Pagefind API

Pagefind supports a JSON search API at `pagefind/fragment/<shard>.pf`. The simplest end-to-end check is to load the search JS in a headless browser and call `pagefind.search()` (see Automated Search Testing below).

Alternatively, inspect the fragment directory for `.pf` files:

```bash
ls docs-site/dist/pagefind/fragment/ | head -5
# Expected: one or more .pf fragment files
```

#### 6. In-Browser Manual Test

Open the deployed site in a browser and:

- Press Ctrl+K (or Cmd+K on macOS) to open the search dialog
- Type a known term that appears in docs (e.g., "reference", "citation", "plugin", "CLI")
- Verify that results appear, showing page titles and relevant snippets
- Click a result and confirm it navigates to the correct page

---

## Manual QA Checklist for Search

Use this checklist to manually verify search before declaring the cutover complete.

| # | Check | Pass/Fail | Notes |
|---|---|---|---|
| 1 | **Search dialog opens** -- Press Ctrl+K or click the search icon | | |
| 2 | **Known term returns results** -- Search for "reference" | | Should list multiple pages |
| 3 | **Exact phrase match** -- Search for "canonical content model" | | Should match CSL/spec pages |
| 4 | **Partial word match** -- Search for "verif" should match "verification" | | Pagefind does substring matching |
| 5 | **Code term** -- Search for "astro build" or "npm" | | Should match CLI guide |
| 6 | **No results for gibberish** -- Search for "zzzzzxyzzy" | | Should show empty state |
| 7 | **Result snippet shows context** -- Verify excerpt highlights match terms | | |
| 8 | **Navigation from result** -- Click a result link; page loads correctly | | |
| 9 | **Page title in results** -- Each result displays the correct page title | | |
| 10 | **Mobile viewport** -- Open DevTools, switch to mobile, repeat checks 1-9 | | |
| 11 | **Search closes gracefully** -- Press Escape or click outside dialog | | |
| 12 | **Search on a guide page** -- Open /guides/quickstart, search, verify results | | |
| 13 | **Search on a reference page** -- Open /reference/providers, search, verify results | | |
| 14 | **All pages indexed** -- Search distinctive terms from each major section | | Cross-reference with page count |
| 15 | **Pagefind entry manifest loads** -- 200 on pagefind-entry.json | | |
| 16 | **Pagefind JS bundle loads** -- 200 on pagefind.js | | |
| 17 | **No console errors** -- Open DevTools Console; verify no 404s | | |

---

## Automated Search Testing

### Option A: Fetch-Based Smoke Test (No Browser)

A lightweight smoke test using `curl` and `jq` can verify that Pagefind assets are deployed and the index is non-empty. This can be run in CI as a post-deployment check.

Create `scripts/verify-search-index.sh`:

```bash
#!/usr/bin/env bash
# Usage: ./verify-search-index.sh [base_url]
# Default: https://edithatogo.github.io/sourceright
BASE="${1:-https://edithatogo.github.io/sourceright}"
echo "=== Search Index Verification ==="
ENTRY_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/pagefind/pagefind-entry.json")
if [ "$ENTRY_STATUS" != "200" ]; then
  echo "FAIL: pagefind-entry.json returned $ENTRY_STATUS (expected 200)"
  exit 1
fi
echo "PASS: pagefind-entry.json returns 200"
JS_STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$BASE/pagefind/pagefind.js")
if [ "$JS_STATUS" != "200" ]; then
  echo "FAIL: pagefind.js returned $JS_STATUS (expected 200)"
  exit 1
fi
echo "PASS: pagefind.js returns 200"
FRAGMENT_COUNT=$(curl -s "$BASE/pagefind/pagefind-entry.json" | jq '.fragments | length')
if [ -z "$FRAGMENT_COUNT" ] || [ "$FRAGMENT_COUNT" = "0" ]; then
  echo "FAIL: No search fragments found in index"
  exit 1
fi
echo "PASS: $FRAGMENT_COUNT search fragment(s) found"
echo "=== All search index checks passed ==="
```

**Integration:** Add as a step after deploy in `.github/workflows/pages.yml`.

### Option B: Playwright E2E Test

A Playwright test opens the deployed site in a headless browser and exercises the search UI directly. This catches issues that fetch-only checks miss (JS runtime errors, UI rendering).

Create `tests/verify-search.spec.js`:

```javascript
import { test, expect } from '@playwright/test';
const BASE = 'https://edithatogo.github.io/sourceright';

test.describe('Starlight Search', () => {
  test('opens and shows results for known term', async ({ page }) => {
    await page.goto(BASE);
    await page.keyboard.press('Control+k');
    const input = page.locator('starlight-search input[type="search"], [role="dialog"] input');
    await expect(input).toBeVisible({ timeout: 10000 });
    await input.fill('reference');
    await page.waitForTimeout(2000);
    const results = page.locator('[role="dialog"] a, .pagefind-ui__result');
    await expect(results.first()).toBeVisible({ timeout: 15000 });
  });
  test('handles no results gracefully', async ({ page }) => {
    await page.goto(BASE);
    await page.keyboard.press('Control+k');
    const input = page.locator('starlight-search input[type="search"], [role="dialog"] input');
    await expect(input).toBeVisible({ timeout: 10000 });
    await input.fill('zzzzzxyzzy');
    await page.waitForTimeout(2000);
    const noResults = page.locator('text=No results').or(page.locator('.pagefind-ui__message'));
    await expect(noResults).toBeVisible({ timeout: 10000 });
  });
  test('result navigates to correct page', async ({ page }) => {
    await page.goto(BASE);
    await page.keyboard.press('Control+k');
    const input = page.locator('starlight-search input[type="search"], [role="dialog"] input');
    await expect(input).toBeVisible({ timeout: 10000 });
    await input.fill('quickstart');
    await page.waitForTimeout(2000);
    const result = page.locator('[role="dialog"] a, .pagefind-ui__result').first();
    await expect(result).toBeVisible({ timeout: 10000 });
    await result.click();
    await expect(page).toHaveURL(/quickstart/);
  });
});
```

**Setup:** `npm init playwright@latest` in repo root or `docs-site/`
**Run:** `npx playwright test tests/verify-search.spec.js --project=chromium`

### Option C: Rust Integration Test

A Rust test (like `docs_site_policy.rs`) verifies that the local build output contains the expected Pagefind files. Fast CI gate before deploying.

Create `tests/search_index_policy.rs`:

```rust
use std::fs;

#[test]
fn pagefind_index_is_present_after_build() {
    let pagefind_dir = "docs-site/dist/pagefind";
    let pagefind_js = format!("{pagefind_dir}/pagefind.js");
    let pagefind_entry = format!("{pagefind_dir}/pagefind-entry.json");
    let fragment_dir = format!("{pagefind_dir}/fragment");

    assert!(fs::metadata(&pagefind_js).is_ok(), "pagefind.js must exist");
    assert!(fs::metadata(&pagefind_entry).is_ok(), "pagefind-entry.json must exist");
    assert!(fs::metadata(&fragment_dir).is_ok(), "fragment/ dir must exist");

    let entry: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&pagefind_entry).expect("read pagefind-entry.json"),
    ).expect("pagefind-entry.json must be valid JSON");

    assert!(entry.get("version").and_then(|v| v.as_str()).is_some(),
        "pagefind-entry.json must have version field");

    let count = fs::read_dir(&fragment_dir).expect("read fragment/").count();
    assert!(count > 0, "At least one fragment file must exist");
}
```

**Note:** Validates the index was *built*, not runtime behavior.

---

## Integration with CI

### Recommended CI Pipeline

```
[Astro Build] -> [Rust test: pagefind files exist] -> [Deploy to Pages] -> [curl smoke test] -> [Playwright E2E]
```

1. **During build** -- `npm run build` in `docs-site/` (already in `pages.yml`)
2. **Pre-deploy CI gate** -- Add Rust test `search_index_policy.rs` to the test suite
3. **Post-deploy smoke** -- Add `scripts/verify-search-index.sh` as a step after deploy in `pages.yml`
4. **Nightly/per-release E2E** -- Playwright tests against the live deployed URL (manual trigger or scheduled workflow)

### What to Do If Search Fails

| Symptom | Likely Cause | Fix |
|---|---|---|
| `pagefind/` directory missing | Starlight version mismatch or build error | Run `npm ls @astrojs/starlight` to verify version; check build output for warnings |
| `pagefind-entry.json` returns 404 | Base path mismatch | Verify `base: '/sourceright/'` in `astro.config.mjs`; Pagefind assets served relative to site base |
| Search dialog opens but no results | Index empty or pagefind.js failed to load | Check browser console for 404s; verify .pf fragment files in dist/pagefind/fragment/ |
| Some pages missing from results | Pages excluded or not in src/content/docs/ | All content in that directory is indexed by default |
| WASM loading errors in console | Browser CSP or MIME type restrictions | GitHub Pages serves .wasm with correct MIME type; check if custom CSP blocks it |

---

## References

- [Starlight Search Documentation](https://starlight.astro.build/guides/search/)
- [Pagefind Documentation](https://pagefind.app/docs/)
- [Starlight built-in search (v0.39.x)](https://starlight.astro.build/reference/configuration/#search)
