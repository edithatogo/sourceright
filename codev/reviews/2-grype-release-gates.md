# Review 2: Grype release gates

The SBOM is now standard CycloneDX with resolved versions, source/lock identity
and the complete locked graph. This is broader than runtime composition. The
separate crates.io publisher runs the same gate. The OCI path scans a unique
candidate digest and promotes that exact digest after success. Existing audit,
package and attestation steps are retained. The previously missing cross-job
SBOM handoff is explicit; the crate artifact is included in release assembly.

Local workflow and actual lockfile validation are recorded in the PR. Hosted
Grype, image build/promotion and the full Rust suite remain unexecuted locally;
normal required CI and release environments remain mandatory.
