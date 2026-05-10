---
title: CLI
description: Command-line entrypoints for validation, reporting, export, and MCP status.
---

The CLI is the primary local operator surface.

- `sourceright --help` prints the command family.
- `sourceright mcp status --json` reports the server state and manifest metadata.
- `sourceright bench --json --manifest sourceright-bench/tasks.yaml` runs the benchmark manifest.
- `sourceright citation-sync` previews or applies sync plans with audit logging.
