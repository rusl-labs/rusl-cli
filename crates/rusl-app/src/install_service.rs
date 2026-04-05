use crate::cache::linker::Linker;
use crate::cache::store::GlobalStore;
use crate::config;
use crate::manifest::bundle::BundleManifest;
use crate::manifest::lock::{LockDependency, LockManifest};
use crate::registry::client::RegistryClient;
use crate::resolver::graph::{ProgressReporter, resolve_graph};
use anyhow::{Context, Result, bail};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallResult {
    pub schema_count: usize,
}

pub async fn install_project<P>(progress: &P) -> Result<InstallResult>
where
    P: ProgressReporter,
{
    let config = config::load().context("Failed to load hierarchical configuration")?;
    let client = RegistryClient::new(config.clone());
    let store = GlobalStore::new()
        .await
        .context("Failed to initialize CAS store")?;

    let cwd = std::env::current_dir().context("Failed to get current working directory")?;
    let linker = Linker::new(cwd.clone(), config.schema_dir());
    let manifest_path = cwd.join("rusl.bundle.toml");

    if !manifest_path.exists() {
        bail!("No rusl.bundle.toml found in current directory! Please create one.");
    }

    let manifest_contents =
        std::fs::read_to_string(&manifest_path).context("Failed to read rusl.bundle.toml")?;
    let manifest: BundleManifest =
        toml::from_str(&manifest_contents).context("Syntax error in rusl.bundle.toml")?;

    progress.set_message(format!(
        "Resolving dependencies for {}@{}...",
        manifest.bundle.name, manifest.bundle.version
    ));
    let resolved = resolve_graph(&manifest, &client, progress)
        .await
        .context("Dependency resolution failed")?;

    progress.set_message("Cleaning old schema cache...".to_string());
    linker
        .purge_all()
        .context("Failed to safely prune .rusl/schemas/ directory manually")?;

    progress.set_message("Downloading schemas...".to_string());

    let mut schema_count = 0;
    let mut integrity_map: HashMap<String, String> = HashMap::new();

    for (package_key, version) in &resolved.versions {
        if !package_key.starts_with("schema:") {
            continue;
        }

        let Some((account, slug)) = package_key.trim_start_matches("schema:").split_once('/')
        else {
            progress.println(format!(
                "Warning: Skipping invalid schema key: {package_key}"
            ));
            continue;
        };

        progress.set_message(format!("Downloading {account}/{slug}@{version} ..."));
        let blob = client
            .download_schema_blob(account, slug, &version.to_string())
            .await?;

        let (integrity, cas_path) = store.put(&blob).await?;
        integrity_map.insert(package_key.clone(), integrity);
        linker.link_schema(account, slug, &cas_path)?;
        schema_count += 1;
    }

    let mut lock_dependencies = BTreeMap::new();
    for (package_key, version) in &resolved.versions {
        let dependencies = resolved.edges.get(package_key).cloned().unwrap_or_default();
        let integrity = integrity_map.get(package_key).cloned().unwrap_or_default();

        lock_dependencies.insert(
            package_key.clone(),
            LockDependency {
                version: version.to_string(),
                integrity,
                source: config.api_base_url.clone(),
                dependencies,
            },
        );
    }

    let lock_manifest = LockManifest {
        version: "1".to_string(),
        dependencies: lock_dependencies,
    };

    let lock_path = cwd.join("rusl.lock");
    let lock_toml = toml::to_string_pretty(&lock_manifest)?;
    std::fs::write(&lock_path, lock_toml).context("Failed to write rusl.lock")?;

    Ok(InstallResult { schema_count })
}
