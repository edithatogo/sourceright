use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn vscode_packaging_has_local_vsix_without_marketplace_acceptance_claims() {
    let decision =
        read("conductor/tracks/77-vscode-open-vsx-publication-hardening/vsix-build-2026-05-18.md");
    let host_packaging = read("docs/src/host-packaging.md");
    let release_status = read("docs/src/release-status.md");
    let mcp_example = read("examples/mcp-clients/vscode-mcp.json");

    for marker in [
        "local VSIX package scaffold plus isolated install/uninstall",
        "Workspace Trust",
        "Install smoke: `passed`",
        "Uninstall smoke: `passed`",
        "Marketplace/Open VSX publication and accepted listing evidence remain",
    ] {
        assert!(
            decision.contains(marker),
            "missing decision marker: {marker}"
        );
    }

    assert!(
        host_packaging
            .contains("Local VSIX scaffold builds and passes isolated install/uninstall smoke")
    );
    assert!(host_packaging.contains("not Marketplace acceptance"));
    assert!(release_status.contains("VS Code Marketplace / Open VSX"));
    assert!(release_status.contains("| VS Code Marketplace / Open VSX | prepared |"));
    assert!(mcp_example.contains("\"type\": \"stdio\""));
    assert!(mcp_example.contains("\"mcp\""));
}

#[test]
fn vscode_local_vsix_scaffold_has_workspace_trust_and_smoke_script() {
    let package = read("extensions/vscode-sourceright/package.json");
    let extension = read("extensions/vscode-sourceright/extension.js");
    let build = read("scripts/build-vscode-vsix.ps1");
    let smoke = read("scripts/smoke-vscode-vsix.ps1");
    let packet = read("conductor/submission-packets/vscode-open-vsx.md");

    for marker in [
        "\"name\": \"sourceright\"",
        "\"publisher\": \"edithatogo\"",
        "\"untrustedWorkspaces\"",
        "sourceright.report",
    ] {
        assert!(package.contains(marker), "package missing {marker}");
    }
    assert!(extension.contains("report --json"));
    assert!(!extension.contains("export --all"));
    assert!(build.contains("sourceright.vscode_vsix_build.v1"));
    assert!(smoke.contains("--extensions-dir"));
    assert!(smoke.contains("--install-extension"));
    assert!(smoke.contains("--uninstall-extension"));
    assert!(smoke.contains("sourceright.vscode_vsix_smoke.v1"));
    assert!(packet.contains("Install/uninstall smoke"));
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
