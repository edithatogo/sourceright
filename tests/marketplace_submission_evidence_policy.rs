use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn marketplace_evidence_covers_required_hosts_with_state_boundaries() {
    let evidence =
        read("conductor/tracks/69-marketplace-submission-evidence/marketplace-evidence.md");
    let release_status = read("docs/src/release-status.md");
    let host_packaging = read("docs/src/host-packaging.md");

    for host in [
        "Official MCP Registry",
        "Glama",
        "Smithery",
        "Claude Desktop",
        "Codex",
        "GitHub Copilot",
        "VS Code Marketplace / Open VSX",
        "Microsoft AppSource / Word add-in",
        "LibreOffice Extensions",
        "Zotero",
        "OJS/PKP",
    ] {
        assert!(
            evidence.contains(host),
            "missing marketplace evidence for {host}"
        );
        assert!(
            release_status.contains(host),
            "missing release-status row for {host}"
        );
    }

    assert!(
        host_packaging.contains("Prepared metadata, local configuration, and development settings")
    );
    assert!(host_packaging.contains("not marketplace acceptance"));
}

#[test]
fn prepared_and_deferred_marketplaces_are_not_claimed_as_accepted() {
    let evidence =
        read("conductor/tracks/69-marketplace-submission-evidence/marketplace-evidence.md");
    let release_status = read("docs/src/release-status.md");
    let docs_site_status = read("docs-site/src/content/docs/release-status.md");

    for marker in [
        "Prepared metadata is not public marketplace acceptance",
        "No Zotero `.xpi` or Plugin Gallery listing is claimed",
        "No PKP Plugin Gallery acceptance is claimed",
        "Requires VSIX package",
        "Requires Office Add-in manifest",
        "Requires `.oxt` package",
    ] {
        assert!(
            evidence.contains(marker),
            "missing evidence boundary: {marker}"
        );
    }

    for marker in [
        "| Zotero | prepared |",
        "| OJS/PKP | prepared |",
        "| VS Code Marketplace / Open VSX | deferred |",
        "| Microsoft AppSource / Word add-in | deferred |",
        "| LibreOffice Extensions | deferred |",
    ] {
        assert!(
            release_status.contains(marker),
            "missing release-status marker: {marker}"
        );
        assert!(
            docs_site_status.contains(marker),
            "missing docs-site marker: {marker}"
        );
    }
}

#[test]
fn accepted_marketplaces_record_install_metadata() {
    let release_status = read("docs/src/release-status.md");

    for marker in [
        "| GitHub Release | accepted | 0.1.20 | <https://github.com/edithatogo/sourceright/releases/tag/v0.1.20> | 2026-05-11 | Platform binaries",
        "| crates.io | accepted | 0.1.20 | <https://crates.io/crates/sourceright> | 2026-05-11 | `cargo install sourceright`",
        "| docs.rs | accepted | 0.1.20 | <https://docs.rs/crate/sourceright/0.1.20> | 2026-05-11 | Auto-built from crates.io publish",
        "| Official MCP Registry | accepted | 0.1.20 | <https://registry.modelcontextprotocol.io/v0.1/servers?search=io.github.edithatogo/sourceright> | 2026-05-13 | Listed as active and latest",
    ] {
        assert!(
            release_status.contains(marker),
            "missing accepted install metadata: {marker}"
        );
    }
}
