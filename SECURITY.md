# Security Policy

## Supported versions

Sourceright is pre-1.0. Security fixes target the `main` branch until release channels are established.

Security reports are acknowledged when maintainers can access the advisory and
are triaged as capacity permits. Reporters should include the affected version
or commit, operating system where relevant, impact, reproduction steps using
synthetic or redacted data, and any suggested mitigation. Do not include
secrets or private manuscripts in the report.

## Reporting a vulnerability

Please report security issues privately through GitHub Security Advisories:

https://github.com/edithatogo/sourceright/security/advisories/new

Avoid opening public issues for vulnerabilities, secrets, or provider-token exposure.

The advisory route is for vulnerabilities and accidental secret exposure only.
Use [SUPPORT.md](SUPPORT.md) for ordinary usage questions and reproducible
non-security bugs.

## Security expectations

Release-surface evidence is checked with
[`scripts/verify-release-surface-refresh.ps1`](scripts/verify-release-surface-refresh.ps1)
and should remain aligned with the documented accepted, prepared, and deferred
states.

- Do not commit API keys, provider tokens, sample documents containing private data, or proprietary citation databases.
- Keep provider fixtures scrubbed and reproducible.
- Use least-privilege GitHub Actions permissions.
- Keep verification provenance separate from clean reference exports.
