use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const PLUGIN_REGISTRY_SCHEMA_VERSION: &str = "sourceright.plugin-registry.v1";
pub const PLUGIN_MANIFEST_SCHEMA_VERSION: &str = "sourceright.plugin.v1";
pub const PLUGIN_DISCOVERY_SCHEMA_VERSION: &str = "sourceright.plugin_registry_report.v1";

const REGISTRY_RELATIVE_PATH: &str = "plugins/registry.toml";
const TRUST_POLICY_RELATIVE_PATH: &str = "plugins/trust-policy.toml";

#[derive(Debug, Clone, Serialize)]
pub struct PluginRegistryReport {
    pub schema_version: &'static str,
    pub registry_path: String,
    pub registry_schema_version: String,
    pub generated_for: String,
    pub runtime_loading: bool,
    pub trust_policy: TrustPolicyReport,
    pub summary: PluginRegistrySummary,
    pub plugins: Vec<PluginDiscoveryEntry>,
    pub diagnostics: Vec<PluginDiagnostic>,
}

impl PluginRegistryReport {
    pub fn summary_text(&self) -> String {
        let mut lines = vec![
            "Sourceright plugin registry".to_string(),
            format!("schema_version: {}", self.registry_schema_version),
            format!("runtime_loading: {}", self.runtime_loading),
            format!("registry_path: {}", self.registry_path),
            format!(
                "trust_policy: {}",
                if self.trust_policy.present {
                    format!("present ({})", self.trust_policy.path)
                } else {
                    "absent".to_string()
                }
            ),
            format!("total_plugins: {}", self.summary.total),
            format!("validated_plugins: {}", self.summary.validated),
            format!("invalid_plugins: {}", self.summary.invalid),
        ];

        for plugin in &self.plugins {
            lines.push(format!(
                "- {} [{}] {} | trust={} execution={}{}",
                plugin.id,
                plugin.category,
                plugin.status,
                if plugin.execution_gate.trusted {
                    "trusted"
                } else {
                    "untrusted"
                },
                if plugin.execution_gate.allowed {
                    "allowed"
                } else {
                    "refused"
                },
                if plugin.valid {
                    String::new()
                } else {
                    format!(" | diagnostics={}", plugin.diagnostics.len())
                }
            ));
        }

        lines.join("\n")
    }

    pub fn is_valid(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginRegistrySummary {
    pub total: usize,
    pub validated: usize,
    pub invalid: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct TrustPolicyReport {
    pub path: String,
    pub present: bool,
    pub allowed_plugin_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginDiscoveryEntry {
    pub id: String,
    pub name: String,
    pub category: String,
    pub status: String,
    pub plugin_api: String,
    pub license_profile: String,
    pub summary: String,
    pub manifest_path: String,
    pub registry_status: String,
    pub auth_required: bool,
    pub auth_mode: String,
    pub auth_env: Vec<String>,
    pub network: bool,
    pub default_enabled: bool,
    pub live_tests_default: bool,
    pub cache_policy: String,
    pub cache_retention: String,
    pub reads: Vec<String>,
    pub writes: Vec<String>,
    pub provenance: PluginProvenance,
    pub execution_gate: PluginExecutionGate,
    pub diagnostics: Vec<PluginDiagnostic>,
    pub valid: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginProvenance {
    pub registry_path: String,
    pub generated_for: String,
    pub manifest_path: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginExecutionGate {
    pub trusted: bool,
    pub allowed: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PluginDiagnostic {
    pub plugin_id: Option<String>,
    pub path: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Error)]
pub enum PluginRegistryError {
    #[error("failed to locate plugin registry under {0}")]
    RegistryNotFound(String),
    #[error("failed to read {path}: {error}")]
    ReadFile { path: String, error: String },
    #[error("failed to parse {path}: {error}")]
    ParseFile { path: String, error: String },
}

pub fn discover_plugins() -> Result<PluginRegistryReport, PluginRegistryError> {
    discover_plugins_from(find_repo_root(
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
    ))
}

pub fn discover_plugins_from(root: PathBuf) -> Result<PluginRegistryReport, PluginRegistryError> {
    let registry_path = root.join(REGISTRY_RELATIVE_PATH);
    if !registry_path.is_file() {
        return Err(PluginRegistryError::RegistryNotFound(
            registry_path.display().to_string(),
        ));
    }

    let registry_text = read_file(&registry_path)?;
    let registry: PluginRegistryFile =
        toml::from_str(&registry_text).map_err(|error| PluginRegistryError::ParseFile {
            path: registry_path.display().to_string(),
            error: error.to_string(),
        })?;

    let trust_policy = load_trust_policy(&root)?;

    let mut plugins = Vec::new();
    let mut diagnostics = Vec::new();

    for registry_entry in registry.plugins {
        let manifest_path = root.join(&registry_entry.manifest);
        let mut entry_diagnostics = Vec::new();
        let mut manifest_file: Option<PluginManifestFile> = None;

        match read_file(&manifest_path) {
            Ok(text) => match toml::from_str::<PluginManifestFile>(&text) {
                Ok(parsed) => manifest_file = Some(parsed),
                Err(error) => entry_diagnostics.push(PluginDiagnostic {
                    plugin_id: Some(registry_entry.id.clone()),
                    path: manifest_path.display().to_string(),
                    code: "manifest.parse_failed".to_string(),
                    message: error.to_string(),
                }),
            },
            Err(error) => entry_diagnostics.push(PluginDiagnostic {
                plugin_id: Some(registry_entry.id.clone()),
                path: manifest_path.display().to_string(),
                code: "manifest.read_failed".to_string(),
                message: error.to_string(),
            }),
        }

        let (
            plugin_name,
            category,
            status,
            plugin_api,
            license_profile,
            summary,
            auth_required,
            auth_mode,
            auth_env,
            network,
            default_enabled,
            live_tests_default,
            cache_policy,
            cache_retention,
            reads,
            writes,
        ) = if let Some(manifest) = &manifest_file {
            let plugin = &manifest.plugin;
            let auth = manifest.auth.as_ref();
            let cache = manifest.cache.as_ref();
            let runtime = manifest.runtime.as_ref();
            let contracts = manifest.contracts.as_ref();

            if plugin.id != registry_entry.id {
                entry_diagnostics.push(PluginDiagnostic {
                    plugin_id: Some(registry_entry.id.clone()),
                    path: manifest_path.display().to_string(),
                    code: "manifest.id_mismatch".to_string(),
                    message: format!(
                        "registry id `{}` does not match manifest id `{}`",
                        registry_entry.id, plugin.id
                    ),
                });
            }

            if plugin.plugin_api != PLUGIN_MANIFEST_SCHEMA_VERSION {
                entry_diagnostics.push(PluginDiagnostic {
                    plugin_id: Some(registry_entry.id.clone()),
                    path: manifest_path.display().to_string(),
                    code: "manifest.plugin_api_invalid".to_string(),
                    message: format!(
                        "expected `{}` but found `{}`",
                        PLUGIN_MANIFEST_SCHEMA_VERSION, plugin.plugin_api
                    ),
                });
            }

            if !plugin.id.starts_with(&format!("{}.", plugin.category)) {
                entry_diagnostics.push(PluginDiagnostic {
                    plugin_id: Some(registry_entry.id.clone()),
                    path: manifest_path.display().to_string(),
                    code: "manifest.category_mismatch".to_string(),
                    message: format!(
                        "plugin id `{}` does not use category `{}` as its prefix",
                        plugin.id, plugin.category
                    ),
                });
            }

            if registry_entry.status != plugin.status {
                entry_diagnostics.push(PluginDiagnostic {
                    plugin_id: Some(registry_entry.id.clone()),
                    path: manifest_path.display().to_string(),
                    code: "registry.status_mismatch".to_string(),
                    message: format!(
                        "registry status `{}` does not match manifest status `{}`",
                        registry_entry.status, plugin.status
                    ),
                });
            }

            (
                plugin.name.clone(),
                plugin.category.clone(),
                plugin.status.clone(),
                plugin.plugin_api.clone(),
                plugin.license_profile.clone(),
                plugin.summary.clone(),
                auth.map(|auth| auth.required).unwrap_or(false),
                auth.map(|auth| auth.mode.clone())
                    .unwrap_or_else(|| "none".to_string()),
                auth.map(|auth| auth.env.clone()).unwrap_or_default(),
                runtime.map(|runtime| runtime.network).unwrap_or(false),
                runtime
                    .map(|runtime| runtime.default_enabled)
                    .unwrap_or(false),
                runtime
                    .map(|runtime| runtime.live_tests_default)
                    .unwrap_or(false),
                cache
                    .map(|cache| cache.policy.clone())
                    .unwrap_or_else(|| "unspecified".to_string()),
                cache
                    .map(|cache| cache.retention.clone())
                    .unwrap_or_else(|| "unspecified".to_string()),
                contracts
                    .and_then(|contracts| contracts.reads.clone())
                    .unwrap_or_default(),
                contracts
                    .and_then(|contracts| contracts.writes.clone())
                    .unwrap_or_default(),
            )
        } else {
            (
                registry_entry.id.clone(),
                registry_entry
                    .id
                    .split('.')
                    .next()
                    .unwrap_or_default()
                    .to_string(),
                registry_entry.status.clone(),
                PLUGIN_MANIFEST_SCHEMA_VERSION.to_string(),
                "unknown".to_string(),
                String::new(),
                false,
                "none".to_string(),
                Vec::new(),
                false,
                false,
                false,
                "unspecified".to_string(),
                "unspecified".to_string(),
                Vec::new(),
                Vec::new(),
            )
        };

        let trusted = trust_policy.allowed_plugin_ids.contains(&registry_entry.id);
        let execution_allowed = trusted;
        let reason = if trusted {
            "trusted by loaded policy".to_string()
        } else if trust_policy.present {
            "plugin is not included in the loaded trust policy".to_string()
        } else {
            "trust policy is absent".to_string()
        };

        let valid = entry_diagnostics.is_empty();
        if !valid {
            diagnostics.extend(entry_diagnostics.clone());
        }

        plugins.push(PluginDiscoveryEntry {
            id: registry_entry.id,
            name: plugin_name,
            category,
            status,
            plugin_api,
            license_profile,
            summary,
            manifest_path: manifest_path.display().to_string(),
            registry_status: registry_entry.status,
            auth_required,
            auth_mode,
            auth_env,
            network,
            default_enabled,
            live_tests_default,
            cache_policy,
            cache_retention,
            reads,
            writes,
            provenance: PluginProvenance {
                registry_path: registry_path.display().to_string(),
                generated_for: registry.registry.generated_for.clone(),
                manifest_path: manifest_path.display().to_string(),
            },
            execution_gate: PluginExecutionGate {
                trusted,
                allowed: execution_allowed,
                reason,
            },
            diagnostics: entry_diagnostics,
            valid,
        });
    }

    let summary = PluginRegistrySummary {
        total: plugins.len(),
        validated: plugins.iter().filter(|plugin| plugin.valid).count(),
        invalid: plugins.iter().filter(|plugin| !plugin.valid).count(),
    };

    Ok(PluginRegistryReport {
        schema_version: PLUGIN_DISCOVERY_SCHEMA_VERSION,
        registry_path: registry_path.display().to_string(),
        registry_schema_version: registry.registry.schema_version,
        generated_for: registry.registry.generated_for,
        runtime_loading: registry.registry.runtime_loading,
        trust_policy,
        summary,
        plugins,
        diagnostics,
    })
}

pub fn plugin_registry_resource_report() -> Result<PluginRegistryReport, PluginRegistryError> {
    discover_plugins()
}

fn load_trust_policy(root: &Path) -> Result<TrustPolicyReport, PluginRegistryError> {
    let trust_policy_path = root.join(TRUST_POLICY_RELATIVE_PATH);
    if !trust_policy_path.is_file() {
        return Ok(TrustPolicyReport {
            path: trust_policy_path.display().to_string(),
            present: false,
            allowed_plugin_ids: Vec::new(),
        });
    }

    let text = read_file(&trust_policy_path)?;
    let parsed = toml::from_str::<TrustPolicyFile>(&text).map_err(|error| {
        PluginRegistryError::ParseFile {
            path: trust_policy_path.display().to_string(),
            error: error.to_string(),
        }
    })?;

    Ok(TrustPolicyReport {
        path: trust_policy_path.display().to_string(),
        present: true,
        allowed_plugin_ids: parsed.trust.allowed_plugins,
    })
}

fn find_repo_root(start: PathBuf) -> PathBuf {
    for ancestor in start.ancestors() {
        if ancestor.join(REGISTRY_RELATIVE_PATH).is_file() {
            return ancestor.to_path_buf();
        }
    }

    start
}

fn read_file(path: &Path) -> Result<String, PluginRegistryError> {
    fs::read_to_string(path).map_err(|error| PluginRegistryError::ReadFile {
        path: path.display().to_string(),
        error: error.to_string(),
    })
}

#[derive(Debug, Clone, Deserialize)]
struct PluginRegistryFile {
    registry: RegistryMetadata,
    plugins: Vec<RegistryEntry>,
}

#[derive(Debug, Clone, Deserialize)]
struct RegistryMetadata {
    schema_version: String,
    generated_for: String,
    runtime_loading: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct RegistryEntry {
    id: String,
    manifest: String,
    status: String,
}

#[derive(Debug, Clone, Deserialize)]
struct PluginManifestFile {
    plugin: PluginMetadata,
    auth: Option<AuthMetadata>,
    runtime: Option<RuntimeMetadata>,
    cache: Option<CacheMetadata>,
    contracts: Option<ContractMetadata>,
}

#[derive(Debug, Clone, Deserialize)]
struct PluginMetadata {
    id: String,
    name: String,
    category: String,
    status: String,
    plugin_api: String,
    license_profile: String,
    summary: String,
}

#[derive(Debug, Clone, Deserialize)]
struct AuthMetadata {
    required: bool,
    mode: String,
    env: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct RuntimeMetadata {
    network: bool,
    default_enabled: bool,
    live_tests_default: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct CacheMetadata {
    policy: String,
    retention: String,
}

#[derive(Debug, Clone, Deserialize)]
struct ContractMetadata {
    reads: Option<Vec<String>>,
    writes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
struct TrustPolicyFile {
    trust: TrustPolicySection,
}

#[derive(Debug, Clone, Deserialize)]
struct TrustPolicySection {
    allowed_plugins: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_plugins_loads_fixture_manifests() {
        let report = discover_plugins().expect("discover plugins");

        assert!(report.summary.total >= 10);
        assert!(report.summary.validated >= 10);
        assert!(
            report
                .plugins
                .iter()
                .any(|plugin| plugin.id == "provider.crossref")
        );
        assert!(
            report
                .plugins
                .iter()
                .any(|plugin| plugin.id == "citation-manager.zotero")
        );
        assert!(!report.trust_policy.present);
        assert!(
            report
                .plugins
                .iter()
                .all(|plugin| !plugin.execution_gate.allowed)
        );
    }

    #[test]
    fn summary_text_mentions_loaded_plugins() {
        let report = discover_plugins().expect("discover plugins");
        let summary = report.summary_text();

        assert!(summary.contains("Sourceright plugin registry"));
        assert!(summary.contains("provider.crossref"));
    }

    #[test]
    fn invalid_registry_entry_is_reported() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        let root = tempdir.path();
        fs::create_dir_all(root.join("plugins/manifests")).expect("create manifests dir");
        fs::write(
            root.join("plugins/registry.toml"),
            r#"
[registry]
schema_version = "sourceright.plugin-registry.v1"
generated_for = "test"
runtime_loading = true

[[plugins]]
id = "provider.crossref"
manifest = "plugins/manifests/provider.crossref.toml"
status = "core_normalizer"
"#,
        )
        .expect("write registry");
        fs::write(
            root.join("plugins/manifests/provider.crossref.toml"),
            r#"
[plugin]
id = "provider.example"
name = "Example"
category = "provider"
status = "core_normalizer"
plugin_api = "sourceright.plugin.v1"
license_profile = "open"
summary = "Example plugin."

[auth]
required = false
mode = "none"
env = []

[runtime]
network = false
default_enabled = false
live_tests_default = false

[contracts]
reads = ["references.csl.json"]
writes = ["sourceright.verification.v1"]
"#,
        )
        .expect("write manifest");

        let report = discover_plugins_from(root.to_path_buf()).expect("discover");

        assert!(!report.is_valid());
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "manifest.id_mismatch")
        );
    }
}
