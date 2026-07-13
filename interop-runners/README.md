# Optional interoperability runners

This package is optional and never writes Sourceright canonical CSL or
verification sidecars. It reads checked-in inputs and emits observations into
the caller-provided output path.

Pinned permissively licensed runners:

- Citation.js 0.8.1 — MIT
- `@retorquere/bibtex-parser` 10.0.0 — ISC

Both runners have checked-in `package-lock.json` evidence and have passed the
protected-casing fixture smoke. Citation.js output is normalized to omit its
non-CSL `citation-key` convenience field before comparison.

The lockfile is the reproducibility boundary. Run from this directory:

```powershell
npm ci
node citation-js-runner.mjs ../fixtures/interoperability/bibtex-basic.bib .tmp/citation-js.json
node bibtex-parser-runner.mjs ../fixtures/interoperability/bibtex-basic.bib .tmp/bibtex-parser.json
```

Run the full local matrix, including the adversarial BibTeX and RIS fixtures,
from the repository root with
`scripts/run-interoperability-matrix.ps1`. The optional GitHub Actions lane
uses the same lockfile and uploads JSON observations and Markdown reports on
success or failure.

`biblatex-csl-converter` remains an isolated LGPL-3.0 option and `citeproc-js`
remains an isolated CPAL-1.0/AGPL-1.0 option pending legal and packaging
review. `astrocite` remains a design reference, not a runtime dependency.

## Support and claim matrix

| Surface | Current evidence | Claim boundary |
| --- | --- | --- |
| BibTeX -> CSL via Citation.js | Basic and adversarial fixtures pass | Optional oracle; not canonical CSL authority |
| RIS -> CSL via Citation.js | Basic fixture passes | Conversion smoke only; round-trip fidelity remains open |
| BibTeX parsing via `@retorquere/bibtex-parser` | Basic and adversarial parser smoke passes | Parser observation; not a truth oracle |
| JabRef/JabKit | Not installed or executed | Optional disposable integration lane |
| `biblatex-csl-converter` | Deferred | LGPL packaging and legal review required |
| `citeproc-js` | Deferred | CPAL/AGPL license review required |
| Fidus Writer | Architecture reference only | No dependency or copied implementation |
| Astrocite | Design reference only | No dependency or adoption claim |

Known losses and discrepancies are reported by `interoperability-diff`; an
oracle never writes `references.csl.json` or `references.verification.json`.
