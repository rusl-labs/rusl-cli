use crate::manifest::bundle::BundleManifest;
use crate::registry::client::RegistryClient;
use anyhow::{Context, Result};
use pubgrub::{
    DefaultStringReporter, OfflineDependencyProvider, PubGrubError, Ranges, Reporter,
    SemanticVersion, resolve,
};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::str::FromStr;

pub trait ProgressReporter {
    fn set_message(&self, message: String);
    fn println(&self, message: String);
}

/// The result of dependency resolution: resolved versions plus the dependency graph edges.
pub struct ResolvedGraph {
    /// Each resolved package and its exact version (excludes the root package).
    pub versions: BTreeMap<String, SemanticVersion>,
    /// Dependency edges: package key -> list of its direct dependency keys.
    pub edges: HashMap<String, Vec<String>>,
}

fn parse_version_range(req: &str) -> Result<Ranges<SemanticVersion>> {
    if req == "*" || req == ">=0.0.0" || req.trim().is_empty() {
        return Ok(Ranges::full());
    }

    match SemanticVersion::from_str(req.trim_start_matches(|c: char| !c.is_ascii_digit())) {
        Ok(v) => Ok(Ranges::singleton(v)),
        Err(_) => Ok(Ranges::full()),
    }
}

pub async fn resolve_graph<P>(
    manifest: &BundleManifest,
    client: &RegistryClient,
    progress: &P,
) -> Result<ResolvedGraph>
where
    P: ProgressReporter,
{
    progress.set_message("Initializing dependency resolver...".to_string());

    let mut provider = OfflineDependencyProvider::<String, Ranges<SemanticVersion>>::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let root_pkg = format!("bundle:{}", manifest.bundle.name);
    let root_version: SemanticVersion = manifest
        .bundle
        .version
        .parse()
        .context("Root bundle version is not valid semantic version")?;

    let mut version_deps: HashMap<(String, String), Vec<String>> = HashMap::new();
    let mut root_deps = Vec::new();

    for (name, req) in &manifest.schemas {
        let key = format!("schema:{name}");
        let range = parse_version_range(req).unwrap_or(Ranges::full());
        root_deps.push((key.clone(), range));
        if visited.insert(key.clone()) {
            queue.push_back(key);
        }
    }

    for (name, req) in &manifest.bundles {
        let key = format!("bundle:{name}");
        let range = parse_version_range(req).unwrap_or(Ranges::full());
        root_deps.push((key.clone(), range));
        if visited.insert(key.clone()) {
            queue.push_back(key);
        }
    }

    let root_dep_keys: Vec<String> = root_deps.iter().map(|(key, _)| key.clone()).collect();
    version_deps.insert((root_pkg.clone(), root_version.to_string()), root_dep_keys);
    provider.add_dependencies(root_pkg.clone(), root_version, root_deps);

    progress.set_message("Fetching package metadata...".to_string());
    while let Some(package_key) = queue.pop_front() {
        let mut split = package_key.split(':');
        let kind = split.next().unwrap_or("");
        let path = split.next().unwrap_or("");

        let Some((account, slug)) = path.split_once('/') else {
            progress.println(format!("Warning: Invalid package name: {package_key}"));
            continue;
        };

        let metadata = if kind == "bundle" {
            client.fetch_bundle_meta(account, slug).await
        } else {
            client.fetch_schema_meta(account, slug).await
        };

        match metadata {
            Ok(metadata) => {
                for version_info in metadata.versions {
                    if let Ok(version) = version_info.version.parse::<SemanticVersion>() {
                        let mut dependencies = Vec::new();

                        for (dep_name, dep_req) in &version_info.schemas {
                            let dep_key = format!("schema:{dep_name}");
                            let range = parse_version_range(dep_req).unwrap_or(Ranges::full());
                            dependencies.push((dep_key.clone(), range));
                            if visited.insert(dep_key.clone()) {
                                queue.push_back(dep_key);
                            }
                        }

                        for (dep_name, dep_req) in &version_info.bundles {
                            let dep_key = format!("bundle:{dep_name}");
                            let range = parse_version_range(dep_req).unwrap_or(Ranges::full());
                            dependencies.push((dep_key.clone(), range));
                            if visited.insert(dep_key.clone()) {
                                queue.push_back(dep_key);
                            }
                        }

                        let edge_keys: Vec<String> =
                            dependencies.iter().map(|(key, _)| key.clone()).collect();
                        version_deps.insert((package_key.clone(), version.to_string()), edge_keys);
                        provider.add_dependencies(package_key.clone(), version, dependencies);
                    }
                }
            }
            Err(error) => progress.println(format!(
                "Warning: Failed to fetch metadata for {package_key}: {error}"
            )),
        }
    }

    progress.set_message("Resolving version constraints...".to_string());
    match resolve(&provider, root_pkg.clone(), root_version) {
        Ok(resolution) => {
            progress.set_message("Dependency graph resolved!".to_string());
            let versions: BTreeMap<String, SemanticVersion> = resolution
                .into_iter()
                .filter(|(key, _)| key != &root_pkg)
                .collect();

            let mut edges = HashMap::new();
            for (package_key, version) in &versions {
                if let Some(dep_keys) =
                    version_deps.get(&(package_key.clone(), version.to_string()))
                {
                    edges.insert(package_key.clone(), dep_keys.clone());
                }
            }

            Ok(ResolvedGraph { versions, edges })
        }
        Err(PubGrubError::NoSolution(derivation_tree)) => {
            anyhow::bail!(
                "Dependency conflict detected:\n{}",
                DefaultStringReporter::report(&derivation_tree)
            );
        }
        Err(error) => anyhow::bail!("Resolver failed natively: {error}"),
    }
}
