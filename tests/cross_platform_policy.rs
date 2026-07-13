use std::fs;
use std::path::Path;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

#[test]
fn local_gate_runners_are_available_for_posix_and_powershell_hosts() {
    assert!(Path::new("scripts/run-local-rust-gates.sh").exists());
    assert!(Path::new("scripts/run-local-rust-gates.ps1").exists());

    let posix = read("scripts/run-local-rust-gates.sh");
    assert!(posix.contains("${TMPDIR:-/tmp}/sourceright-target"));
    assert!(posix.contains("RUSTUP_TOOLCHAIN=\"${RUSTUP_TOOLCHAIN:-stable}\""));

    let powershell = read("scripts/run-local-rust-gates.ps1");
    assert!(powershell.contains("[System.IO.Path]::GetTempPath()"));
    assert!(powershell.contains("$env:OS -eq \"Windows_NT\""));
    assert!(!powershell.contains("C:\\tmp\\sourceright-target"));
}

#[test]
fn interoperability_scripts_do_not_depend_on_a_specific_user_install() {
    for path in [
        "scripts/run-interoperability-conformance.ps1",
        "scripts/run-interoperability-matrix.ps1",
    ] {
        let script = read(path);
        assert!(script.contains("Get-Command node"), "{path} should resolve node");
        assert!(!script.contains("60217257\\scoop"), "{path} has a user path");
        assert!(!script.contains("C:\\tmp\\sourceright-target"), "{path} has a host path");
    }
}
