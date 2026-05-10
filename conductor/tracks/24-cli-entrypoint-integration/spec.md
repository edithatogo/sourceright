# CLI Entrypoint Integration Spec

## Goal

Make the newly added `bench` and `citation-sync` entrypoints first-class CLI surfaces and bring the repo documentation into alignment with the runtime commands.

## Scope

- Expose `bench` and `citation-sync` through the main CLI help and usage surfaces.
- Ensure command naming, argument help, and examples are consistent across docs and runtime output.
- Perform a final repo-wide consistency pass for the new entrypoints and their supporting docs.
- Preserve the existing separation between canonical CSL, verification sidecar data, and derived review artifacts.
- Keep the work local and deterministic by default.

## Outputs

- CLI help and command routing updates for the new entrypoints.
- Documentation updates that describe the new commands as first-class surfaces.
- Consistency checks for command names, examples, and references.

## Boundaries

This track must not change the benchmark runner semantics or citation-sync runtime behavior beyond CLI surface integration and documentation alignment.

It must not introduce live network requirements, new provider mutations, or silent writes to canonical bibliographic data.
