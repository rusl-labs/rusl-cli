use crate::config;
use crate::manifest::lock::LockManifest;
use crate::registry::client::{RegistryClient, RegistryVersion};
use anyhow::{Context, Result};
use pubgrub::SemanticVersion;
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutdatedOutput {
    MissingLockfile,
    EmptyLockfile,
    Items(Vec<OutdatedItem>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutdatedItem {
    pub package_key: String,
    pub display_name: String,
    pub current_version: String,
    pub latest_version: String,
}

pub async fn load_outdated_dependencies() -> Result<OutdatedOutput> {
    let cwd = env::current_dir().context("Failed to get current working directory")?;
    let lock_path = cwd.join("rusl.lock");

    if !lock_path.exists() {
        return Ok(OutdatedOutput::MissingLockfile);
    }

    let lock_str = fs::read_to_string(&lock_path).context("Failed to read rusl.lock")?;
    let lock_manifest: LockManifest =
        toml::from_str(&lock_str).context("Failed to parse rusl.lock")?;

    if lock_manifest.dependencies.is_empty() {
        return Ok(OutdatedOutput::EmptyLockfile);
    }

    let config = config::load().context("Failed to load configuration")?;
    let client = RegistryClient::new(config);
    let mut outdated = Vec::new();

    for (package_key, dependency) in lock_manifest.dependencies {
        let Some(current_version) = parse_version(&dependency.version) else {
            continue;
        };
        let Some(target) = parse_registry_target(&package_key) else {
            continue;
        };

        let metadata = match target.kind {
            RegistryKind::Bundle => {
                client
                    .fetch_bundle_meta(&target.account, &target.slug)
                    .await
            }
            RegistryKind::Schema => {
                client
                    .fetch_schema_meta(&target.account, &target.slug)
                    .await
            }
        };

        let Ok(metadata) = metadata else {
            continue;
        };

        let Some(latest_version) = latest_version(&metadata.versions) else {
            continue;
        };

        if latest_version > current_version {
            outdated.push(OutdatedItem {
                package_key,
                display_name: target.display_name,
                current_version: dependency.version,
                latest_version: latest_version.to_string(),
            });
        }
    }

    Ok(OutdatedOutput::Items(outdated))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RegistryKind {
    Schema,
    Bundle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RegistryTarget {
    kind: RegistryKind,
    account: String,
    slug: String,
    display_name: String,
}

fn parse_registry_target(package_key: &str) -> Option<RegistryTarget> {
    let (kind, path) = package_key.split_once(':')?;
    let (account, slug) = path.split_once('/')?;

    let kind = match kind {
        "bundle" => RegistryKind::Bundle,
        "schema" => RegistryKind::Schema,
        _ => return None,
    };

    Some(RegistryTarget {
        kind,
        account: account.to_string(),
        slug: slug.to_string(),
        display_name: display_name(package_key),
    })
}

fn latest_version(versions: &[RegistryVersion]) -> Option<SemanticVersion> {
    versions
        .iter()
        .filter_map(|version| parse_version(&version.version))
        .max()
}

fn parse_version(version: &str) -> Option<SemanticVersion> {
    version.parse::<SemanticVersion>().ok()
}

fn display_name(package_key: &str) -> String {
    if let Some(path) = package_key.strip_prefix("bundle:") {
        format!("bundles/{path}")
    } else if let Some(path) = package_key.strip_prefix("schema:") {
        path.to_string()
    } else {
        package_key.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{OutdatedItem, display_name, latest_version, parse_registry_target};
    use crate::registry::client::RegistryVersion;
    use std::collections::HashMap;

    #[test]
    fn formats_registry_display_names() {
        assert_eq!(display_name("bundle:acme/common"), "bundles/acme/common");
        assert_eq!(display_name("schema:acme/types"), "acme/types");
    }

    #[test]
    fn parses_registry_targets() {
        let target = parse_registry_target("bundle:acme/common").expect("target should parse");

        assert_eq!(target.account, "acme");
        assert_eq!(target.slug, "common");
        assert_eq!(target.display_name, "bundles/acme/common");
        assert!(parse_registry_target("external:https://example.com/schema.json").is_none());
    }

    #[test]
    fn finds_latest_semantic_version() {
        let versions = vec![
            sample_version("1.2.0"),
            sample_version("2.0.0"),
            sample_version("not-a-version"),
            sample_version("1.5.1"),
        ];

        let latest = latest_version(&versions).expect("latest version should exist");
        assert_eq!(latest.to_string(), "2.0.0");
    }

    #[test]
    fn outdated_item_captures_render_data() {
        let item = OutdatedItem {
            package_key: "schema:acme/types".to_string(),
            display_name: "acme/types".to_string(),
            current_version: "1.0.0".to_string(),
            latest_version: "1.1.0".to_string(),
        };

        assert_eq!(item.display_name, "acme/types");
        assert_eq!(item.current_version, "1.0.0");
        assert_eq!(item.latest_version, "1.1.0");
    }

    fn sample_version(version: &str) -> RegistryVersion {
        RegistryVersion {
            version: version.to_string(),
            schemas: HashMap::new(),
            bundles: HashMap::new(),
        }
    }
}
