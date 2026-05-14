use serde_json::Value;
use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

#[test]
fn legal_citation_audit_pack_preserves_connector_boundary() {
    let docs = read("docs/src/legal-citation-audit-mcp.md");
    let site = read("docs-site/src/content/docs/guides/legal-citation-audit-mcp.md");
    let roadmap = read("docs/src/legal-roadmap.md");
    let workflow = read("docs/src/legal-citation-mode-workflow.md");

    for content in [&docs, &site, &roadmap, &workflow] {
        assert!(
            content.contains("attorney review"),
            "legal audit docs should require attorney review"
        );
        assert!(
            content.contains("legal advice"),
            "legal audit docs should reject legal-advice claims"
        );
        assert!(
            content.contains("legal conclusion") || content.contains("legal conclusions"),
            "legal audit docs should avoid legal-conclusion claims"
        );
    }

    assert!(docs.contains("citation audit and enrichment connector"));
    assert!(docs.contains("CourtListener"));
    assert!(docs.contains("licensed systems"));
    assert!(docs.contains("read-heavy"));
    assert!(docs.contains("provenance-first"));
}

#[test]
fn mcp_legal_tool_is_read_only_and_described_as_audit() {
    let manifest: Value =
        serde_json::from_str(&read("mcp/tools.v1.json")).expect("tools manifest must be JSON");
    let tools = manifest["tools"]
        .as_array()
        .expect("tools manifest must contain tools array");
    let legal_tool = tools
        .iter()
        .find(|tool| tool["name"].as_str() == Some("legal.analyze_citations"))
        .expect("legal.analyze_citations tool should exist");

    assert_eq!(legal_tool["read_only"], true);
    assert_eq!(
        legal_tool["output_contract"],
        "sourceright.legal_citation_report"
    );
    let boundary = legal_tool["boundary"]
        .as_str()
        .expect("legal tool should declare a boundary");
    assert!(boundary.contains("Draft legal citation audit only"));
    assert!(boundary.contains("not legal advice"));
    assert!(boundary.contains("outcome prediction"));

    let source = read("src/mcp.rs");
    assert!(source.contains("attorney-review flags"));
    assert!(source.contains("does not provide legal advice"));
}

#[test]
fn track_53_records_claude_for_legal_as_reference_not_dependency() {
    let compatibility =
        read("conductor/tracks/53-courtlistener-legal-provider/claude-for-legal-compatibility.md");
    let plan = read("conductor/tracks/53-courtlistener-legal-provider/plan.md");
    let matrix = read("conductor/tracks/53-courtlistener-legal-provider/test-matrix.md");

    assert!(compatibility.contains("connector/workflow design reference"));
    assert!(compatibility.contains("not as a code dependency"));
    assert!(compatibility.contains("not become a general legal assistant"));
    assert!(compatibility.contains("licensed research systems"));
    assert!(plan.contains("Claude For Legal Compatibility"));
    assert!(matrix.contains("Connector boundary"));
}
