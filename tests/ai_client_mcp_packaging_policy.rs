use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("read {path}: {error}"))
}

#[test]
fn ai_client_examples_cover_required_hosts_without_plugin_claims() {
    let host_packaging = read("docs/src/host-packaging.md");
    let mcp = read("docs/src/mcp.md");
    let examples = read("examples/mcp-clients/README.md");

    for required in [
        "Claude Desktop",
        "Codex",
        "GitHub Copilot",
        "Generic MCP clients",
        "client configuration",
        "not a Claude plugin package",
        "not a Codex plugin package",
        "not a Copilot extension",
    ] {
        assert!(
            host_packaging.contains(required)
                || mcp.contains(required)
                || examples.contains(required),
            "missing AI client packaging boundary: {required}",
        );
    }

    for path in [
        "examples/mcp-clients/claude-desktop.json",
        "examples/mcp-clients/codex-config.toml",
        "examples/mcp-clients/generic-mcp-client.json",
        "examples/mcp-clients/vscode-mcp.json",
        "examples/mcp-clients/github-copilot-coding-agent.md",
    ] {
        let content = read(path);
        assert!(
            content.contains("sourceright"),
            "missing sourceright in {path}",
        );
    }
}

#[test]
fn ai_client_release_status_keeps_config_separate_from_marketplace_acceptance() {
    let release_status = read("docs/src/release-status.md");
    let host_packaging = read("docs/src/host-packaging.md");

    assert!(release_status.contains("Claude Desktop client config"));
    assert!(release_status.contains("Codex MCP client config"));
    assert!(release_status.contains("Generic MCP client config"));
    assert!(release_status.contains("GitHub Copilot coding-agent prep"));
    assert!(release_status.contains("prepared"));
    assert!(release_status.contains("No Claude, Codex, or Copilot marketplace package exists"));
    assert!(
        host_packaging.contains("Prepared metadata, local configuration, and development settings")
    );
    assert!(host_packaging.contains("not marketplace acceptance"));
}

#[test]
fn ai_client_mcp_smoke_contract_is_documented() {
    let matrix = read("conductor/tracks/65-ai-client-mcp-packaging/test-matrix.md");
    let generic = read("examples/mcp-clients/host-manifest.json");
    let mcp = read("docs/src/mcp.md");

    for method in ["initialize", "tools/list", "resources/list", "prompts/list"] {
        assert!(
            matrix.contains(method),
            "missing matrix smoke method {method}",
        );
        assert!(
            generic.contains(method),
            "missing generic smoke method {method}",
        );
        assert!(
            mcp.contains(method),
            "missing MCP docs smoke method {method}",
        );
    }

    assert!(mcp.contains("apply: true"));
    assert!(generic.contains("dry-run unless apply is true"));
}
