# Plan 2: Grype release gates

1. Generate resolved CycloneDX with the pinned shared lockfile helper, then run
   the pinned shared Grype policy in release.yml and publish-crate.yml.
2. Retain and explicitly hand off the inventory to the crate artifact job; make
   packaging and OCI depend on the release gate.
3. Push an isolated uniquely named image candidate, scan its immutable digest,
   then promote that digest to version/latest tags without rebuilding.
4. Add advisory source scans and document broad lockfile scope; verify workflow
   syntax, dependency reachability, promotion order and current lock parsing.

Only scoped branches/PRs are delivered. No releases or registries are published.
