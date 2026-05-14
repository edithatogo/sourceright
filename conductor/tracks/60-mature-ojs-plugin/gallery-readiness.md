# OJS / PKP Plugin Gallery Readiness

## 1. Purpose

This document tracks readiness for PKP Plugin Gallery submission.

---

## 2. Gallery Submission Requirements

| Requirement | Details | Status |
| Plugin entry point | index.php extending GenericPlugin | X Not created |
| Plugin XML metadata | version.xml | X Not created |
| Plugin descriptor | plugins/generic/sourceright/ | X Not created |
| Localisation | locale/ with locale.po | X Not created |
| Installation docs | README with install steps | X Not created |
| Gallery metadata | plugin.xml | X Not created |
| Compatibility | OJS 3.x range | X Not declared |
| License | PKP-approved OSS | OK Apache-2.0/MIT |
| Repository | Public source | OK GitHub |
| Release tag | SemVer tag | X Not created |
| No conflicts | Unique namespace | OK |

---

## 4. Plugin Gallery Review Criteria

Based on PKP Plugin Gallery review guidelines:

| Criterion | Description | Sourceright Assessment |
|---|---|---|
| **Documentation** | README with install, config, uninstall | X Not created |
| **Localisation** | At minimum English; i18n encouraged | X Not created |
| **Stability** | No fatal errors, PHP notices suppressed | X No PHP code |
| **OJS compatibility** | Tested against OJS 3.x stable | X Not tested |
| **No debug output** | No var_dump, print_r, error_log | N/A |
| **Security** | CSRF, input sanitisation, XSS prevention | N/A |
| **Performance** | No blocking DB queries | N/A |
| **Uninstall clean** | Removes DB tables/settings | X No logic |
| **License compliance** | Compatible OSS license | OK Apache-2.0/MIT |
| **Maintainer resp.** | Issue tracker active | OK GitHub Issues |
| **No obfuscation** | Human-readable source | OK Plaintext |
| **Releases via tag** | Gallery uses GitHub releases | X No release tag |

---

## 5. Current Integration Architecture

Sourceright OJS integration is a **CLI / MCP service**, not a traditional OJS PHP plugin:

`
+----------------------+     HTTP/JSON or CLI      +---------------------+
|   OJS 3.x Instance   | ----------------------->  | Sourceright Service  |
|                      | <-----------------------  | (Rust / MCP / CLI)  |
|  - Submission page   |    screening report        |                     |
|  - Editor dashboard  |    (JSON + Markdown)       | - Reference intake  |
|  - Author workflow   |                            | - Provider verify   |
+----------------------+                            | - Report generate   |
                                                    +---------------------+
`

### 5.1 Integration Methods

| Method | Description | Status |
|---|---|---|
| **CLI integration** | OJS calls sourceright journal-screen via exec() | OK CLI exists; OJS not wired |
| **REST API gateway** | OJS sends JSON to HTTP endpoint | X Not packaged |
| **MCP tool invocation** | OJS calls MCP tool | OK MCP exists; adapter not built |
| **Direct PHP library** | PHP wrapper calling Rust | X Not created |
| **OJS webhook receiver** | Sourceright listens for webhooks | X Not created |

### 5.2 Data Flow

1. Editor opens submission in OJS workflow
2. OJS plugin extracts or references manuscript files
3. Plugin calls Sourceright CLI/MCP with CSL + sidecar
4. Sourceright returns JournalScreeningReport
5. Plugin displays report in OJS and optionally stores it

---

## 6. Readiness Assessment

### 6.1 What Exists

| Component | Location | Notes |
|---|---|---|
| Plugin manifest | plugins/manifests/journal.ojs.toml | Declares planned_adapter |
| Screening engine | src/journal.rs (188 lines) | screen_journal_submission() |
| Platform variant | JournalPlatform::Ojs | Ready for dispatch |
| Editor summaries | editorial_summary field | Implemented |
| Author checklists | author_action_checklist field | Implemented |
| Reference report | ReferenceReportJsonOutput | Stable JSON schema |
| CSL validation | csl.rs | Fixture-backed technical preview |
| Sidecar model | Sidecar with candidates/conflicts | Fixture-backed technical preview |
| OJS fixture | fixtures/journal/ojs-submission.json | **NEW - 2026-05-14** |
| Gallery readiness | This document | **NEW - 2026-05-14** |

### 6.2 Gaps Blocking Gallery Submission

| # | Gap | Priority | Effort |
|---|---|---|---|
| 1 | No PHP plugin directory (plugins/generic/sourceright/) | Critical | Medium |
| 2 | No version.xml | Critical | Small |
| 3 | No index.php entry point | Critical | Small |
| 4 | No SourcerightPlugin.php main class | Critical | Medium |
| 5 | No locale files | High | Small |
| 6 | No gallery plugin.xml | High | Small |
| 7 | No README with install steps | High | Small |
| 8 | No release tag | Medium | Small |
| 9 | No OJS compatibility testing (3.3-3.5) | Medium | Medium |
| 10 | No REST/HTTP endpoint for OJS | Medium | Medium |
| 11 | No uninstall logic | Low | Small |
| 12 | No live smoke test | Low | Medium |

### 6.3 Effort Estimate

- **Minimal gallery-ready plugin** (gaps 1-7): ~2-3 days for PHP dev familiar with OJS API
- **Integration testing** (gap 9): ~1 day with OJS Docker instance
- **REST endpoint** (gap 10): ~2-3 days for HTTP server / webhook pattern
- **Full gallery submission**: ~1 week total with packaging, testing, docs, gallery PR

---

## 7. Submission Process

1. **Prepare plugin package**: Create plugins/generic/sourceright/ with all required files
2. **Tag a release**: git tag v0.1.0 && git push --tags
3. **Create GitHub release**: Add release notes pointing to this readiness doc
4. **Open gallery PR**: Fork pkp/plugin-gallery, add plugin entry, submit PR
5. **Address review**: PKP maintainers review; address issues
6. **Maintain release**: Update version.xml with each compatible release

### 7.1 plugin.xml Hosting

PKP Gallery fetches plugin.xml from a stable release URL:

`
https://raw.githubusercontent.com/edithatogo/sourceright/v0.1.0/plugins/generic/sourceright/plugin.xml
`

---

## 8. Decision: PHP Plugin vs. CLI/MCP Service

### Path A: PHP Plugin Wrapper (Recommended for Gallery)

- Thin PHP plugin calls Sourceright CLI via exec() or proc_open()
- Plugin handles display, permissions, OJS hook registration
- CLI handles screening engine, provider verification, report generation
- **Pro**: Fits PKP Gallery expectations; familiar install experience
- **Con**: Requires PHP + OJS plugin dev; Rust binary must be available on server

### Path B: MCP / HTTP Service Adapter

- Run Sourceright as a sidecar or HTTP service alongside OJS
- Minimal OJS plugin sends requests to the service endpoint
- **Pro**: Decouples screening from OJS; service can be shared across journals
- **Con**: Additional infrastructure; PKP Gallery may not accept without PHP plugin

### Recommendation

**Pursue Path A** for PKP Gallery submission. Rust CLI handles screening;
PHP plugin handles OJS integration. Matches PKP Gallery expectations.

---

## 9. Related Tracks

| Track | Relationship |
|---|---|
| 16 - Journal Workflow Integrations | Defines the platform integration contract |
| 45 - External Proof Suites | OJS test-instance smoke scripts live here |
| 63 - Plugin Packaging and Supply Chain | Plugin distribution, signing, hardening |
| 58 - Mature Zotero Plugin | Parallel plugin maturity effort; shares patterns |

---

## 10. Revision History

| Date | Change |
|---|---|
| 2026-05-14 | Initial document - gaps and requirements for PKP Plugin Gallery submission |

---

## 3. Plugin Packaging Requirements

### 3.1 Directory Structure

A complete OJS 3.x plugin should follow this structure:

`
plugins/generic/sourceright/
+-- index.php              # Plugin entry point
+-- SourcerightPlugin.php  # Main plugin class
+-- version.xml            # Version metadata for gallery
+-- README.md              # Install and config docs
+-- locale/
|   +-- en_US/
|       +-- locale.po      # English localisation
+-- src/                   # PHP helper classes
`

### 3.2 version.xml

`xml
<?xml version="1.0" encoding="UTF-8"?>
<version>
    <application>ojs</application>
    <type>plugins.generic</type>
    <package>sourceright</package>
    <release>0.1.0</release>
    <date>2026-05-14</date>
    <lazy-load>1</lazy-load>
    <sitewide>1</sitewide>
</version>
`

### 3.3 Gallery plugin.xml

`xml
<?xml version="1.0" encoding="UTF-8"?>
<plugin>
    <category>generic</category>
    <product>sourceright</product>
    <name><en>Reference Integrity Screening (Sourceright)</en></name>
    <summary><en>Integrates Sourceright reference screening into OJS editorial workflows.</en></summary>
    <description><en>Plugin integrates Sourceright with OJS submission workflows.</en></description>
    <release>0.1.0</release>
    <version>
        <application>ojs</application>
        <type>plugins.generic</type>
        <package>sourceright</package>
        <release>0.1.0</release>
        <date>2026-05-14</date>
    </version>
    <author><name>Sourceright Project</name></author>
    <homepage>https://github.com/edithatogo/sourceright</homepage>
    <license>Apache-2.0 OR MIT</license>
</plugin>
`

### 3.4 Plugin Entry Point (index.php)

`php
<?php
require_once('SourcerightPlugin.php');
return new SourcerightPlugin();
`

### 3.5 Main Plugin Class Hooks

| Hook | Purpose |
|---|---|
| LoadComponentHandler | Register screening report grid/page handler |
| Templates::Manager::Submissions::SubmissionGrid::Cell | Add screening status column |
| Template::Workflow::Submission | Display screening report in workflow |
| Schema::get::submission | Extend submission schema with screening status |
