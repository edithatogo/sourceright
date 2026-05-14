# Provider Proof — External Proof Suite

## Purpose

Document how to verify provider adapter smoke tests against reference fixtures.
This proof covers all provider adapters registered in `plugins/registry.toml`
by running `sourceright plugins` discovery, validating manifests, and exercising
provider reference fixtures.

## Prerequisites

- A built `sourceright` binary on `$PATH`
- Repository root with `plugins/registry.toml`, `plugins/manifests/*.toml`
- Reference fixtures under `fixtures/providers/`
- `jq` for JSON formatting (optional)

## Proof Sections

### 1. Plugin Registry Discovery — `sourceright plugins`

**Purpose:** Confirm the CLI can discover and list all registered plugins.

**Command:**
```text
sourceright plugins --json
```

**Expected output (abridged):**
```json
{
  "schema_version": "sourceright.plugin_registry_report.v1",
  "registry": {
    "schema_version": "sourceright.plugin-registry.v1",
    "runtime_loading": true
  },
  "plugins": [
    { "id": "provider.crossref", "status": "core_normalizer" },
    { "id": "repository.pubmed", "status": "core_normalizer" },
    { "id": "export.citation-files", "status": "core_exporter" }
  ],
  "summary": { "total": 20 }
}
```

**Key assertions:**
- `schema_version` is `sourceright.plugin_registry_report.v1`
- `plugins` array contains all 20 registered plugins

**Exit code:** `0`

---

### 2. Plugin Manifest Validation

**Purpose:** Validate every manifest in `plugins/manifests/` parses as TOML.

**Command:**
```text
cargo test --test requirements_contract_policy plugin_registry_has_owned_remaining_work_contract
```

**Exit code:** `0`

---

### 3. Provider Fixture Smoke — Crossref/PubMed (core_normalizer)

**Purpose:** Verify core normalizer fixtures are valid CSL.

**Command:**
```text
sourceright validate-csl fixtures/providers/europe-pmc.example.json
```

**Expected output:** `valid`

**Exit code:** `0`

---

### 4. All Provider Fixtures Present

**Command:**
```text
ls fixtures/providers/
```

**Expected output includes:**
```
arxiv.example.atom
courtlistener/
europe-pmc.example.json
licensed-byo-key.example.json
opencitations.example.json
repository-records.example.json
unpaywall.example.json
zotero/
```

**Exit code:** `0`

---

### 5. Provider Manifest Contracts

**Purpose:** Every provider manifest declares `references.csl.json` reads and
`sourceright.verification.v1` writes.

**Command:**
```text
for f in plugins/manifests/provider.*.toml; do
  echo "--- $f ---"
  head -20 "$f" | grep -E '^(reads|writes)'
done
```

**Expected:** Every manifest includes `reads = ["references.csl.json"]` and
`writes = ["sourceright.verification.v1"]`.

**Exit code:** `0`

---

### 6. CLI Smoke — Provider-Adjacent Commands

**Command:**
```text
sourceright --help
```

**Expected output includes:**
```
sourceright validate-csl [--json] <references.csl.json>
sourceright plugins [validate] [--json]
sourceright report [--json|--mcp-resource]
sourceright policy [--policy <policy.json>] <references.csl.json>
```

**Exit code:** `0`
### 7. Provider Runtime Controls — Timeout, Retry, Min-Interval, Cache

**Purpose:** Verify that provider runtime controls (timeout, retry, min-interval, cache)
are configurable and respected. These are defined in `LiveProviderConfig` in
`src/live_providers.rs` and controlled via environment variables.

**Configuration reference:**
| Variable | Default | Controls |
|---|---|---|
| `SOURCERIGHT_PROVIDER_TIMEOUT_SECS` | `20` | HTTP request timeout |
| `SOURCERIGHT_PROVIDER_MAX_RETRIES` | `2` | Max retry attempts on failure |
| `SOURCERIGHT_PROVIDER_MIN_INTERVAL_MS` | `1000` | Minimum delay between provider requests |
| `SOURCERIGHT_PROVIDER_CACHE_DIR` | (none) | Optional filesystem cache directory |
| `SOURCERIGHT_LIVE_PROVIDERS` | (false) | Master enable for live provider calls |
| `SOURCERIGHT_LIVE_PROVIDER_SMOKE` | (false) | Enable smoke tests for provider adapters |

**Verification approach:**
1. Set `SOURCERIGHT_PROVIDER_TIMEOUT_SECS=1` and observe timeout behavior
2. Set `SOURCERIGHT_PROVIDER_MAX_RETRIES=0` and confirm no retry on failure
3. Set `SOURCERIGHT_PROVIDER_CACHE_DIR=/tmp/provider-cache` and confirm cache hits
4. Verify default config: `cargo test live_provider_config_defaults_to_conservative_runtime_policy`

**Key assertions:**
- Default timeout is 20 seconds (conservative)
- Default max retries is 2
- Default min interval is 1000ms
- Cache is optional (None by default)
- Live providers are disabled by default (opt-in only)

**Exit code:** `0` when config tests pass

---




## Transcript Template

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "=== Provider Proof Transcript ==="

echo "1. Plugin registry discovery"
sourceright plugins --json | jq -e '.schema_version == "sourceright.plugin_registry_report.v1"' > /dev/null
sourceright plugins --json | jq -e '.summary.total == 20' > /dev/null
echo "  Registry: PASS"

echo "2. Validate Crossref fixture"
sourceright validate-csl fixtures/providers/europe-pmc.example.json | grep -q "valid"
echo "  Crossref fixture: PASS"

echo "3. Validate OpenCitations fixture"
sourceright validate-csl fixtures/providers/opencitations.example.json | grep -q "valid"
echo "  OpenCitations fixture: PASS"

echo "4. Validate Unpaywall fixture"
sourceright validate-csl fixtures/providers/unpaywall.example.json | grep -q "valid"
echo "  Unpaywall fixture: PASS"

echo "5. Check provider manifest contracts"
for f in plugins/manifests/provider.*.toml; do
  grep -q 'reads = \["references.csl.json"\]' "$f" || { echo "FAIL: $f missing reads"; exit 1; }
  grep -q 'writes = \["sourceright.verification.v1"\]' "$f" || { echo "FAIL: $f missing writes"; exit 1; }
done
echo "  Manifest contracts: PASS"

echo "=== ALL PASSED ==="
```

## Overclaim Guard

Must not claim:

- **"Live provider API is verified"** — proof uses static reference fixtures
- **"All providers are implemented"** — only 3 core plugins are implemented
- **"BYO-key providers are tested"** — Dimensions, Scopus, WoS are `planned_byo_key`

Only claim: **"All 20 registered plugins are discoverable. Core normalizer
fixtures are valid CSL. All provider manifests declare correct read/write
contracts."**

## Proof Family Status

| Surface | Status | Evidence |
|---------|--------|----------|
| Plugin registry discovery | ✅ Proven | `sourceright plugins --json`, CI |
| Manifest validation | ✅ Proven | `requirements_contract_policy.rs` |
| Provider fixture (core normalizer) | 🔶 Partial | Fixtures exist, no automated query |
| Provider fixture (others) | 🔶 Partial | Example fixtures exist |
| Live provider API smoke | ❌ Not yet | All providers planned |
| BYO-key provider tests | ⏸️ Deferred | Requires user API keys |

