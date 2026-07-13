# Quickstart

Sourceright is a technical preview for reference verification and related
workflow support. The fastest way to get a useful local loop is to start from a
checked-out workspace, validate the canonical CSL file, inspect the sidecar,
and then export clean outputs only after the queue is clear.

## 1. Start with a workspace

```text
sourceright init [.sourceright-directory]
```

If you are using the checked-in sample data, start from
`examples/workspace/`. That fixture-backed workspace is small enough for local
experimentation and documentation work.

## 2. Validate the canonical CSL file

```text
sourceright validate-csl references.csl.json
```

This checks the clean CSL boundary first. The canonical file should not contain
provider evidence, review notes, or other Sourceright workflow state.

## 3. Inspect verification and review state

```text
sourceright report [.sourceright-directory]
sourceright review queue [.sourceright-directory]
```

`report` gives the reference-integrity summary. `review queue` shows the
records that still need attention. Commands that read a workspace accept either
the `.sourceright` directory itself or its parent project directory, so
`sourceright report --json examples/workspace` works against the checked-in
sample data.

## 4. Export only from clean canonical data

```text
sourceright export --preview --all [.sourceright-directory]
sourceright export --all [.sourceright-directory]
```

Use preview mode first. It shows the output set without writing files, which is
useful when you are checking whether the workspace is ready for a downstream
tool.

## 5. Choose the next workflow

- Author preflight: [Author Preflight Workflow](author-preflight-workflow.md)
- Editorial triage: [Editorial Triage Workflow](editorial-triage-workflow.md)
- Repository deposition: [University Repository Workflow](university-repository-workflow.md)
- Legal citations: [Legal Citation Mode Workflow](legal-citation-mode-workflow.md)
- Artifact contracts: [Artifact and Schema Guide](artifact-schema-guide.md)
- Provider setup: [Live Provider Configuration Guide](live-provider-configuration.md)
- Scope limits: [Limitations](limitations.md)
