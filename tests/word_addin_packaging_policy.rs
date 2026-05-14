use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn word_addin_packaging_is_explicitly_deferred_without_addin_claims() {
    let decision = read("conductor/tracks/67-word-addin-packaging/packaging-decision.md");
    let host_packaging = read("docs/src/host-packaging.md");
    let release_status = read("docs/src/release-status.md");

    for marker in [
        "does not currently ship an Office Add-in manifest",
        "not Word add-in support",
        "paragraph, footnote/endnote, table-cell, and bibliography anchors",
        "dry-run first",
        "explicit user action",
        "Microsoft AppSource remains deferred",
    ] {
        assert!(
            decision.contains(marker),
            "missing decision marker: {marker}"
        );
    }

    assert!(host_packaging.contains("DOCX extraction is separate"));
    assert!(host_packaging.contains("no Office Add-in package"));
    assert!(release_status.contains("Microsoft AppSource / Word add-in"));
    assert!(release_status.contains("deferred"));
}

#[test]
fn word_addin_contract_requires_reversible_audited_range_mapping() {
    let spec = read("conductor/tracks/67-word-addin-packaging/spec.md");
    let decision = read("conductor/tracks/67-word-addin-packaging/packaging-decision.md");
    let requirements = read("conductor/requirements.md");

    assert!(spec.contains("document-range provenance"));
    assert!(spec.contains("reversible change plans"));
    assert!(decision.contains("audit log"));
    assert!(decision.contains("reversible plan"));
    assert!(requirements.contains("DOCX extraction is not Word add-in support"));
    assert!(
        requirements
            .contains("Do not claim safe in-document editing before reversible change plans exist")
    );
}
