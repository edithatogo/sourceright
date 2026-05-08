# Security Policy

## Supported versions

Sourceright is pre-1.0. Security fixes target the `main` branch until release channels are established.

## Reporting a vulnerability

Please report security issues privately through GitHub Security Advisories once the public repository is available. Avoid opening public issues for vulnerabilities, secrets, or provider-token exposure.

## Security expectations

- Do not commit API keys, provider tokens, sample documents containing private data, or proprietary citation databases.
- Keep provider fixtures scrubbed and reproducible.
- Use least-privilege GitHub Actions permissions.
- Keep verification provenance separate from clean reference exports.
