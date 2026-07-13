use std::fs;

#[test]
fn arxiv_bundle_has_source_bibliography_and_explicit_claim_boundaries() {
    for path in [
        "arxiv/sourceright.tex",
        "arxiv/references.bib",
        "arxiv/00README",
        "arxiv/README.md",
    ] {
        assert!(
            fs::metadata(path).is_ok(),
            "missing arXiv package file: {path}"
        );
    }

    let tex = fs::read_to_string("arxiv/sourceright.tex").expect("arXiv TeX source");
    assert!(tex.contains("\\bibliography{references}"));
    assert!(tex.contains("technical preview"));
    assert!(tex.contains("GROBID-inspired"));
    assert!(tex.contains("does not submit to arXiv"));
}

#[test]
fn arxiv_bundle_does_not_depend_on_shell_escape_or_generated_output() {
    let tex = fs::read_to_string("arxiv/sourceright.tex").expect("arXiv TeX source");
    assert!(!tex.contains("\\immediate\\write18"));
    assert!(!tex.contains("shell-escape"));
    assert!(!tex.contains("arxiv/build"));
}
