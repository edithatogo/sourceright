# Spec 2: Grype release dependency gates

Problem: Cargo metadata is labelled as an SBOM, an independent crate publishing
workflow can bypass release.yml, and the OCI image receives production tags
before any vulnerability check.

Acceptance: exact Cargo.lock CycloneDX inventory, critical gate in both crate
paths, candidate image digest scanned before tag promotion without rebuilding,
reports on failure, preserved existing audits/attestations, and advisory PR scans.
No runtime or bibliography contract changes.
