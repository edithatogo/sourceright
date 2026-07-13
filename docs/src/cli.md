# CLI Plan

The initial Rust binary is planned around a small, stable command surface first:

- `sourceright init`
- `sourceright validate-csl`
- `sourceright report`
- `sourceright conflicts`
- `sourceright citations`
- `sourceright review`
- `sourceright journal-screen`
- `sourceright legal`
- `sourceright provenance`
- `sourceright policy`
- `sourceright export`
- `sourceright plugins`
- `sourceright bench`
- `sourceright citation-sync`
- `sourceright mcp`
- `sourceright mcp status`
- `sourceright mcp tools|resources|prompts`

`init` creates or confirms the local Sourceright workspace layout and prints the workspace path. `validate-csl` validates canonical CSL JSON input and returns deterministic diagnostics suitable for agents and CI. `validate-csl --json` emits a compact machine-readable envelope with `ok`, `path`, and `diagnostics` fields. `report` produces a reference integrity report that can identify AI-related citation-error signals without claiming authorship or intent; `report --json` and `report --mcp-resource` expose the same report through machine-readable envelopes. `conflicts` explains deterministic provider merge decisions. `citations` reconciles manuscript citations against reference-list entries and accepts plain text or DOCX manuscripts. `review` inspects and imports manual review work. `journal-screen` produces a platform-neutral editorial screening report. `legal` extracts legal citation records into a separate legal model. `provenance` builds a claim/source graph without asserting claim truth. `policy` evaluates deterministic style and recency checks against canonical CSL JSON. `export` writes clean XML, ENW, RIS, BibLaTeX, and YAML outputs from the workspace CSL file, or prints a dry-run export manifest with `--preview`. `bench` runs deterministic fixture-backed benchmark tasks. `citation-sync` previews or applies Zotero-first citation-manager sync plans with audit logging and conflict reporting.

`mcp` starts the local MCP server. `sourceright mcp` brings up the server runtime, while `sourceright mcp status` and `sourceright mcp --status` remain read-only status checks for scripts that need to inspect readiness without starting a server. `sourceright mcp status --json` and `sourceright mcp --json` print a compact machine-readable readiness envelope. `sourceright mcp tools --json`, `sourceright mcp resources --json`, and `sourceright mcp prompts --json` remain read-only inspection surfaces. The server also exposes validated plugin discovery through `plugins.list` and `sourceright://plugins/registry`. The server also exposes dry-run write tools for workspace init, review decision import, and export writes; those tools default to plan mode and only mutate when `apply: true` is passed.

The public release path assumes `cargo package --locked` and
`cargo publish --dry-run --locked` have already passed before `cargo publish`
is invoked manually.

Each implemented command supports command-specific help:

- `sourceright init --help`
- `sourceright validate-csl --help`
- `sourceright report --help`
- `sourceright conflicts --help`
- `sourceright citations --help`
- `sourceright review --help`
- `sourceright journal-screen --help`
- `sourceright legal --help`
- `sourceright provenance --help`
- `sourceright policy --help`
- `sourceright export --help`
- `sourceright plugins --help`
- `sourceright bench --help`
- `sourceright citation-sync --help`
- `sourceright mcp --help`

The planned workflow command family remains:

- `sourceright extract`
- `sourceright normalize`
- `sourceright verify`
- `sourceright pipeline`

Commands that return structured data support deterministic JSON for the implemented surfaces. Human-readable output remains useful for local use, while CI and agent workflows can depend on stable machine-readable results, exit codes, and file paths. The CLI rejects unexpected extra arguments with command-specific usage hints.

## `validate-csl` contract

Usage:

```text
sourceright validate-csl [--json] <references.csl.json>
```

Human-readable output remains line-oriented:

- valid CSL input prints `valid` to stdout and exits `0`.
- readable CSL input with validation diagnostics prints one stable diagnostic per line as `<code> <path> <message>` and exits `1`.
- usage errors, unreadable files, and JSON parse errors print an error to stderr and exit `2`.

Machine-readable output is enabled with `--json`:

```json
{"ok":false,"path":"references.csl.json","diagnostics":[{"code":"csl.title.empty","path":"$[0].title","message":"CSL item title must not be empty"}]}
```

The `path` field is the caller-supplied file path, preserved as the CLI received it. Diagnostic ordering is the validator's stable traversal order.

## `report` contract

Usage:

```text
sourceright report [--json|--mcp-resource] [.sourceright-directory]
```

Default Markdown output renders the editor-facing audit report. `--json` emits compact `sourceright.reference_report.v1` JSON with summary counters and stable issue records. `--mcp-resource` wraps the JSON report as an MCP-ready resource envelope at `sourceright://reports/reference-integrity`.
Workspace-reading commands accept either the `.sourceright` directory itself or
its parent project directory when that parent contains `.sourceright`, for
example `sourceright report --json examples/workspace`.

## `citations` contract

Usage:

```text
sourceright citations <manuscript.txt|manuscript.docx> [.sourceright-directory]
```

The command reads manuscript text or a DOCX manuscript, detects initial author-date and numeric citation forms, matches them against workspace CSL references, and prints a Markdown report covering missing references, uncited references, duplicate citations, ambiguous author matches, and numeric-order issues. DOCX numeric markers are normalized from superscript runs before reconciliation.

## `review` contract

Usage:

```text
sourceright review queue [.sourceright-directory]
sourceright review partitions [--size <n>] [.sourceright-directory]
sourceright review import-decisions <decisions.json> [.sourceright-directory]
```

`queue` refreshes and prints `review-queue.jsonl`. `partitions` emits stable JSON partitions that can be handed to agents or subagents. `import-decisions` accepts a JSON array of review decisions, records them in `references.verification.json`, and refreshes the queue.

## `journal-screen` contract

Usage:

```text
sourceright journal-screen [--platform <platform>] [--submission-id <id>] [--manuscript <label>] [.sourceright-directory]
```

Supported platform labels are `generic-webhook`, `ojs`, `arxiv-submit-ce`,
`arxiv-submission-core`, `scholarone`, `editorial-manager`, `ejournalpress`, and
`manuscript-manager`. The command emits `sourceright.journal_screening.v1` JSON
for editorial workflow adapters.

## `legal` contract

Usage:

```text
sourceright legal <legal-text.txt>
```

The command emits compact JSON with legal citation records, jurisdiction/provider hints, and review issues. Legal records are not forced into CSL JSON.

## `provenance` contract

Usage:

```text
sourceright provenance <document-text.txt>
```

The command emits compact JSON with claim nodes, detected citation source nodes, claim/source links, and provenance issues. This report describes evidence linkage; it does not assert whether a claim is true.

## `policy` contract

Usage:

```text
sourceright policy [--policy <policy.json>] <references.csl.json>
```

The command emits compact `sourceright.policy_report.v1` JSON. Without
`--policy`, it uses the built-in `journal-vancouver` policy. The policy command
is local-file based and deterministic; it does not call providers, perform
semantic relevance checks, or score claim truth.

## `export` contract

Usage:

```text
sourceright export --format <format> [.sourceright-directory]
sourceright export --all [.sourceright-directory]
sourceright export --preview --format <format> [.sourceright-directory]
sourceright export --preview --all [.sourceright-directory]
```

Exports are opt-in. The command does not write files unless a single `--format` or explicit `--all` is supplied. Supported format names are `yaml`, `xml`, `ris`, `enw`, and `biblatex`. The command writes deterministic files into the workspace `exports` directory and prints the written paths. With `--preview`, it prints compact `sourceright.export_manifest.v1` JSON and does not create export files.

## `plugins` contract

Usage:

```text
sourceright plugins [validate] [--json]
```

The command discovers `plugins/registry.toml` and the manifests under
`plugins/manifests/`. It validates the manifest structure, reports provenance
metadata, and surfaces trust gating without executing plugin code. `--json`
prints compact `sourceright.plugin_registry_report.v1` JSON. `validate` exits
non-zero if the discovered registry contains invalid manifests.

## `bench` contract

Usage:

```text
sourceright bench [--json] [--manifest <tasks.yaml>]
sourceright bench [--json] <tasks.yaml>
```

The command defaults to `sourceright-bench/tasks.yaml`, runs fixture-backed
tasks, and compares outputs against checked-in baselines. It does not call live
providers, citation-manager APIs, or journal systems. Human-readable output is
a pass/fail summary; `--json` emits compact `sourceright.benchmark_run.v1`
JSON.

## `citation-sync` contract

Usage:

```text
sourceright citation-sync [--preview|--apply] [--remote-fixture <remote.json>] [--audit-log <audit.jsonl>] [.sourceright-directory]
```

The command defaults to `.sourceright` and preview mode. `--apply` is required
before audit logs or remote fixture updates are written. Live Zotero transport
is opt-in through `SOURCERIGHT_ZOTERO_API_URL`,
`SOURCERIGHT_ZOTERO_API_KEY`, `SOURCERIGHT_ZOTERO_LIBRARY_ID`, and optional
`SOURCERIGHT_ZOTERO_LIBRARY_TYPE`. Conflicts are reported in
`sourceright.citation_sync.v1` JSON and do not silently overwrite canonical
CSL data.
