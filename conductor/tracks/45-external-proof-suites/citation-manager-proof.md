# Citation Manager Proof — External Proof Suite

## Purpose

This document defines how to verify citation-manager sync integration (Zotero,
EndNote) using fixture-backed test commands. It serves as a **human-readable
proof transcript** and **verification checklist** for the Citation-Manager Proof
family.

## Prerequisites

- A built `sourceright` binary on `$PATH`, or accessible via `cargo run --bin sourceright`
- The repository root containing `fixtures/providers/zotero/` fixture files
- A workspace directory for running citation sync commands
- `jq` for JSON formatting (optional, for readability)

## Proof Sections

### 1. Validate Zotero Fixture Files Exist

**Purpose:** Confirm that the Zotero fixture directory contains expected files.

**Command:**
```text
ls fixtures/providers/zotero/
```

**Expected output includes:**
```
preview-exact-match.json
preview-title-update.json
apply-success-preview.json
README.md
```

**Exit code:** `0`

---

### 2. Initialise a Workspace

**Purpose:** Create a `.sourceright` workspace for citation sync testing.

**Command:**
```text
sourceright init
test -d .sourceright || { echo "FAIL: .sourceright not created"; exit 1; }
```

**Exit code:** `0`

---

### 3. Run Citation Sync Preview (Exact Match)

**Purpose:** Exercise the citation sync engine in preview mode using the Zotero
exact-match fixture. Tests the simplest sync path: one CSL reference that matches
exactly one Zotero item.

**Command:**
```text
sourceright citation-sync --preview --remote-fixture fixtures/providers/zotero/preview-exact-match.json .sourceright
```

**Expected JSON output (abridged):**
```json
{
  "schema_version": "sourceright.citation_sync.v1",
  "mode": "dry_run",
  "remote_source": "fixture",
  "plan": {
    "items": [ ... ],
    "summary": {
      "total": 1,
      "create": 0,
      "update": 0,
      "noop": 1,
      "error": 0
    }
  }
}
```

**Key assertions:**
- `schema_version` is `sourceright.citation_sync.v1`
- `mode` is `dry_run`
- `remote_source` is `fixture`
- `plan.summary.total` is `1`
- The plan item shows a `noop` action (item already matched)

**Exit code:** `0`

---

### 4. Run Citation Sync Preview (Title Update)

**Purpose:** Test that the engine detects when a Zotero item has a different
title than the CSL reference (requires update).

**Command:**
```text
sourceright citation-sync --preview --remote-fixture fixtures/providers/zotero/preview-title-update.json .sourceright
```

**Expected JSON output (abridged):**
```json
{
  "schema_version": "sourceright.citation_sync.v1",
  "mode": "dry_run",
  "plan": {
    "summary": {
      "total": 1,
      "noop": 0,
      "update": 1,
      "create": 0,
      "error": 0
    }
  }
}
```

**Key assertions:**
- `plan.summary.update` is `1`
- The plan item has action `update` and includes a `diff` field

**Exit code:** `0`

---

### 5. Run Citation Sync Apply

**Purpose:** Test the apply path with a success-preview fixture.

**Command:**
```text
sourceright citation-sync --apply --remote-fixture fixtures/providers/zotero/apply-success-preview.json --audit-log /tmp/audit.jsonl .sourceright
```

**Expected JSON output (abridged):**
```json
{
  "schema_version": "sourceright.citation_sync.v1",
  "mode": "apply",
  "plan": {
    "summary": {
      "total": 1,
      "noop": 0,
      "update": 1,
      "create": 0,
      "error": 0
    }
  }
}
```

**Key assertions:**
- `mode` is `apply`
- `plan.summary.update` is `1`
- An audit log file is created at `/tmp/audit.jsonl`

**Exit code:** `0`

---

### 6. Verify Audit Log

**Command:**
```text
jq -r '.action' /tmp/audit.jsonl | sort | uniq -c
```

**Expected output:**
```
1 update
```

**Exit code:** `0`

---

### 7. MCP Surface: CLI Citation-Sync in Read-Only Surfaces

**Command:**
```text
sourceright mcp status --json | jq '.implemented_read_only_surfaces[] | select(contains("citation-sync"))'
```

**Expected output:**
```
"sourceright citation-sync --preview [.sourceright-directory]"
```

**Exit code:** `0`

---

### 8. Audit Log Verification After Apply

**Command:**
```text
jq -s 'length' /tmp/audit.jsonl
```

**Expected output:** A positive integer (>= 1 line)

**Exit code:** `0`

---

## Transcript Template

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "=== Citation-Manager Proof Transcript ==="
WORKSPACE=$(mktemp -d)
cd "$WORKSPACE"

echo "1. Init workspace"
sourceright init
test -d .sourceright || { echo "FAIL"; exit 1; }

echo "2. Run citation-sync preview (exact match)"
REPO_ROOT="${1:-.}"
PREVIEW=$(sourceright citation-sync --preview \
  --remote-fixture "$REPO_ROOT/fixtures/providers/zotero/preview-exact-match.json" .sourceright)
echo "$PREVIEW" | jq -e '.schema_version == "sourceright.citation_sync.v1"' > /dev/null
echo "$PREVIEW" | jq -e '.mode == "dry_run"' > /dev/null
echo "$PREVIEW" | jq -e '.plan.summary.total == 1' > /dev/null
echo "  Exact match: PASS"

echo "3. Run citation-sync preview (title update)"
PREVIEW2=$(sourceright citation-sync --preview \
  --remote-fixture "$REPO_ROOT/fixtures/providers/zotero/preview-title-update.json" .sourceright)
echo "$PREVIEW2" | jq -e '.plan.summary.update >= 1' > /dev/null
echo "  Title update: PASS"

echo "4. Run citation-sync apply"
APPLY=$(sourceright citation-sync --apply \
  --remote-fixture "$REPO_ROOT/fixtures/providers/zotero/apply-success-preview.json" \
  --audit-log /tmp/citation-audit.jsonl .sourceright)
echo "$APPLY" | jq -e '.mode == "apply"' > /dev/null
test -f /tmp/citation-audit.jsonl || { echo "FAIL: audit log missing"; exit 1; }
echo "  Apply: PASS"

echo "5. Verify MCP tool surface"
sourceright mcp status --json | jq -e '.implemented_read_only_surfaces[] | contains("citation-sync")' > /dev/null
sourceright mcp status --json | jq -e '.implemented_read_only_surfaces[] | contains("citation-sync")' > /dev/null
echo "  MCP surface: PASS"

echo "=== ALL PASSED ==="
rm -rf "$WORKSPACE" /tmp/citation-audit.jsonl
```

## Overclaim Guard

Must not claim:

- "Live Zotero/EndNote instance integration" — proof uses synthetic fixtures
- "EndNote fixture-backed testing" — no EndNote fixtures exist
- "Citation manager plugin deployed" — both Zotero/EndNote are `planned_adapter`

Only claim: **"Fixture-backed citation sync engine produces correct
`sourceright.citation_sync.v1` output for three Zotero fixture scenarios:
exact match, title update, and apply success."**

## Proof Family Status

| Surface | Fixture | CI Coverage | Evidence | Status |
|---------|---------|-------------|----------|--------|
| Zotero exact-match | `fixtures/providers/zotero/preview-exact-match.json` plus unit fixture `zotero-exact-match.json` | ✅ `citation_sync.rs` tests and manual workflow fixture smoke | ✅ This doc | ✅ Proven |
| Zotero title-update | `fixtures/providers/zotero/preview-title-update.json` plus unit fixture `zotero-title-update.json` | ✅ `citation_sync.rs` tests and manual workflow fixture smoke | ✅ This doc | ✅ Proven |
| Zotero apply success | `fixtures/providers/zotero/apply-success-preview.json` | ✅ `cli_end_to_end.rs` apply surface and documented transcript | ✅ This doc | ✅ Proven |
| `sourceright citation-sync` CLI surface | N/A | ✅ `cli_end_to_end.rs` | ✅ This doc | ✅ Proven |
| `sourceright citation-sync --apply` CLI surface | N/A | ✅ `cli_end_to_end.rs` | ✅ This doc | ✅ Proven |
| EndNote live sync | N/A | N/A (opt-in only) | 🔄 Missing | ⏸️ Deferred |

The CLI citation-sync surface and Zotero fixture-backed engine are proven via
CI and the manual Zotero workflow fixture job. EndNote live sync is deferred
pending fixture creation and adapter implementation.
