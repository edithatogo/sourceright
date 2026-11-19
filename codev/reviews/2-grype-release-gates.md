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

## Hosted qualifier corrections — 20 November 2026

The first hosted source scan passed, but the native contract tests still required
Cargo metadata under an SBOM filename. They now require resolved CycloneDX and
critical gating before publication. Release preparation uses the same inventory
and policy, preserving meaningful dry-run parity. Thirteen affected Rust policy
tests pass when compiled and run directly with rustc 1.99.1; the application's
Rust 1.94 selection remains unchanged and still needs hosted qualification.

The existing workflow harness requires a timeout on every job, including monitors.
The released-asset monitor therefore uses pinned component actions in a local
25-minute job. The rule is unchanged.

The mandatory dry run could not compile newly released cargo-deny 0.22.0 with
application Rust 1.94. Its official Linux x86_64 musl archive matches both upstream
SHA-256 sources: `98c56e6a408bfa5c3c60744b6015aaa6aa08f743fdeb88386d06fca035bb1786`.
The installer validates that digest before extraction and requires exact version
0.22.0 before adding the executable to PATH. Two negative tests verify refusal of
corrupt downloads and unsupported hosts; four OCI qualification tests still pass.

Actual `cargo deny --locked check advisories bans sources` passes locally with
the verified binary and available Cargo 1.99.1. Cargo.lock is byte-identical. The
existing unmatched wit-bindgen skip remains a warning. The explicit
`unmaintained = "all"` retains the older tool's policy when 0.22 changes its default.
No application dependency, advisory ignore or deny severity was changed.

The current fuzz PR 144 owns different paths. Bot PRs 138 and 67 overlap workflow
files only at runner-hardening and checkout-reference hunks; this correction
leaves those pending proposals untouched. Existing source acceptance, release
environments and external publication approvals remain separate gates.

Upstream: <https://github.com/EmbarkStudios/cargo-deny/releases/tag/0.22.0>.
