# FAQ

## Is Sourceright a final reference verifier?

No. It is a technical preview for structured reference triage. It helps validate
canonical CSL, keep provider evidence separate, generate review queues, and
export clean citation-manager formats. A human reviewer still owns final
judgement.

## Does provider evidence overwrite CSL?

No. Provider candidates, conflicts, confidence, and provenance belong in the
verification sidecar. Canonical CSL changes should come from explicit review,
not silent provider replacement.

## Are the benchmarks externally comparable?

No. The benchmark harness is a deterministic, fixture-backed regression suite.
It is useful for local quality gates and maturity tracking, not public ranking.

## Are legal citations part of the CSL model?

No. Legal citation extraction and review stay separate from academic CSL so
jurisdiction-specific handling does not contaminate bibliographic exports.

## Can the MCP server write files?

The intended MCP posture is read-first. Write-capable tools must use dry-run
plans by default and require explicit apply semantics before mutation.

## What should a pilot user test first?

Start with a small known reference set, run validation and reporting, inspect
the review queue, preview exports, and record where the workflow still requires
manual judgement.
