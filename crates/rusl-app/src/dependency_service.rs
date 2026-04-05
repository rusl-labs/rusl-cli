use crate::install_service::{self, InstallResult};
use crate::registry::client::{RegistryClient, RegistryVersion};
use crate::resolver::graph::ProgressReporter;
use anyhow::{Context, Result, bail};
use pubgrub::SemanticVersion;
use toml_edit::{DocumentMut, table, value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    Schema,
    Bundle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDependencyRequest {
    pub kind: DependencyKind,
    pub slug: String,
    pub version_requirement: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDependencyResult {
    pub slug: String,
    pub table_key: &'static str,
    pub version_requirement: String,
    pub install: InstallResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveDependencyRequest {
    pub kind: DependencyKind,
    pub slug: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoveDependencyResult {
    Removed {
        slug: String,
        table_key: &'static str,
        install: InstallResult,
    },
    NotPresent {
        slug: String,
        table_key: &'static str,
    },
}

pub async fn add_dependency<P>(
    request: AddDependencyRequest,
    progress: &P,
) -> Result<AddDependencyResult>
where
    P: ProgressReporter,
{
    progress.set_message("Reading rusl.bundle.toml...".to_string());

    let manifest_path = current_manifest_path()?;
    let mut document = read_manifest_document(&manifest_path)?;
    let table_key = table_key(request.kind);
    let version_requirement = match request.version_requirement {
        Some(version) => version,
        None => latest_version_requirement(request.kind, &request.slug, progress).await?,
    };

    if document.get(table_key).is_none() {
        document[table_key] = table();
    }

    let Some(table_ref) = document.get_mut(table_key) else {
        bail!("The [{table_key}] key exists but could not be loaded.");
    };
    let Some(table) = table_ref.as_table_mut() else {
        bail!("The [{table_key}] key exists but is not a TOML table.");
    };

    progress.set_message(format!(
        "Adding {}@{} to [{}]...",
        request.slug, version_requirement, table_key
    ));
    table.insert(&request.slug, value(&version_requirement));

    std::fs::write(&manifest_path, document.to_string())
        .context("Failed to persist rusl.bundle.toml")?;

    let install = install_service::install_project(progress).await?;
    Ok(AddDependencyResult {
        slug: request.slug,
        table_key,
        version_requirement,
        install,
    })
}

pub async fn remove_dependency<P>(
    request: RemoveDependencyRequest,
    progress: &P,
) -> Result<RemoveDependencyResult>
where
    P: ProgressReporter,
{
    progress.set_message("Reading rusl.bundle.toml...".to_string());

    let manifest_path = current_manifest_path()?;
    let mut document = read_manifest_document(&manifest_path)?;
    let table_key = table_key(request.kind);

    let removed = document
        .get_mut(table_key)
        .and_then(|item| item.as_table_mut())
        .and_then(|table| table.remove(&request.slug));

    if removed.is_none() {
        return Ok(RemoveDependencyResult::NotPresent {
            slug: request.slug,
            table_key,
        });
    }

    progress.set_message(format!("Removing {} from [{}]...", request.slug, table_key));
    std::fs::write(&manifest_path, document.to_string())
        .context("Failed to persist rusl.bundle.toml")?;

    let install = install_service::install_project(progress).await?;
    Ok(RemoveDependencyResult::Removed {
        slug: request.slug,
        table_key,
        install,
    })
}

fn current_manifest_path() -> Result<std::path::PathBuf> {
    let cwd = std::env::current_dir().context("Failed to get current working directory")?;
    let manifest_path = cwd.join("rusl.bundle.toml");
    if !manifest_path.exists() {
        bail!("No rusl.bundle.toml found in current directory! Please create one.");
    }
    Ok(manifest_path)
}

fn read_manifest_document(path: &std::path::Path) -> Result<DocumentMut> {
    let contents = std::fs::read_to_string(path).context("Failed to read rusl.bundle.toml")?;
    contents
        .parse::<DocumentMut>()
        .context("Failed to parse rusl.bundle.toml")
}

async fn latest_version_requirement<P>(
    kind: DependencyKind,
    slug: &str,
    progress: &P,
) -> Result<String>
where
    P: ProgressReporter,
{
    progress.set_message("Finding the latest version...".to_string());
    let Some((account, name)) = slug.split_once('/') else {
        bail!("Slug must be in format account/name");
    };

    let config = crate::config::load().context("Failed to load hierarchical configuration")?;
    let client = RegistryClient::new(config);
    let metadata = match kind {
        DependencyKind::Schema => client.fetch_schema_meta(account, name).await?,
        DependencyKind::Bundle => client.fetch_bundle_meta(account, name).await?,
    };

    let latest = latest_version(&metadata.versions)
        .context("The registry did not return any resolvable semantic versions for this slug.")?;
    Ok(format!(">={latest}"))
}

fn latest_version(versions: &[RegistryVersion]) -> Option<SemanticVersion> {
    versions
        .iter()
        .filter_map(|version| version.version.parse::<SemanticVersion>().ok())
        .max()
}

fn table_key(kind: DependencyKind) -> &'static str {
    match kind {
        DependencyKind::Schema => "schemas",
        DependencyKind::Bundle => "bundles",
    }
}

#[cfg(test)]
mod tests {
    use super::{DependencyKind, latest_version, table_key};
    use crate::registry::client::RegistryVersion;
    use std::collections::HashMap;

    #[test]
    fn maps_dependency_kinds_to_manifest_tables() {
        assert_eq!(table_key(DependencyKind::Schema), "schemas");
        assert_eq!(table_key(DependencyKind::Bundle), "bundles");
    }

    #[test]
    fn picks_highest_semantic_version() {
        let versions = vec![
            sample_version("1.0.0"),
            sample_version("2.1.0"),
            sample_version("not-a-version"),
            sample_version("2.0.1"),
        ];

        let latest = latest_version(&versions).expect("latest version should exist");
        assert_eq!(latest.to_string(), "2.1.0");
    }

    fn sample_version(version: &str) -> RegistryVersion {
        RegistryVersion {
            version: version.to_string(),
            schemas: HashMap::new(),
            bundles: HashMap::new(),
        }
    }
}
