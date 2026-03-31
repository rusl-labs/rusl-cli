use crate::manifest::bundle::BundleManifest;
use crate::registry::client::RegistryClient;
use anyhow::{Context, Result};
use colored::Colorize;
use indicatif::ProgressBar;
use pubgrub::{
    DefaultStringReporter, OfflineDependencyProvider, PubGrubError, Ranges, Reporter,
    SemanticVersion, resolve,
};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::str::FromStr;

/// The result of dependency resolution: resolved versions plus the dependency graph edges.
pub struct ResolvedGraph {
    /// Each resolved package and its exact version (excludes the root package).
    pub versions: BTreeMap<String, SemanticVersion>,
    /// Dependency edges: package key → list of its direct dependency keys.
    pub edges: HashMap<String, Vec<String>>,
}

fn parse_version_range(req: &str) -> Result<Ranges<SemanticVersion>> {
    // Correctly process wildcard and empty constraints mapping to infinite dependency boundaries mathematically
    if req == "*" || req == ">=0.0.0" || req.trim().is_empty() {
        return Ok(Ranges::full());
    }

    // Default parser mapping exact constraints back sequentially for Pubgrub
    match SemanticVersion::from_str(req.trim_start_matches(|c: char| !c.is_ascii_digit())) {
        Ok(v) => Ok(Ranges::singleton(v)),
        Err(_) => Ok(Ranges::full()),
    }
}

pub async fn resolve_graph(
    manifest: &BundleManifest,
    client: &RegistryClient,
    pb: &ProgressBar,
) -> Result<ResolvedGraph> {
    pb.set_message("Initializing dependency resolver...");

    let mut provider = OfflineDependencyProvider::<String, Ranges<SemanticVersion>>::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let root_pkg = format!("bundle:{}", manifest.bundle.name);
    let root_version: SemanticVersion = manifest
        .bundle
        .version
        .parse()
        .context("Root bundle version is not valid semantic version")?;

    // Track dependency edges: (package, version_string) → list of dep keys
    let mut version_deps: HashMap<(String, String), Vec<String>> = HashMap::new();

    let mut root_deps = Vec::new();

    for (name, req) in manifest.schemas.iter() {
        let key = format!("schema:{}", name);
        let range = parse_version_range(req).unwrap_or(Ranges::full());
        root_deps.push((key.clone(), range));
        if !visited.contains(&key) {
            queue.push_back(key.clone());
            visited.insert(key.clone());
        }
    }

    for (name, req) in manifest.bundles.iter() {
        let key = format!("bundle:{}", name);
        let range = parse_version_range(req).unwrap_or(Ranges::full());
        root_deps.push((key.clone(), range));
        if !visited.contains(&key) {
            queue.push_back(key.clone());
            visited.insert(key.clone());
        }
    }

    let root_dep_keys: Vec<String> = root_deps.iter().map(|(k, _)| k.clone()).collect();
    version_deps.insert((root_pkg.clone(), root_version.to_string()), root_dep_keys);
    provider.add_dependencies(root_pkg.clone(), root_version, root_deps);

    pb.set_message("Fetching package metadata...");
    while let Some(pkg) = queue.pop_front() {
        // We structurally suppress deeper dynamic tracing logs here as they cause severe log spam in resolving trees

        let mut split = pkg.split(':');
        let kind = split.next().unwrap_or("");
        let path = split.next().unwrap_or("");

        let path_parts: Vec<&str> = path.split('/').collect();
        if path_parts.len() != 2 {
            pb.println(format!(
                "{} Invalid package name: {}",
                "Warning:".yellow().bold(),
                pkg
            ));
            continue;
        }

        let account = path_parts[0];
        let slug = path_parts[1];

        let meta_result = if kind == "bundle" {
            client.fetch_bundle_meta(account, slug).await
        } else {
            client.fetch_schema_meta(account, slug).await
        };

        match meta_result {
            Ok(meta) => {
                // Instantly absorb the entire publishing history of the dependency into our local RAM map
                for v_info in meta.versions {
                    if let Ok(v) = v_info.version.parse::<SemanticVersion>() {
                        let mut deps = Vec::new();

                        for (dep_name, dep_req) in v_info.schemas.iter() {
                            let key = format!("schema:{}", dep_name);
                            let range = parse_version_range(dep_req).unwrap_or(Ranges::full());
                            deps.push((key.clone(), range));
                            if !visited.contains(&key) {
                                queue.push_back(key.clone());
                                visited.insert(key.clone());
                            }
                        }

                        for (dep_name, dep_req) in v_info.bundles.iter() {
                            let key = format!("bundle:{}", dep_name);
                            let range = parse_version_range(dep_req).unwrap_or(Ranges::full());
                            deps.push((key.clone(), range));
                            if !visited.contains(&key) {
                                queue.push_back(key.clone());
                                visited.insert(key.clone());
                            }
                        }

                        let edge_keys: Vec<String> = deps.iter().map(|(k, _)| k.clone()).collect();
                        version_deps.insert((pkg.clone(), v.to_string()), edge_keys);
                        provider.add_dependencies(pkg.clone(), v, deps);
                    }
                }
            }
            Err(e) => pb.println(format!(
                "{} Failed to fetch metadata for {}: {}",
                "Warning:".yellow().bold(),
                pkg,
                e
            )),
        }
    }

    pb.set_message("Resolving version constraints...");
    match resolve(&provider, root_pkg.clone(), root_version) {
        Ok(resolution) => {
            pb.set_message("Dependency graph resolved!");
            let map: BTreeMap<String, SemanticVersion> = resolution
                .into_iter()
                .filter(|(k, _)| k != &root_pkg)
                .collect();

            // Build edges for resolved packages by looking up their deps at the resolved version
            let mut edges = HashMap::new();
            for (pkg, version) in &map {
                if let Some(dep_keys) = version_deps.get(&(pkg.clone(), version.to_string())) {
                    edges.insert(pkg.clone(), dep_keys.clone());
                }
            }

            Ok(ResolvedGraph {
                versions: map,
                edges,
            })
        }
        Err(PubGrubError::NoSolution(derivation_tree)) => {
            anyhow::bail!(
                "Dependency conflict detected:\n{}",
                DefaultStringReporter::report(&derivation_tree)
            );
        }
        Err(e) => anyhow::bail!("Resolver failed natively: {}", e),
    }
}
