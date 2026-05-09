# CSL Canonical Model Plan

1. Choose schema validation strategy for CSL JSON. Completed through serde-compatible CSL structures plus deterministic validator diagnostics.
2. Define minimum supported reference item types. Completed with a supported CSL item-type set that includes academic, web, report, and legal-adjacent CSL types.
3. Implement read/write with stable formatting. Completed through `parse_csl_json` and `format_csl_json`.
4. Add validation diagnostics. Completed with stable codes for required fields, duplicate normalized IDs, unsupported/non-canonical types, titles, DOIs, and sidecar-boundary violations.
5. Add migration hooks for records created by earlier versions. Completed through `migrate_csl_json` and `migrate_csl_document`.
