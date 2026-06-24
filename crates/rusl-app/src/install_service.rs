use crate::cache::linker::Linker;
use crate::cache::store::GlobalStore;
use crate::config;
use crate::manifest::bundle::BundleManifest;
use crate::manifest::lock::{LockDependency, LockManifest};
use crate::registry::client::RegistryClient;
use crate::resolver::graph::{ProgressReporter, resolve_graph};
use crate::resource_identifier::{ResourceKind, parse_package_key};
use anyhow::{Context, Result, bail};
use std::collections::{BTreeMap, HashMap};

const LOCAL_BUNDLE_NAME: &str = "local bundle";
const LOCAL_BUNDLE_VERSION: &str = "unversioned";

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
    let linker = Linker::new(cwd.clone(), config.schema_dir(), config.output_suffix());
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
        manifest.bundle.name.as_deref().unwrap_or(LOCAL_BUNDLE_NAME),
        manifest
            .bundle
            .version
            .as_deref()
            .unwrap_or(LOCAL_BUNDLE_VERSION)
    ));
    let resolved = resolve_graph(&manifest, &client, progress)
        .await
        .context("Dependency resolution failed")?;

    progress.set_message("Cleaning old schema cache...".to_string());
    linker
        .purge_all()
        .with_context(|| format!("Failed to safely prune {} directory", config.schema_dir()))?;

    progress.set_message("Downloading schemas...".to_string());

    let mut schema_count = 0;
    let mut integrity_map: HashMap<String, String> = HashMap::new();

    for (package_key, version) in &resolved.versions {
        let Some(resource) = parse_package_key(package_key) else {
            progress.println(format!(
                "Warning: Skipping invalid package key: {package_key}"
            ));
            continue;
        };
        if resource.kind != ResourceKind::Schema {
            continue;
        }

        progress.set_message(format!(
            "Downloading {}@{version} ...",
            resource.identifier()
        ));
        let blob = client
            .download_schema_blob(&resource.account, &resource.slug, &version.to_string())
            .await?;

        let (integrity, cas_path) = store.put(&blob).await?;
        integrity_map.insert(package_key.clone(), integrity);
        linker.link_schema(&resource.identifier(), &cas_path)?;
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

#[cfg(test)]
mod tests {
    use super::install_project;
    use crate::manifest::lock::LockManifest;
    use crate::resolver::graph::ProgressReporter;
    use axum::{
        Json, Router,
        extract::{Path as AxumPath, State},
        http::{HeaderMap, StatusCode},
        routing::get,
    };
    use serde_json::{Value, json};
    use serial_test::serial;
    use std::{ffi::OsString, path::PathBuf, sync::Arc};
    use tempfile::TempDir;
    use tokio::{
        net::TcpListener,
        sync::{Mutex, oneshot},
        task::JoinHandle,
    };

    #[derive(Default)]
    struct TestProgress;

    impl ProgressReporter for TestProgress {
        fn set_message(&self, _message: String) {}
        fn println(&self, _message: String) {}
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct RecordedRequest {
        path: String,
        authorization: Option<String>,
    }

    #[derive(Clone)]
    struct TestState {
        requests: Arc<Mutex<Vec<RecordedRequest>>>,
    }

    struct TestServer {
        base_url: String,
        requests: Arc<Mutex<Vec<RecordedRequest>>>,
        shutdown: Option<oneshot::Sender<()>>,
        task: JoinHandle<()>,
    }

    impl TestServer {
        async fn start() -> Self {
            let requests = Arc::new(Mutex::new(Vec::new()));
            let state = TestState {
                requests: requests.clone(),
            };

            let app = Router::new()
                .route(
                    "/resources/{account}/schemas/{slug}/metadata",
                    get(metadata_handler),
                )
                .route(
                    "/resources/{account}/schemas/{slug_and_version}",
                    get(schema_handler),
                )
                .with_state(state);

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
                requests,
                shutdown: Some(shutdown_tx),
                task,
            }
        }

        async fn recorded_requests(&self) -> Vec<RecordedRequest> {
            self.requests.lock().await.clone()
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

    struct EnvGuard {
        previous_home: Option<OsString>,
        previous_xdg_config_home: Option<OsString>,
        previous_xdg_data_home: Option<OsString>,
        previous_api_url: Option<OsString>,
        previous_dir: PathBuf,
    }

    impl EnvGuard {
        fn new(
            home_dir: &std::path::Path,
            workspace_dir: &std::path::Path,
            api_base_url: &str,
        ) -> Self {
            let previous_home = std::env::var_os(home_var_name());
            let previous_xdg_config_home = std::env::var_os("XDG_CONFIG_HOME");
            let previous_xdg_data_home = std::env::var_os("XDG_DATA_HOME");
            let previous_api_url = std::env::var_os("RUSL_API_URL");
            let previous_dir = std::env::current_dir().expect("current dir");

            unsafe { std::env::set_var(home_var_name(), home_dir.as_os_str()) };
            unsafe { std::env::set_var("XDG_CONFIG_HOME", home_dir.join(".config")) };
            unsafe { std::env::set_var("XDG_DATA_HOME", home_dir.join(".local").join("share")) };
            unsafe { std::env::set_var("RUSL_API_URL", api_base_url) };
            std::env::set_current_dir(workspace_dir).expect("set current dir");

            Self {
                previous_home,
                previous_xdg_config_home,
                previous_xdg_data_home,
                previous_api_url,
                previous_dir,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match self.previous_home.as_ref() {
                Some(value) => unsafe { std::env::set_var(home_var_name(), value) },
                None => unsafe { std::env::remove_var(home_var_name()) },
            }
            match self.previous_xdg_config_home.as_ref() {
                Some(value) => unsafe { std::env::set_var("XDG_CONFIG_HOME", value) },
                None => unsafe { std::env::remove_var("XDG_CONFIG_HOME") },
            }
            match self.previous_xdg_data_home.as_ref() {
                Some(value) => unsafe { std::env::set_var("XDG_DATA_HOME", value) },
                None => unsafe { std::env::remove_var("XDG_DATA_HOME") },
            }
            match self.previous_api_url.as_ref() {
                Some(value) => unsafe { std::env::set_var("RUSL_API_URL", value) },
                None => unsafe { std::env::remove_var("RUSL_API_URL") },
            }
            std::env::set_current_dir(&self.previous_dir).expect("restore current dir");
        }
    }

    #[tokio::test]
    #[serial]
    async fn installs_schemas_and_writes_lockfile_from_registry_metadata() {
        let server = TestServer::start().await;
        let temp_dir = TempDir::new().expect("create temp dir");
        let home_dir = temp_dir.path().join("home");
        let workspace_dir = temp_dir.path().join("workspace");
        std::fs::create_dir_all(&home_dir).expect("create home dir");
        std::fs::create_dir_all(&workspace_dir).expect("create workspace dir");
        let _guard = EnvGuard::new(&home_dir, &workspace_dir, &server.base_url);

        std::fs::write(
            workspace_dir.join("rusl.bundle.toml"),
            r#"
[rusl.resources]
"hassox/schemas/root" = ">=1.0.0"
"#,
        )
        .expect("write manifest");
        std::fs::write(
            workspace_dir.join("rusl.config.toml"),
            r#"
schema_dir = "schemas/vendor"
"#,
        )
        .expect("write config");

        let result = install_project(&TestProgress)
            .await
            .expect("install project");

        assert_eq!(result.schema_count, 2);

        let root_schema = workspace_dir
            .join("schemas")
            .join("vendor")
            .join("hassox")
            .join("schemas")
            .join("root.schema.json");
        let dep_schema = workspace_dir
            .join("schemas")
            .join("vendor")
            .join("hassox")
            .join("schemas")
            .join("dep.schema.json");
        assert_eq!(
            std::fs::read_to_string(&root_schema).expect("root schema"),
            r#"{"title":"root","type":"object"}"#
        );
        assert_eq!(
            std::fs::read_to_string(&dep_schema).expect("dep schema"),
            r#"{"title":"dep","type":"object"}"#
        );

        let lock: LockManifest = toml::from_str(
            &std::fs::read_to_string(workspace_dir.join("rusl.lock")).expect("read lockfile"),
        )
        .expect("parse lockfile");
        assert_eq!(lock.dependencies.len(), 2);
        assert_eq!(
            lock.dependencies["schema:hassox/schemas/root"].dependencies,
            vec!["schema:hassox/schemas/dep".to_string()]
        );
        assert_eq!(
            lock.dependencies["schema:hassox/schemas/root"].source,
            server.base_url
        );
        assert!(
            !lock.dependencies["schema:hassox/schemas/root"]
                .integrity
                .is_empty()
        );
        assert!(
            !lock.dependencies["schema:hassox/schemas/dep"]
                .integrity
                .is_empty()
        );

        assert_eq!(
            server.recorded_requests().await,
            vec![
                RecordedRequest {
                    path: "/resources/hassox/schemas/root/metadata".to_string(),
                    authorization: None,
                },
                RecordedRequest {
                    path: "/resources/hassox/schemas/dep/metadata".to_string(),
                    authorization: None,
                },
                RecordedRequest {
                    path: "/resources/hassox/schemas/dep@v1.0.0".to_string(),
                    authorization: None,
                },
                RecordedRequest {
                    path: "/resources/hassox/schemas/root@v1.0.0".to_string(),
                    authorization: None,
                },
            ]
        );
    }

    async fn metadata_handler(
        State(state): State<TestState>,
        AxumPath((account, slug)): AxumPath<(String, String)>,
        headers: HeaderMap,
    ) -> (StatusCode, Json<Value>) {
        record_request(
            &state,
            format!("/resources/{account}/schemas/{slug}/metadata"),
            &headers,
        )
        .await;
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
                "versions": [{
                    "version": "1.0.0",
                    "schemas": {},
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

    async fn schema_handler(
        State(state): State<TestState>,
        AxumPath((account, slug_and_version)): AxumPath<(String, String)>,
        headers: HeaderMap,
    ) -> (StatusCode, Json<Value>) {
        record_request(
            &state,
            format!("/resources/{account}/schemas/{slug_and_version}"),
            &headers,
        )
        .await;
        let body = match (account.as_str(), slug_and_version.as_str()) {
            ("hassox", "root@v1.0.0") => json!({ "title": "root", "type": "object" }),
            ("hassox", "dep@v1.0.0") => json!({ "title": "dep", "type": "object" }),
            _ => json!({ "error": "not found" }),
        };
        let status = if body.get("error").is_some() {
            StatusCode::NOT_FOUND
        } else {
            StatusCode::OK
        };
        (status, Json(body))
    }

    async fn record_request(state: &TestState, path: String, headers: &HeaderMap) {
        state.requests.lock().await.push(RecordedRequest {
            path,
            authorization: headers
                .get("authorization")
                .and_then(|value| value.to_str().ok())
                .map(ToOwned::to_owned),
        });
    }

    #[cfg(windows)]
    fn home_var_name() -> &'static str {
        "USERPROFILE"
    }

    #[cfg(not(windows))]
    fn home_var_name() -> &'static str {
        "HOME"
    }
}
