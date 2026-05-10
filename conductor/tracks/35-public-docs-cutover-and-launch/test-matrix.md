# Public Docs Cutover And Launch Test Matrix

| Area | Check |
| --- | --- |
| Astro build | `npm run build` under `docs-site/` |
| Pages deploy | Validate `.github/workflows/pages.yml` uploads `docs-site/dist` |
| CI parity | Confirm docs CI runs both the Astro build and Rust docs checks |
| Navigation | Check guide/reference coverage against the docs summary |
| Cutover | Verify README and publishing guidance point to the public docs site |
