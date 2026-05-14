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
fn ojs_plugin_has_install_test_package_builder_and_smoke_path() {
    let script = read("scripts/build-ojs-plugin-package.ps1");
    let docker_smoke = read("scripts/ojs-docker-install-smoke.ps1");
    let smoke = read("conductor/tracks/60-mature-ojs-plugin/ojs-install-smoke.md");
    let readme = read("plugins/ojs/sourceright/README.md");

    assert!(script.contains("sourceright-ojs-generic-plugin-$Version.tar.gz"));
    assert!(script.contains("plugins/generic/sourceright"));
    assert!(script.contains("Get-FileHash -Algorithm SHA256"));
    assert!(script.contains("tar -czf"));

    assert!(docker_smoke.contains("docker --version"));
    assert!(docker_smoke.contains("docker compose version"));
    assert!(docker_smoke.contains("https://github.com/pkp/containers.git"));
    assert!(docker_smoke.contains("installPluginVersion.php"));

    assert!(smoke.contains("installPluginVersion.php"));
    assert!(smoke.contains("pkp/containers"));
    assert!(smoke.contains("pkp/docker-ojs"));
    assert!(smoke.contains("not PKP Plugin Gallery acceptance"));
    assert!(smoke.contains("no verified live OJS smoke transcript"));

    assert!(readme.contains("scripts/build-ojs-plugin-package.ps1"));
    assert!(readme.contains("scripts/ojs-docker-install-smoke.ps1"));
    assert!(readme.contains("installPluginVersion.php"));
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
