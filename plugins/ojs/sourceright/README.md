# Sourceright OJS Generic Plugin

This is a thin Open Journal Systems generic plugin skeleton for calling the
existing Sourceright CLI/MCP core from OJS editorial workflows.

The plugin does not implement reference screening itself. It shells out to a
configured `sourceright` executable and keeps write-capable operations behind
preview or explicit configuration boundaries.

## Install

1. Build the install-test archive from the repository root:

   ```powershell
   powershell -NoProfile -ExecutionPolicy Bypass -File scripts/build-ojs-plugin-package.ps1
   ```

2. Copy this directory, or extract the generated archive, to an OJS installation as
   `plugins/generic/sourceright`.
3. From the OJS root, register the plugin version:

   ```bash
   php lib/pkp/tools/installPluginVersion.php plugins/generic/sourceright/version.xml
   ```

4. Ensure the Sourceright CLI is installed on the OJS server.
5. Configure the plugin setting `sourcerightCliPath` if the executable is not
   available as `sourceright` on the web server path.
6. Enable the generic plugin in the OJS administration plugin list.

## Runtime Boundary

- Screening runs `sourceright journal-screen --platform ojs`.
- Export integration uses `sourceright export --preview` by default.
- Write-capable flows must check `allowExplicitWrites` before adding any future
  non-preview command.
- The command path and all command arguments are escaped before execution.
- Author-facing output is read from `author_action_checklist`.
- Editor-facing output is read from `editorial_summary` and the full screening
  report payload.

## Required Sourceright Workspace

The configured workspace should contain the normal Sourceright files:

- `references.csl.json`
- `references.verification.json`
- `review-queue.jsonl` when review work has been generated

Provider evidence and review state stay in Sourceright sidecar files. The
plugin must not silently overwrite canonical CSL data.

## Current Status

This package is a source skeleton for Track 60. It is intended to be installable
as an OJS generic plugin source tree, but it is not PKP Plugin Gallery accepted.
It still needs live OJS handler, settings-form, and workflow-template wiring
before PKP Plugin Gallery review.

The repo-local install smoke path is documented in
`conductor/tracks/60-mature-ojs-plugin/ojs-install-smoke.md`.
For Docker-based preflight planning, run
`scripts/ojs-docker-install-smoke.ps1`.
For repo-local checks without Docker, run `scripts/ojs-plugin-lint.ps1`.
