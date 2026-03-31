use crate::cli::OutdatedArgs;
use crate::manifest::lock::LockManifest;
use crate::registry::client::RegistryClient;
use anyhow::{Context, Result};
use colored::Colorize;
use pubgrub::SemanticVersion;
use std::env;
use std::fs;

pub async fn run(_args: OutdatedArgs) -> Result<()> {
    let cwd = env::current_dir().context("Failed to get current working directory")?;
    let lock_path = cwd.join("rusl.lock");

    if !lock_path.exists() {
        println!("{}", "No schemas installed. `rusl.lock` not found.".yellow());
        return Ok(());
    }

    let lock_str = fs::read_to_string(&lock_path).context("Failed to read rusl.lock")?;
    let lock_manifest: LockManifest = toml::from_str(&lock_str).context("Failed to parse rusl.lock")?;

    if lock_manifest.dependencies.is_empty() {
        println!("{}", "No dependencies found in rusl.lock.".yellow());
        return Ok(());
    }

    let config = crate::config::load().context("Failed to load configuration")?;
    let client = RegistryClient::new(config);

    let pb = crate::ui::spinner("Checking registry for updates...");

    let mut outdated_deps: Vec<(String, String, String)> = Vec::new();

    for (pkg, dep) in lock_manifest.dependencies {
        // Parse current version
        let current_version = match dep.version.parse::<SemanticVersion>() {
            Ok(v) => v,
            Err(_) => {
                // If it's not a valid semantic version we can't reliably update it here.
                continue;
            }
        };

        let mut split = pkg.split(':');
        let kind = split.next().unwrap_or("");
        let path = split.next().unwrap_or("");

        let path_parts: Vec<&str> = path.split('/').collect();
        if path_parts.len() != 2 {
            continue;
        }

        let account = path_parts[0];
        let slug = path_parts[1];

        let meta_result = if kind == "bundle" {
            client.fetch_bundle_meta(account, slug).await
        } else if kind == "schema" {
            client.fetch_schema_meta(account, slug).await
        } else {
            // Unrecognized dependency sort
            continue;
        };

        if let Ok(meta) = meta_result {
            let mut latest_version = None;

            for v_info in meta.versions {
                if let Ok(v) = v_info.version.parse::<SemanticVersion>() {
                    if let Some(current_latest) = &latest_version {
                        if v > *current_latest {
                            latest_version = Some(v);
                        }
                    } else {
                        latest_version = Some(v);
                    }
                }
            }

            if let Some(latest) = latest_version {
                if latest > current_version {
                    outdated_deps.push((pkg.clone(), dep.version.clone(), latest.to_string()));
                }
            }
        }
    }

    pb.finish_and_clear();

    if outdated_deps.is_empty() {
        println!("{}", "Everything is up to date!".green().bold());
    } else {
        println!("\n{}", "Outdated Schemas".bold());
        let count = outdated_deps.len();
        for (i, (pkg, current, latest)) in outdated_deps.into_iter().enumerate() {
            let is_last = i == count - 1;
            let prefix = if is_last { "└── " } else { "├── " };

            let display_name = if let Some(path) = pkg.strip_prefix("bundle:") {
                format!("bundles/{}", path)
            } else if let Some(path) = pkg.strip_prefix("schema:") {
                path.to_string()
            } else {
                pkg.clone()
            };

            let current_str = format!("@v{}", current).yellow();
            let latest_str = format!("@v{}", latest).green().bold();

            println!("{}{}{} -> {}", prefix.dimmed(), display_name.bold(), current_str, latest_str);
        }
    }

    Ok(())
}
