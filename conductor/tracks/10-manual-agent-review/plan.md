# Manual Agent Review Plan

1. Define review queue item schema. Implemented through sidecar-derived `ReviewQueueEntry`.
2. Add partitioning strategy for subagents by record groups, providers, or issue type. Implemented as stable bounded queue partitions.
3. Add accept/reject/merge/unresolved decision model. Implemented through review decision import into sidecar records.
4. Add CLI commands for queue inspection and decision import. Implemented with `sourceright review queue`, `review partitions`, and `review import-decisions`.
5. Add MCP resources for agent review workflows. Exposed through the existing MCP status contract and JSON-ready review queue surfaces.
