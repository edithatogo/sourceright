use std::fs;

#[test]
fn host_packaging_tracks_are_contractually_registered() {
    let tracks = fs::read_to_string("conductor/tracks.md").expect("read tracks");
    for track in [
        "65 AI client MCP packaging",
        "66 VS Code extension packaging",
        "67 Microsoft Word add-in packaging",
        "68 LibreOffice extension packaging",
        "69 Marketplace submission evidence",
    ] {
        assert!(tracks.contains(track), "missing {track}");
    }

    let requirements = fs::read_to_string("conductor/requirements.md").expect("read requirements");
    for guard in [
        "Do not describe Sourceright as a Claude, Codex, or Copilot plugin",
        "Development `.vscode` settings are not a VS Code extension",
        "DOCX extraction is not Word add-in support",
        "ODT/DOCX processing is not LibreOffice extension support",
        "Prepared metadata is not public marketplace acceptance",
    ] {
        assert!(requirements.contains(guard), "missing guard: {guard}");
    }
}

#[test]
fn host_packaging_public_docs_cover_all_requested_hosts() {
    let docs = fs::read_to_string("docs/src/host-packaging.md").expect("read host packaging docs");
    for host in [
        "Claude Desktop",
        "Codex",
        "GitHub Copilot",
        "VS Code",
        "Microsoft Word",
        "LibreOffice Writer",
    ] {
        assert!(docs.contains(host), "missing host row: {host}");
    }
    assert!(docs.contains("Prepared metadata, local configuration, and development settings"));
}
