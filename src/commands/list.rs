use crate::cli::ListArgs;
use crate::manifest::bundle::BundleManifest;
use crate::manifest::lock::LockManifest;
use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::HashSet;
use std::env;
use std::fs;

/// Formats a lock file key (e.g. "schema:acme/foo" or "bundle:acme/bar") for display.
/// Bundles become "bundles/acme/bar" (URL-paste-friendly), schemas stay as "acme/foo".
fn display_name(key: &str) -> String {
    if let Some(path) = key.strip_prefix("bundle:") {
        format!("bundles/{}", path)
    } else if let Some(path) = key.strip_prefix("schema:") {
        path.to_string()
    } else {
        key.to_string()
    }
}

pub async fn run(args: ListArgs) -> Result<()> {
    let cwd = env::current_dir().context("Failed to get current working directory")?;
    let lock_path = cwd.join("rusl.lock");

    if !lock_path.exists() {
        println!("{}", "No schemas installed. `rusl.lock` not found.".yellow());
        return Ok(());
    }

    let lock_str = fs::read_to_string(&lock_path).context("Failed to read rusl.lock")?;
    let lock_manifest: LockManifest =
        toml::from_str(&lock_str).context("Failed to parse rusl.lock")?;

    if lock_manifest.dependencies.is_empty() {
        println!("{}", "No dependencies found in rusl.lock.".yellow());
        return Ok(());
    }

    if args.tree {
        print_tree_view(&cwd, &lock_manifest)?;
    } else {
        print_flat_view(&lock_manifest);
    }

    Ok(())
}

fn print_flat_view(lock: &LockManifest) {
    println!("{}", "rusl.lock".bold());
    let count = lock.dependencies.len();
    for (i, (name, dep)) in lock.dependencies.iter().enumerate() {
        let is_last = i == count - 1;
        let prefix = if is_last { "└── " } else { "├── " };
        let name_display = display_name(name);
        let is_external = !dep.source.contains("rusl.app") && !dep.source.contains("localhost");

        if is_external {
            println!(
                "{}{}{}  ({})",
                prefix.dimmed(),
                name_display.bold(),
                format!("@v{}", dep.version).cyan(),
                dep.source.dimmed()
            );
        } else {
            println!(
                "{}{}{}",
                prefix.dimmed(),
                name_display.bold(),
                format!("@v{}", dep.version).cyan()
            );
        }
    }
}

fn print_tree_view(cwd: &std::path::Path, lock: &LockManifest) -> Result<()> {
    let manifest_path = cwd.join("rusl.bundle.toml");
    if !manifest_path.exists() {
        anyhow::bail!("No rusl.bundle.toml found. Cannot display dependency tree.");
    }

    let manifest_str = fs::read_to_string(&manifest_path)?;
    let manifest: BundleManifest = toml::from_str(&manifest_str)?;

    println!(
        "{}",
        format!("{}@v{}", manifest.bundle.name, manifest.bundle.version).bold()
    );

    let mut root_deps: Vec<String> = Vec::new();
    for name in manifest.schemas.keys() {
        root_deps.push(format!("schema:{}", name));
    }
    for name in manifest.bundles.keys() {
        root_deps.push(format!("bundle:{}", name));
    }
    root_deps.sort();

    let count = root_deps.len();
    let mut seen = HashSet::new();
    for (i, dep_key) in root_deps.iter().enumerate() {
        let is_last = i == count - 1;
        print_tree_node(dep_key, lock, "", is_last, &mut seen);
    }

    Ok(())
}

fn print_tree_node(
    key: &str,
    lock: &LockManifest,
    prefix: &str,
    is_last: bool,
    seen: &mut HashSet<String>,
) {
    let connector = if is_last { "└── " } else { "├── " };
    let name = display_name(key);

    let version_str = lock
        .dependencies
        .get(key)
        .map(|d| format!("@v{}", d.version))
        .unwrap_or_default();

    if seen.contains(key) {
        println!(
            "{}{}{}{} {}",
            prefix,
            connector.dimmed(),
            name.bold(),
            version_str.cyan(),
            "(*)".dimmed()
        );
        return;
    }

    println!(
        "{}{}{}{}",
        prefix,
        connector.dimmed(),
        name.bold(),
        version_str.cyan()
    );

    seen.insert(key.to_string());
    let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

    if let Some(dep) = lock.dependencies.get(key) {
        let children = &dep.dependencies;
        let count = children.len();
        for (i, child) in children.iter().enumerate() {
            let child_is_last = i == count - 1;
            print_tree_node(child, lock, &child_prefix, child_is_last, seen);
        }
    }
}
