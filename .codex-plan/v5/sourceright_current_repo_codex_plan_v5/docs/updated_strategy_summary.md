# Updated strategy summary

The original recommendation was to modularize aggressively and add a core + plugin structure. The repo has since implemented much of the core functionality inside a single Rust crate. The updated recommendation is therefore:

- keep the current single-crate implementation for now;
- add external contracts around it;
- add plugin manifests before runtime plugin loading;
- add demos and benchmarks that consume existing artifacts;
- add policy/recency/style work without destabilizing journal screening;
- add read-only MCP server contracts before write-capable tools;
- defer workspace splitting until APIs are stable.

## Updated SOTA path

```text
Current core
  -> schemas and validation contracts
  -> plugin manifest registry
  -> demo apps
  -> benchmark harness
  -> policy/style/recency checks
  -> citation-manager profiles
  -> provider fixture expansion
  -> read-only MCP server
  -> eventual plugin runtime/workspace split
```
