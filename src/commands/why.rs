use crate::cli::WhyArgs;
use crate::manifest::bundle::BundleManifest;
use crate::manifest::lock::LockManifest;
use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

fn display_name(key: &str) -> String {
    if let Some(path) = key.strip_prefix("bundle:") {
        format!("bundles/{}", path)
    } else if let Some(path) = key.strip_prefix("schema:") {
        path.to_string()
    } else {
        key.to_string()
    }
}

/// Resolves a user-provided search term to a lock file key.
/// Tries: exact key, schema: prefix, bundle: prefix, display name match.
fn resolve_search_key(search: &str, lock: &LockManifest) -> Option<String> {
    if lock.dependencies.contains_key(search) {
        return Some(search.to_string());
    }
    let schema_key = format!("schema:{}", search);
    if lock.dependencies.contains_key(&schema_key) {
        return Some(schema_key);
    }
    let bundle_key = format!("bundle:{}", search);
    if lock.dependencies.contains_key(&bundle_key) {
        return Some(bundle_key);
    }
    for key in lock.dependencies.keys() {
        if display_name(key) == search {
            return Some(key.clone());
        }
    }
    None
}

/// Finds all nodes that are on a path from any root dep to the target.
fn find_ancestors(target: &str, lock: &LockManifest) -> HashSet<String> {
    let mut reverse: HashMap<&str, Vec<&str>> = HashMap::new();
    for (key, dep) in &lock.dependencies {
        for child in &dep.dependencies {
            reverse.entry(child.as_str()).or_default().push(key.as_str());
        }
    }

    let mut ancestors = HashSet::new();
    ancestors.insert(target.to_string());
    let mut queue = VecDeque::new();
    queue.push_back(target.to_string());
    while let Some(node) = queue.pop_front() {
        if let Some(parents) = reverse.get(node.as_str()) {
            for &parent in parents {
                if ancestors.insert(parent.to_string()) {
                    queue.push_back(parent.to_string());
                }
            }
        }
    }
    ancestors
}

pub async fn run(args: WhyArgs) -> Result<()> {
    let cwd = env::current_dir().context("Failed to get current working directory")?;
    let lock_path = cwd.join("rusl.lock");

    if !lock_path.exists() {
        println!("{}", "No schemas installed. `rusl.lock` not found.".yellow());
        return Ok(());
    }

    let lock_str = fs::read_to_string(&lock_path).context("Failed to read rusl.lock")?;
    let lock: LockManifest = toml::from_str(&lock_str).context("Failed to parse rusl.lock")?;

    let target = match resolve_search_key(&args.package, &lock) {
        Some(key) => key,
        None => {
            println!(
                "{} Package {} not found in rusl.lock.",
                "Error:".red().bold(),
                args.package.bold()
            );
            return Ok(());
        }
    };

    let manifest_path = cwd.join("rusl.bundle.toml");
    if !manifest_path.exists() {
        anyhow::bail!("No rusl.bundle.toml found. Cannot trace dependency paths.");
    }

    let manifest_str = fs::read_to_string(&manifest_path)?;
    let manifest: BundleManifest = toml::from_str(&manifest_str)?;

    let mut root_deps: Vec<String> = Vec::new();
    for name in manifest.schemas.keys() {
        root_deps.push(format!("schema:{}", name));
    }
    for name in manifest.bundles.keys() {
        root_deps.push(format!("bundle:{}", name));
    }
    root_deps.sort();

    let ancestors = find_ancestors(&target, &lock);

    let relevant_roots: Vec<&String> = root_deps
        .iter()
        .filter(|d| ancestors.contains(d.as_str()))
        .collect();

    if relevant_roots.is_empty() {
        println!(
            "{} {} is in the lockfile but not reachable from root dependencies.",
            "Warning:".yellow().bold(),
            display_name(&target).bold()
        );
        return Ok(());
    }

    let target_display = display_name(&target);
    let target_version = lock
        .dependencies
        .get(&target)
        .map(|d| format!("@v{}", d.version))
        .unwrap_or_default();

    println!(
        "{} {}{}",
        "why:".green().bold(),
        target_display.bold(),
        target_version.cyan()
    );
    println!();

    println!(
        "{}",
        format!("{}@v{}", manifest.bundle.name, manifest.bundle.version).bold()
    );

    let count = relevant_roots.len();
    for (i, dep_key) in relevant_roots.iter().enumerate() {
        let is_last = i == count - 1;
        print_search_node(dep_key, &target, &lock, &ancestors, "", is_last);
    }

    Ok(())
}

fn print_search_node(
    key: &str,
    target: &str,
    lock: &LockManifest,
    ancestors: &HashSet<String>,
    prefix: &str,
    is_last: bool,
) {
    let connector = if is_last { "└── " } else { "├── " };
    let name = display_name(key);
    let version_str = lock
        .dependencies
        .get(key)
        .map(|d| format!("@v{}", d.version))
        .unwrap_or_default();

    if key == target {
        println!(
            "{}{}{}{}",
            prefix,
            connector.dimmed(),
            name.bold().underline(),
            version_str.cyan()
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

    let child_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });

    if let Some(dep) = lock.dependencies.get(key) {
        let relevant_children: Vec<&String> = dep
            .dependencies
            .iter()
            .filter(|c| ancestors.contains(c.as_str()))
            .collect();
        let count = relevant_children.len();
        for (i, child) in relevant_children.iter().enumerate() {
            let child_is_last = i == count - 1;
            print_search_node(child, target, lock, ancestors, &child_prefix, child_is_last);
        }
    }
}
