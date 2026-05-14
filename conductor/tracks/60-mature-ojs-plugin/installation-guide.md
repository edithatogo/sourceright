# OJS Integration Installation Guide

> Generated: 2026-05-14
> Architecture: CLI/MCP Service Model
> Gallery readiness tracked separately: gallery-readiness.md
---

## 1. Architecture Decision

Sourceright integrates with Open Journal Systems (OJS) as a **CLI / MCP service**, not as a traditional OJS PHP plugin. The reference screening engine is a Rust binary (`sourceright`) that operates independently of OJS.

```
+----------------------+     CLI / MCP / HTTP      +---------------------+
|   OJS 3.x Instance   | ----------------------->  | Sourceright Service  |
|                      | <-----------------------  | (Rust / CLI / MCP) |
|  - Submission        |    screening report        | - Reference intake  |
|  - Editor dashboard  |    (JSON + Markdown)       | - Provider verify   |
|  - Author workflow   |                            | - Report generate   |
+----------------------+                            +---------------------+
```

### 1.1 Why Not a PHP Plugin?

| Factor | Decision | Rationale |
|--------|----------|-----------|
| Screening engine | Rust binary | Mature engine in src/journal.rs with JournalPlatform::Ojs |
| Plugin packaging | CLI/MCP service | No PHP plugin directory; service runs independently |
| PKP Gallery | Tracked separately | gallery-readiness.md documents 12 gaps and Path A/B |
| Platform compat | OJS 3.x, any env | Binary works on Linux/macOS/Windows |


### 1.2 Integration Methods

| Method | Description | Status |
|--------|-------------|--------|
| **CLI integration** | OJS calls sourceright journal-screen via exec() | ✅ CLI exists; wiring documented |
| **MCP tool invocation** | OJS calls MCP tool (sourceright mcp) | ✅ MCP exists; docs below |
| **REST API gateway** | OJS sends JSON to HTTP endpoint | 🔧 Not packaged |
| **Direct PHP library** | PHP wrapper calling Rust | ❌ Not created |
| **OJS webhook receiver** | Sourceright listens for webhooks | ❌ Not created |

---

## 2. Installation

### 2.1 Prerequisites

- **OJS 3.3+** running on a server (Linux recommended; macOS and Windows supported)
- **Sourceright binary** on the same server or a network-accessible host
- **Workspace directory** with CSL reference data and verification sidecar (see section 3)
- **PHP** on the OJS server (for CLI exec() or HTTP client calls)

### 2.2 Obtain the Sourceright Binary

#### Option A: Download from GitHub Releases (Recommended)

```bash
# Linux (x86_64)
curl -L -o sourceright https://github.com/edithatogo/sourceright/releases/latest/download/sourceright-linux-x86_64
chmod +x sourceright
sudo mv sourceright /usr/local/bin/
```

```powershell
# Windows (PowerShell)
Invoke-WebRequest -Uri "https://github.com/edithatogo/sourceright/releases/latest/download/sourceright-windows-x86_64.exe" -OutFile "sourceright.exe"
```

#### Option B: Build from Source

```bash
git clone https://github.com/edithatogo/sourceright.git
cd sourceright
cargo build --release
sudo cp target/release/sourceright /usr/local/bin/
```

#### Option C: Install via Cargo

```bash
cargo install sourceright
```

### 2.3 Verify Installation

```bash
sourceright --version
sourceright journal-screen --help
```

Expected output shows subcommands and the `--platform ojs` flag.

### 2.4 Configure the Workspace

```bash
mkdir -p /var/sourceright/workspace
cd /var/sourceright/workspace
cp /path/to/your/references.json ./references.csl.json
echo '{"$schema":"sourceright.sidecar.v1","references":{}}' > references.verification.json
```

---

## 3. OJS Integration

### 3.1 Method 1: CLI Integration (Recommended)

Add a custom workflow step or PHP script in OJS that calls the Sourceright CLI.

```php
<?php
/**
 * Call Sourceright screening from OJS.
 * Place in a custom theme, hook, or editor dashboard page.
 */
$submissionId = $submission->getId();
$workspaceRoot = '/var/sourceright/workspace';
$manuscriptFile = '/var/www/ojs/files/submissions/' . $submissionId . '/manuscript.docx';

$command = sprintf(
    'sourceright journal-screen --workspace %s --submission-id %s --platform ojs --manuscript-label %s',
    escapeshellarg($workspaceRoot),
    escapeshellarg('OJS-SUB-' . $submissionId),
    escapeshellarg(basename($manuscriptFile))
);

$output = shell_exec($command . ' 2>&1');
$report = json_decode($output, true);
$editorialSummary = $report['editorial_summary'] ?? 'No screening available';
$authorChecklist = $report['author_action_checklist'] ?? [];
$status = $report['status'] ?? 'unknown';
```

### 3.2 Method 2: MCP Tool Integration

Run Sourceright in MCP server mode and invoke the `journal_screen` tool.

**Start the MCP server:**

```bash
sourceright mcp --workspace /var/sourceright/workspace
```

**Invoke the screening tool:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "journal_screen",
    "arguments": {
      "submission_id": "OJS-SUB-2025-0042",
      "platform": "ojs",
      "manuscript_label": "manuscript.docx"
    }
  }
}
```

**Response includes:**

```json
{
  "schema_version": "sourceright.journal_screening.v1",
  "submission_id": "OJS-SUB-2025-0042",
  "platform": "ojs",
  "status": "screened_with_warnings",
  "editorial_summary": "Submission (manuscript.docx) screened: 5 references, 5 issues, 4 AI-risk signals.",
  "author_action_checklist": [
    "Review missing identifiers, unverified references, and provider conflicts.",
    "Resolve references currently queued for manual review."
  ]
}
```

### 3.3 Method 3: File-Based Batch Workflow

For shared hosting or environments where direct CLI calls are not feasible:

1. OJS exports submission metadata and CSL references to the workspace
2. A cron job runs `sourceright journal-screen` periodically
3. Screening reports are written to a reports directory
4. OJS reads the reports for display

```bash
# Cron job: every 10 minutes
*/10 * * * * /usr/local/bin/sourceright journal-screen \
  --workspace /var/sourceright/workspace \
  --platform ojs \
  --output /var/sourceright/reports/
```

---

## 4. Permission Boundaries

### 4.1 OJS Role-Based Access

Sourceright operates **outside** OJS's PHP runtime and does not participate in OJS role-based access control (RBAC). Integration code is responsible for enforcing visibility.

| OJS Role | Screening Access | Notes |
|----------|-----------------|-------|
| Journal Manager | Full: run screening, view reports | Can trigger screening and see all results |
| Editor | Full: run screening, view reports | Can trigger screening and see all results |
| Section Editor | Assigned submissions only | Limited via OJS workflow |
| Author | Checklist only (when enabled) | Sees author_action_checklist, not full report |
| Reviewer | None (by default) | Screening is editorial, not peer review |
| Reader / Public | None | No screening data exposed |

### 4.2 Data Boundaries

| Data | Read | Write | Persistence |
|------|------|-------|-------------|
| CSL references (references.csl.json) | Sourceright reads canonical data | OJS writes via export | Workspace filesystem |
| Verification sidecar (references.verification.json) | Sourceright reads provider evidence | Sourceright writes during verification | Workspace filesystem |
| Review queue (review-queue.jsonl) | Sourceright reads/writes | OJS reads | Workspace filesystem |
| Screening reports | Sourceright returns JSON | Stored by OJS integration | Configurable |
| Manuscript text | Provided by OJS integration | Read-only by Sourceright | Not persisted by Sourceright |

### 4.3 Privacy and Processing

1. OJS passes submission metadata and references to Sourceright via CLI/MCP
2. All processing is local — no submission data leaves the server
3. Manuscript full-text is not retained after screening
4. Reports use excerpt minimization: roles see only their authorized subset

---

## 5. Configuration Reference

### 5.1 CLI Flags for OJS Integration

```text
sourceright journal-screen [OPTIONS]
  -w, --workspace <PATH>       Workspace root directory [default: .]
  -s, --submission-id <ID>     Submission identifier
  -p, --platform <PLATFORM>    Journal platform [default: generic-webhook]
                               Supported: ojs, scholarone, editorial-manager,
                               ejournalpress, manuscript-manager, generic-webhook
  -m, --manuscript-label <STR> Manuscript file label [default: manuscript]
  -o, --output <PATH>          Output directory for report JSON
  -j, --json                   Output as JSON
  --help                       Print help
```

### 5.2 Environment Variables

| Variable | Purpose | Required |
|----------|---------|----------|
| `SOURCERIGHT_WORKSPACE` | Default workspace path | No |
| `SOURCERIGHT_LOG` | Log level (error, warn, info, debug) | No (default: warn) |
| `SOURCERIGHT_CONFIG` | Path to config file | No |

### 5.3 Workspace File Layout

```
/var/sourceright/workspace/
+-- references.csl.json          # Canonical CSL bibliographic data
+-- references.verification.json # Provider verification evidence
+-- review-queue.jsonl           # Operational review queue
+-- reports/                     # Generated screening reports
+-- pending/                     # Batch workflow pending submissions
```

---

## 6. Current Status and Limitations

### 6.1 What Works

| Capability | Status |
|------------|--------|
| CLI `journal-screen` with `--platform ojs` | ✅ Technical preview |
| MCP `journal_screen` tool | ✅ Technical preview |
| Editorial summary generation | ✅ Tested with fixtures |
| Author action checklist generation | ✅ Tested with fixtures |
| 5-reference OJS fixture (ojs-submission.json) | ✅ Covers verified, conflicted, retracted, queued, missing DOI |
| Platform-neutral screening contract | ✅ JournalPlatform::Ojs serialized in reports |
| CSL validation and sidecar model | ✅ Fixture-backed technical preview |

### 6.2 Current Limitations

| Limitation | Impact | Workaround |
|------------|--------|------------|
| No PHP plugin directory | No PKP Gallery submission | Use CLI/MCP integration (this guide) |
| No OJS hook registration | Must use custom theme/script | Implement integration in OJS template |
| No live OJS test instance smoke | Not tested against real OJS 3.x | Manual testing on target OJS instance |
| No REST/HTTP endpoint | MCP requires stdio transport | Use CLI method or add HTTP adapter |
| No uninstall logic | Manual workspace cleanup | Delete workspace directory |
| No localisation (i18n) | Reports are English-only | N/A for initial deployment |

### 6.3 Gallery Readiness

PKP/OJS Plugin Gallery submission is tracked separately in `gallery-readiness.md`. The Gallery readiness assessment identifies 12 gaps. **Recommendation**: The CLI/MCP service model is pilot-ready for controlled editorial workflow trials. Gallery submission (Path A: PHP wrapper + CLI) can proceed when a thin PHP plugin is added.

### 6.4 Roadmap

| Future Work | Track | Priority |
|-------------|-------|----------|
| PHP plugin wrapper (Path A) | Track 60 | Medium |
| REST/HTTP endpoint | Track 60 | Medium |
| OJS test-instance smoke script | Track 45 | Low |
| PKP Gallery submission | Track 63 | Low |
| Localisation / i18n | Track 60 | Low |

---

## 7. Troubleshooting

### 7.1 Binary Not Found

```bash
which sourceright
sourceright --version
```

### 7.2 Workspace Errors

```bash
ls -la /var/sourceright/workspace/
# Expected: references.csl.json, references.verification.json
```

### 7.3 Permission Errors

The user running screening must have read access to the workspace and write access to output. On shared OJS hosting, ensure the web server user (e.g., `www-data`) can execute the binary:

```bash
sudo chmod +x /usr/local/bin/sourceright
sudo chown -R www-data:www-data /var/sourceright/
```

### 7.4 No Output / Empty Report

If `sourceright journal-screen` returns an empty or blocked report:
1. Verify CSL JSON has valid references
2. Check verification sidecar has entries matching CSL IDs
3. Run with `SOURCERIGHT_LOG=debug` for verbose output
4. Check the manuscript label matches an expected file

---

## 8. Uninstallation

1. **Stop** any MCP server process
2. **Remove binary**: `sudo rm /usr/local/bin/sourceright`
3. **Remove workspace**: `rm -rf /var/sourceright/`
4. **Remove OJS integration code** (custom theme modifications, PHP scripts)
5. **Remove cron jobs** referencing `sourceright journal-screen`

No database changes needed — Sourceright stores all data in the workspace filesystem.

---

## 9. Related Documentation

| Document | Location | Purpose |
|----------|----------|---------|
| Gallery readiness | `conductor/tracks/60-mature-ojs-plugin/gallery-readiness.md` | PKP Plugin Gallery gaps and recommendations |
| Test matrix update | `conductor/tracks/60-mature-ojs-plugin/test-matrix-update.md` | OJS fixture test scenarios |
| Review | `conductor/tracks/60-mature-ojs-plugin/review.md` | Track status and completion evidence |
| Journal integrations | `docs/src/journal-integrations.md` | Platform-neutral integration contract |
| General installation | `docs/src/installation.md` | Sourceright binary installation |
| Plugin manifest | `plugins/manifests/journal.ojs.toml` | Declares planned_adapter status |
| OJS submission fixture | `fixtures/journal/ojs-submission.json` | 5-reference screening test fixture |

---

## 10. Revision History

| Date | Change |
|------|--------|
| 2026-05-14 | Initial installation guide — CLI/MCP service model, integration methods, permission boundaries, current limitations |
