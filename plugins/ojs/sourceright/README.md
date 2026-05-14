# Sourceright OJS Generic Plugin

This is a thin Open Journal Systems generic plugin skeleton for calling the
existing Sourceright CLI/MCP core from OJS editorial workflows.

The plugin does not implement reference screening itself. It shells out to a
configured `sourceright` executable and keeps write-capable operations behind
preview or explicit configuration boundaries.

## Install

1. Copy this directory to an OJS installation as
   `plugins/generic/sourceright`.
2. Ensure the Sourceright CLI is installed on the OJS server.
3. Configure the plugin setting `sourcerightCliPath` if the executable is not
   available as `sourceright` on the web server path.
4. Enable the generic plugin in the OJS administration plugin list.

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
