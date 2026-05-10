# Coverage Verification And Reporting Test Matrix

| Area | Check |
| --- | --- |
| Coverage gate | `cargo llvm-cov --locked --all-targets --summary-only --fail-under-lines 90` |
| Hook parity | Run the tracked pre-commit hook against the same threshold |
| CI parity | Validate workflow steps match the hook and script inputs |
| Documentation | Coverage threshold is stated in README and CONTRIBUTING |
| Reporting | Coverage output is reproducible in the supported environment |
