use crate::manifest::bundle::BundleManifest;
use crate::registry::client::RegistryClient;
use crate::resource_identifier::{
    RegistryResource, ResourceKind, package_key_from_identifier, parse_package_key,
};
use anyhow::{Context, Result};
use pubgrub::{
    DefaultStringReporter, OfflineDependencyProvider, PubGrubError, Ranges, Reporter,
    SemanticVersion, resolve,
};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::str::FromStr;

const LOCAL_ROOT_PACKAGE: &str = "__local__/bundles/root";
const LOCAL_ROOT_VERSION: &str = "0.0.0";

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

    // The local project is anonymous to the resolver. `[bundle].name` is a free-form local label,
    // not a published bundle identifier, so the root node is always the local sentinel.
    let root_pkg = format!("bundle:{LOCAL_ROOT_PACKAGE}");
    let root_version: SemanticVersion = manifest
        .bundle
        .version
        .as_deref()
        .unwrap_or(LOCAL_ROOT_VERSION)
        .parse()
        .context("Root bundle version is not valid semantic version")?;

    let mut version_deps: HashMap<(String, String), Vec<String>> = HashMap::new();
    let mut root_deps = Vec::new();

    for (identifier, req) in &manifest.rusl.resources {
        let Some(key) = package_key_from_identifier(identifier) else {
            progress.println(format!(
                "Warning: Invalid resource identifier: {identifier}"
            ));
            continue;
        };
        let range = parse_version_range(req.version()).unwrap_or(Ranges::full());
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
        let Some(target) = parse_package_key(&package_key) else {
            progress.println(format!("Warning: Invalid package name: {package_key}"));
            continue;
        };

        let metadata = match target.kind {
            ResourceKind::Bundle => {
                client
                    .fetch_bundle_meta(&target.account, &target.slug)
                    .await
            }
            ResourceKind::Schema => {
                client
                    .fetch_schema_meta(&target.account, &target.slug)
                    .await
            }
        };

        match metadata {
            Ok(metadata) => {
                for version_info in metadata.versions {
                    if let Ok(version) = version_info.version.parse::<SemanticVersion>() {
                        let mut dependencies = Vec::new();

                        for (dep_name, dep_req) in &version_info.schemas {
                            let Some(dep_key) = dependency_key(ResourceKind::Schema, dep_name)
                            else {
                                progress.println(format!(
                                    "Warning: Invalid schema dependency identifier: {dep_name}"
                                ));
                                continue;
                            };
                            let range = parse_version_range(dep_req).unwrap_or(Ranges::full());
                            dependencies.push((dep_key.clone(), range));
                            if visited.insert(dep_key.clone()) {
                                queue.push_back(dep_key);
                            }
                        }

                        for (dep_name, dep_req) in &version_info.bundles {
                            let Some(dep_key) = dependency_key(ResourceKind::Bundle, dep_name)
                            else {
                                progress.println(format!(
                                    "Warning: Invalid bundle dependency identifier: {dep_name}"
                                ));
                                continue;
                            };
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

fn dependency_key(kind: ResourceKind, identifier: &str) -> Option<String> {
    match kind {
        ResourceKind::Schema => RegistryResource::schema(identifier),
        ResourceKind::Bundle => RegistryResource::bundle(identifier),
    }
    .map(|resource| resource.package_key())
}

#[cfg(test)]
mod tests {
    use super::{ProgressReporter, resolve_graph};
    use crate::{
        config::Config, manifest::bundle::BundleManifest, registry::client::RegistryClient,
    };
    use axum::{
        Json, Router,
        extract::{Path as AxumPath, State},
        http::StatusCode,
        routing::get,
    };
    use serde_json::{Value, json};
    use serial_test::serial;
    use tokio::{net::TcpListener, sync::oneshot, task::JoinHandle};

    #[derive(Default)]
    struct TestProgress {
        messages: std::sync::Mutex<Vec<String>>,
    }

    impl ProgressReporter for TestProgress {
        fn set_message(&self, message: String) {
            self.messages.lock().expect("lock messages").push(message);
        }

        fn println(&self, message: String) {
            self.messages.lock().expect("lock messages").push(message);
        }
    }

    #[derive(Clone)]
    struct TestState;

    struct TestServer {
        base_url: String,
        shutdown: Option<oneshot::Sender<()>>,
        task: JoinHandle<()>,
    }

    impl TestServer {
        async fn start() -> Self {
            let app = Router::new()
                .route(
                    "/resources/{account}/schemas/{slug}/metadata",
                    get(schema_metadata_handler),
                )
                .route(
                    "/resources/{account}/bundles/{slug}/metadata",
                    get(bundle_metadata_handler),
                )
                .with_state(TestState);

            let listener = TcpListener::bind("127.0.0.1:0")
                .await
                .expect("bind test server");
            let address = listener.local_addr().expect("read test server address");
            let (shutdown_tx, shutdown_rx) = oneshot::channel();
            let task = tokio::spawn(async move {
                axum::serve(listener, app)
                    .with_graceful_shutdown(async {
                        let _ = shutdown_rx.await;
                    })
                    .await
                    .expect("run test server");
            });

            Self {
                base_url: format!("http://{address}"),
                shutdown: Some(shutdown_tx),
                task,
            }
        }
    }

    impl Drop for TestServer {
        fn drop(&mut self) {
            if let Some(shutdown) = self.shutdown.take() {
                let _ = shutdown.send(());
            }
            self.task.abort();
        }
    }

    #[tokio::test]
    #[serial]
    async fn resolves_schema_dependencies_from_registry_metadata() {
        let server = TestServer::start().await;
        let client = RegistryClient::new(Config {
            api_base_url: server.base_url.clone(),
            website_url: "https://example.test".to_string(),
            ..Config::default()
        });
        let manifest: BundleManifest = toml::from_str(
            r#"
[bundle]
name = "hassox/demo"
version = "0.1.0"

[rusl.resources]
"hassox/schemas/root" = ">=1.0.0"
"#,
        )
        .expect("parse manifest");
        let progress = TestProgress::default();

        let resolved = resolve_graph(&manifest, &client, &progress)
            .await
            .expect("resolve graph");

        assert_eq!(
            resolved.versions["schema:hassox/schemas/root"].to_string(),
            "1.0.0"
        );
        assert_eq!(
            resolved.versions["schema:hassox/schemas/dep"].to_string(),
            "1.0.0"
        );
        assert_eq!(
            resolved.edges["schema:hassox/schemas/root"],
            vec!["schema:hassox/schemas/dep".to_string()]
        );
    }

    #[tokio::test]
    #[serial]
    async fn reports_conflicts_when_no_single_version_satisfies_all_dependencies() {
        let server = TestServer::start().await;
        let client = RegistryClient::new(Config {
            api_base_url: server.base_url.clone(),
            website_url: "https://example.test".to_string(),
            ..Config::default()
        });
        let manifest: BundleManifest = toml::from_str(
            r#"
[bundle]
name = "hassox/demo"
version = "0.1.0"

[rusl.resources]
"hassox/schemas/root" = ">=1.0.0"
"hassox/bundles/consumer" = ">=1.0.0"
"#,
        )
        .expect("parse manifest");
        let progress = TestProgress::default();

        let error = resolve_graph(&manifest, &client, &progress)
            .await
            .err()
            .expect("expected dependency conflict");

        assert!(error.to_string().contains("Dependency conflict detected"));
    }

    async fn schema_metadata_handler(
        State(_state): State<TestState>,
        AxumPath((account, slug)): AxumPath<(String, String)>,
    ) -> (StatusCode, Json<Value>) {
        let body = match (account.as_str(), slug.as_str()) {
            ("hassox", "root") => json!({
                "name": "root",
                "versions": [{
                    "version": "1.0.0",
                    "schemas": { "hassox/schemas/dep": ">=1.0.0" },
                    "bundles": {}
                }]
            }),
            ("hassox", "dep") => json!({
                "name": "dep",
                "versions": [
                    { "version": "1.0.0", "schemas": {}, "bundles": {} },
                    { "version": "2.0.0", "schemas": {}, "bundles": {} }
                ]
            }),
            _ => json!({ "error": "not found" }),
        };
        let status = if body.get("error").is_some() {
            StatusCode::NOT_FOUND
        } else {
            StatusCode::OK
        };
        (status, Json(body))
    }

    async fn bundle_metadata_handler(
        State(_state): State<TestState>,
        AxumPath((account, slug)): AxumPath<(String, String)>,
    ) -> (StatusCode, Json<Value>) {
        let body = match (account.as_str(), slug.as_str()) {
            ("hassox", "consumer") => json!({
                "name": "consumer",
                "versions": [{
                    "version": "1.0.0",
                    "schemas": { "hassox/schemas/dep": ">=2.0.0" },
                    "bundles": {}
                }]
            }),
            _ => json!({ "error": "not found" }),
        };
        let status = if body.get("error").is_some() {
            StatusCode::NOT_FOUND
        } else {
            StatusCode::OK
        };
        (status, Json(body))
    }
}
