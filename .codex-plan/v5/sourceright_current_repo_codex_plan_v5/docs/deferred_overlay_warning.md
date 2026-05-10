# Warning about older overlays

Older overlay ZIPs were created before the repository gained many implemented modules. They may contain stale assumptions such as:

- converting the repo immediately to a multi-crate workspace;
- adding placeholder modules that now already exist;
- replacing CLI/MCP scaffolding that may have since been implemented;
- copying schema examples that may not match the current Rust structs;
- treating legal/provenance/reporting as future work when it now appears in the current repo.

Use old overlays only as design notes. Do not bulk-apply them.
