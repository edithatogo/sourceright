---
title: Coverage reporting
description: Coverage gate, hook parity, and summary reporting.
---

Coverage stays above 85 percent branch coverage and is checked in CI, in the
pre-commit hook, and through the shared verification script.

- Use `scripts/verify.ps1` for local parity.
- Keep coverage evidence reproducible on the supported toolchain.
- Treat the Ubuntu workflow summary as the authoritative numeric branch
  coverage report.
