# OJS Package Lint Evidence

Date: 2026-05-18

## Command

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts\ojs-plugin-lint.ps1
```

## Result

The OJS package lint completed successfully.

- Archive: `C:\tmp\sourceright-ojs-lint\packages\sourceright-ojs-generic-plugin-0.1.0.tar.gz`
- SHA-256 sidecar: `C:\tmp\sourceright-ojs-lint\packages\sourceright-ojs-generic-plugin-0.1.0.tar.gz.sha256`
- Plugin install path: `plugins/generic/sourceright`
- Archive entries: `ok`
- XML validation: `powershell-xml`
- PHP lint: `skipped: php not on PATH`
- Rust policy tests: `passed`

## Boundary

This proves the repo-local OJS source package and lint path. It does not prove
live OJS compatibility, Docker install smoke, PKP Plugin Gallery submission, or
PKP acceptance.
