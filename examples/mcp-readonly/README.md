# Sourceright MCP Read-Only And Dry-Run Examples

This directory shows the local stdio wiring for the checked-in MCP surface. It
is an example transcript only; it does not run a live server or mutate files by
itself.

- `stdio-config.json` is the minimal client launch config.
- `transcript.md` shows the initialize, inspect, dry-run, and explicitly applied
  write pattern.

The operating rule is:

- read-only tools can be called directly;
- `workspace.init`, `review.import_decisions`, and `exports.write` default to
  dry-run planning;
- only set `apply: true` when you actually want a mutation.
