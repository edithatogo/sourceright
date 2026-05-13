# Citation Matching Disambiguation Test Matrix

| Scenario | Expected coverage |
| --- | --- |
| Institutional author | Matches organization citations to organization CSL authors. |
| Same-author same-year | Distinguishes suffixes or queues ambiguity. |
| `et al.` variants | Matches safe variants and reports unsafe ambiguity. |
| Vancouver numeric citations | Preserves order and missing-reference diagnostics. |
| Mixed style manuscript | Reports style drift without inventing matches. |
| Title fallback | Uses title evidence only when author-year matching is insufficient. |
