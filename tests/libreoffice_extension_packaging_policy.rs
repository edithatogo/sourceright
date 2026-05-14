use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn libreoffice_packaging_is_explicitly_deferred_without_extension_claims() {
    let decision =
        read("conductor/tracks/68-libreoffice-extension-packaging/packaging-decision.md");
    let host_packaging = read("docs/src/host-packaging.md");
    let release_status = read("docs/src/release-status.md");

    for marker in [
        "does not currently ship a `.oxt`",
        "not LibreOffice Writer extension support",
        "paragraph, footnote/endnote, table-cell, text-frame, and bibliography",
        "dry-run first",
        "explicit user action",
        "LibreOffice Extensions publication remains deferred",
    ] {
        assert!(
            decision.contains(marker),
            "missing decision marker: {marker}"
        );
    }

    assert!(host_packaging.contains("No `.oxt`/UNO extension package"));
    assert!(release_status.contains("LibreOffice Extensions"));
    assert!(release_status.contains("deferred"));
}

#[test]
fn libreoffice_contract_requires_reversible_audited_writer_mapping() {
    let spec = read("conductor/tracks/68-libreoffice-extension-packaging/spec.md");
    let decision =
        read("conductor/tracks/68-libreoffice-extension-packaging/packaging-decision.md");
    let requirements = read("conductor/requirements.md");

    assert!(spec.contains("document-range provenance"));
    assert!(spec.contains("dry-run write plans"));
    assert!(decision.contains("audit log"));
    assert!(decision.contains("reversible plan"));
    assert!(requirements.contains("ODT/DOCX processing is not LibreOffice extension support"));
    assert!(
        requirements
            .contains("Do not claim safe in-document editing before reversible change plans exist")
    );
}
