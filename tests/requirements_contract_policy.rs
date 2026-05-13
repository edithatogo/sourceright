use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn feature_contract_matrix_is_canonical_and_granular() {
    let matrix = read("docs/src/feature-contract-matrix.md");

    assert!(matrix.contains("canonical requirements document and repo contract"));
    for priority in ["Must", "Should", "Could", "Won't for now"] {
        assert!(
            matrix.contains(priority),
            "missing MoSCoW priority {priority}"
        );
    }
    for contract in [
        "references.csl.json",
        "references.verification.json",
        "review-queue.jsonl",
        "Provider evidence must not silently overwrite canonical CSL",
        "OJS",
        "MCP",
        "Zotero",
        "official MCP Registry",
    ] {
        assert!(
            matrix.contains(contract),
            "missing contract marker {contract}"
        );
    }
}

#[test]
fn design_doc_covers_architecture_with_mermaid_diagrams() {
    let design = read("docs/src/design.md");

    assert!(design.matches("```mermaid").count() >= 6);
    for section in [
        "Data Boundaries",
        "CLI And MCP Surfaces",
        "Providers And Plugins",
        "Journal Integration",
        "Citation Manager Sync",
        "Release And Registry Flow",
    ] {
        assert!(design.contains(section), "missing design section {section}");
    }
}

#[test]
fn docs_site_has_contract_and_design_parity() {
    let site_matrix = read("docs-site/src/content/docs/feature-contract-matrix.md");
    let site_design = read("docs-site/src/content/docs/design.md");

    assert!(site_matrix.contains("MoSCoW matrix"));
    assert!(site_matrix.contains("Default live external CI"));
    assert!(site_design.matches("```mermaid").count() >= 6);
    assert!(site_design.contains("Release and registry flow"));
}
