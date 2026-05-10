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
- `sourceright mcp`
- `sourceright mcp status`
- `sourceright mcp tools|resources|prompts`

`init` creates or confirms the local Sourceright workspace layout and prints the workspace path. `validate-csl` validates canonical CSL JSON input and returns deterministic diagnostics suitable for agents and CI. `validate-csl --json` emits a compact machine-readable envelope with `ok`, `path`, and `diagnostics` fields. `report` produces a reference integrity report that can identify AI-related citation-error signals without claiming authorship or intent; `report --json` and `report --mcp-resource` expose the same report through machine-readable envelopes. `conflicts` explains deterministic provider merge decisions. `citations` reconciles manuscript citations against reference-list entries. `review` inspects and imports manual review work. `journal-screen` produces a platform-neutral editorial screening report. `legal` extracts legal citation records into a separate legal model. `provenance` builds a claim/source graph without asserting claim truth. `policy` evaluates deterministic style and recency checks against canonical CSL JSON. `export` writes clean XML, ENW, RIS, BibLaTeX, and YAML outputs from the workspace CSL file, or prints a dry-run export manifest with `--preview`.

`mcp` remains a placeholder entry point for the future local MCP server. Plain `sourceright mcp` prints the current MCP status but exits non-zero because it does not start a server. `sourceright mcp status` and `sourceright mcp --status` print the same honest status output and exit successfully for scripts that need to check readiness. `sourceright mcp status --json` and `sourceright mcp --json` print a compact machine-readable readiness envelope. `sourceright mcp tools --json`, `sourceright mcp resources --json`, and `sourceright mcp prompts --json` print compact copies of the checked-in MCP manifests.

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

## `citations` contract

Usage:

```text
sourceright citations <manuscript.txt> [.sourceright-directory]
```

The command reads manuscript text, detects initial author-date and numeric citation forms, matches them against workspace CSL references, and prints a Markdown report covering missing references, uncited references, duplicate citations, ambiguous author matches, and numeric-order issues.

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

Supported platform labels are `generic-webhook`, `ojs`, `scholarone`, `editorial-manager`, `ejournalpress`, and `manuscript-manager`. The command emits `sourceright.journal_screening.v1` JSON for editorial workflow adapters.

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
