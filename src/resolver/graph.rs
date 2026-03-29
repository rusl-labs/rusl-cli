use crate::manifest::bundle::BundleManifest;
use crate::registry::client::RegistryClient;
use anyhow::{Context, Result};
use pubgrub::{
    DefaultStringReporter, OfflineDependencyProvider, PubGrubError, Ranges, Reporter,
    SemanticVersion, resolve,
};
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::str::FromStr;
use tracing::{debug, info, warn};

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
) -> Result<BTreeMap<String, SemanticVersion>> {
    info!("Starting dependency resolution algorithm...");

    let mut provider = OfflineDependencyProvider::<String, Ranges<SemanticVersion>>::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let root_pkg = format!("bundle:{}", manifest.bundle.name);
    let root_version: SemanticVersion = manifest
        .bundle
        .version
        .parse()
        .context("Root bundle version is not valid semantic version")?;

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

    provider.add_dependencies(root_pkg.clone(), root_version, root_deps);

    info!("Fetching transitive dependencies from registry metadata indexes...");
    while let Some(pkg) = queue.pop_front() {
        debug!("Fetching full dynamic version matrix natively for: {}", pkg);

        let mut split = pkg.split(':');
        let kind = split.next().unwrap_or("");
        let path = split.next().unwrap_or("");

        let path_parts: Vec<&str> = path.split('/').collect();
        if path_parts.len() != 2 {
            warn!("Invalid package name format '{}'", pkg);
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

                        provider.add_dependencies(pkg.clone(), v, deps);
                    }
                }
            }
            Err(e) => warn!(
                "Failed to fetch metadata index {}: {}. Will ignore constraints branch.",
                pkg, e
            ),
        }
    }

    info!("Executing Native PubGrub Constraint Solver Math...");
    match resolve(&provider, root_pkg.clone(), root_version) {
        Ok(resolution) => {
            info!("Successfully built mathematical dependency graph!");
            let map: BTreeMap<String, SemanticVersion> = resolution
                .into_iter()
                .filter(|(k, _)| k != &root_pkg)
                .collect();
            Ok(map)
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
