# Split rehearsal status

The independent destination is now created at
`https://github.com/edithatogo/citeweft`. The initial neutral extraction was
published as commit `f4a859e` on `main`; governance and workflow hardening was
then published as `c6d6a01`. It contains only the audited neutral
modules, standalone Cargo metadata, CI/security policy, and an independent
Conductor system. Sourceright adapters and canonical CSL/provider/sidecar
logic remain in this repository.

The source boundary remains non-destructive: the initial publication was built
from a disposable extraction directory and did not rewrite Sourceright history.
The destination's Rust, workflow-harness, and supply-chain CI jobs passed on
2026-07-12. The remaining split blocker is specifically source history: the
five neutral Sourceright modules are still untracked in the current worktree,
so a history-preserving split cannot yet be claimed.

Required next gate:

Remaining gates:

1. complete independent package/release evidence;
2. obtain release/registry approval;
3. consume an immutable candidate from Sourceright and run the downstream suite;
4. publish only after provenance, checksum, issue migration, and rollback proof.

## Path-preserving rehearsal

On 2026-07-13, a disposable Git fast-export/import rehearsal produced a
standalone one-commit repository containing exactly the five neutral modules.
All five module object hashes matched the published CiteWeft repository. This
proves the committed extraction slice is reproducibly separable; it does not
claim preservation of older Sourceright history because those files were
previously untracked.
