use std::fs;
use std::path::Path;

fn read(path: &str) -> String {
    fs::read_to_string(path).expect("fixture should be present")
}

#[test]
fn ojs_plugin_source_skeleton_contains_required_package_files() {
    for path in [
        "plugins/ojs/sourceright/index.php",
        "plugins/ojs/sourceright/SourcerightPlugin.php",
        "plugins/ojs/sourceright/classes/SourcerightCliRunner.php",
        "plugins/ojs/sourceright/version.xml",
        "plugins/ojs/sourceright/locale/en_US/locale.po",
        "plugins/ojs/sourceright/README.md",
    ] {
        assert!(Path::new(path).exists(), "missing OJS plugin file {path}");
    }
}

#[test]
fn ojs_plugin_skeleton_keeps_cli_and_write_boundaries_explicit() {
    let runner = read("plugins/ojs/sourceright/classes/SourcerightCliRunner.php");
    let plugin = read("plugins/ojs/sourceright/SourcerightPlugin.php");
    let readme = read("plugins/ojs/sourceright/README.md");
    let version = read("plugins/ojs/sourceright/version.xml");

    assert!(runner.contains("escapeshellarg"));
    assert!(runner.contains("'journal-screen'"));
    assert!(runner.contains("'--platform'"));
    assert!(runner.contains("'ojs'"));
    assert!(runner.contains("'export'"));
    assert!(runner.contains("'--preview'"));
    assert!(runner.contains("'editorial_summary'"));
    assert!(runner.contains("'author_action_checklist'"));

    assert!(plugin.contains("allowExplicitWrites"));
    assert!(plugin.contains("sourcerightCliPath"));
    assert!(plugin.contains("sourcerightTimeoutSeconds"));

    assert!(readme.contains("source skeleton"));
    assert!(readme.contains("not PKP Plugin Gallery accepted"));
    assert!(readme.contains("must not silently overwrite canonical CSL data"));

    assert!(version.contains("<application>ojs</application>"));
    assert!(version.contains("<type>plugins.generic</type>"));
    assert!(version.contains("<package>sourceright</package>"));
}
