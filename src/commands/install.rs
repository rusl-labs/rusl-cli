use crate::cache::linker::Linker;
use crate::cache::store::GlobalStore;
use crate::cli::InstallArgs;
use crate::config;
use crate::manifest::bundle::BundleManifest;
use crate::manifest::lock::{LockDependency, LockManifest};
use crate::registry::client::RegistryClient;
use crate::resolver::graph::resolve_graph;
use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::{BTreeMap, HashMap};

pub async fn run(_args: InstallArgs) -> Result<()> {
    let pb = crate::ui::spinner("Starting installation...");

    let config = config::load().context("Failed to load hierarchical configuration")?;
    let client = RegistryClient::new(config.clone());
    let store = GlobalStore::new()
        .await
        .context("Failed to initialize CAS store")?;
    let linker = Linker::new(std::env::current_dir()?, config.schema_dir());

    let cwd = std::env::current_dir()?;
    let manifest_path = cwd.join("rusl.bundle.toml");

    if !manifest_path.exists() {
        anyhow::bail!("No rusl.bundle.toml found in current directory! Please create one.");
    }

    let manifest_contents = std::fs::read_to_string(&manifest_path)?;
    let manifest: BundleManifest =
        toml::from_str(&manifest_contents).context("Syntax error in rusl.bundle.toml")?;

    pb.set_message(format!(
        "Resolving dependencies for {}@{}...",
        manifest.bundle.name, manifest.bundle.version
    ));
    let resolved = resolve_graph(&manifest, &client, &pb)
        .await
        .context("Dependency resolution failed")?;

    let mut schema_count = 0;
    let mut integrity_map: HashMap<String, String> = HashMap::new();

    pb.set_message("Cleaning old schema cache...");
    linker
        .purge_all()
        .context("Failed to safely prune .rusl/schemas/ directory manually")?;

    pb.set_message("Downloading schemas...");

    for (pkg, version) in &resolved.versions {
        // Only download schema blobs — bundles are logical groupings with no artifact
        if !pkg.starts_with("schema:") {
            continue;
        }

        let clean_pkg = pkg.trim_start_matches("schema:");
        let parts: Vec<&str> = clean_pkg.split('/').collect();
        if parts.len() != 2 {
            pb.println(format!(
                "{} Skipping invalid schema key: {}",
                "Warning:".yellow().bold(),
                clean_pkg
            ));
            continue;
        }

        pb.set_message(format!("Downloading {}@{} ...", clean_pkg, version));
        let blob = client
            .download_schema_blob(parts[0], parts[1], &version.to_string())
            .await?;

        let (integrity, cas_path) = store.put(&blob).await?;
        integrity_map.insert(pkg.clone(), integrity);
        linker.link_schema(parts[0], parts[1], &cas_path)?;
        schema_count += 1;
    }

    // Build lock deps for ALL resolved packages (schemas + bundles) with dependency edges
    let mut lock_deps = BTreeMap::new();
    for (pkg, version) in &resolved.versions {
        let deps = resolved.edges.get(pkg).cloned().unwrap_or_default();
        let integrity = integrity_map.get(pkg).cloned().unwrap_or_default();

        lock_deps.insert(
            pkg.clone(),
            LockDependency {
                version: version.to_string(),
                integrity,
                source: config.api_base_url.clone(),
                dependencies: deps,
            },
        );
    }

    let new_lock = LockManifest {
        version: "1".to_string(),
        dependencies: lock_deps,
    };

    let lock_path = cwd.join("rusl.lock");
    let lock_toml = toml::to_string_pretty(&new_lock)?;
    std::fs::write(&lock_path, lock_toml).context("Failed to write rusl.lock")?;

    pb.finish_with_message(format!(
        "{} Installed {} schemas.",
        "Success:".green().bold(),
        schema_count
    ));
    Ok(())
}
