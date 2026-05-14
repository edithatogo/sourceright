use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn vscode_packaging_is_explicitly_deferred_without_extension_claims() {
    let decision = read("conductor/tracks/66-vscode-extension-packaging/packaging-decision.md");
    let host_packaging = read("docs/src/host-packaging.md");
    let release_status = read("docs/src/release-status.md");
    let mcp_example = read("examples/mcp-clients/vscode-mcp.json");

    for marker in [
        "does not currently ship a VSIX",
        "Neither asset is a Sourceright VS Code extension",
        "VS Code Marketplace and Open VSX remain deferred",
        "reimplementing reference verification logic",
        "Workspace Trust",
        "preview-only",
    ] {
        assert!(
            decision.contains(marker),
            "missing decision marker: {marker}"
        );
    }

    assert!(host_packaging.contains("Explicitly deferred with a future VSIX contract"));
    assert!(host_packaging.contains("explicit deferral"));
    assert!(release_status.contains("VS Code Marketplace / Open VSX"));
    assert!(release_status.contains("deferred"));
    assert!(mcp_example.contains("\"type\": \"stdio\""));
    assert!(mcp_example.contains("\"mcp\""));
}

#[test]
fn development_vscode_settings_remain_tooling_only() {
    let requirements = read("conductor/requirements.md");
    let settings = read(".vscode/settings.json");

    assert!(requirements.contains("Development `.vscode` settings are not a VS Code extension"));
    assert!(settings.contains("rust-analyzer"));
    assert!(!settings.contains("sourceright.extension"));
    assert!(!settings.contains("vsce"));
}
