use crate::config;
use crate::manifest::lock::LockManifest;
use crate::registry::client::{RegistryClient, RegistryVersion};
use crate::resource_identifier::{ResourceKind, display_package_key, parse_package_key};
use anyhow::{Context, Result};
use pubgrub::SemanticVersion;
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutdatedOutput {
    MissingLockfile,
    EmptyLockfile,
    Items(Vec<OutdatedItem>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutdatedItem {
    pub package_key: String,
    pub display_name: String,
    pub current_version: String,
    pub latest_version: String,
}

pub async fn load_outdated_dependencies() -> Result<OutdatedOutput> {
    let cwd = env::current_dir().context("Failed to get current working directory")?;
    let lock_path = cwd.join("rusl.lock");

    if !lock_path.exists() {
        return Ok(OutdatedOutput::MissingLockfile);
    }

    let lock_str = fs::read_to_string(&lock_path).context("Failed to read rusl.lock")?;
    let lock_manifest: LockManifest =
        toml::from_str(&lock_str).context("Failed to parse rusl.lock")?;

    if lock_manifest.dependencies.is_empty() {
        return Ok(OutdatedOutput::EmptyLockfile);
    }

    let config = config::load().context("Failed to load configuration")?;
    let client = RegistryClient::new(config);
    let mut outdated = Vec::new();

    for (package_key, dependency) in lock_manifest.dependencies {
        let Some(current_version) = parse_version(&dependency.version) else {
            continue;
        };
        let Some(target) = parse_registry_target(&package_key) else {
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

        let Ok(metadata) = metadata else {
            continue;
        };

        let Some(latest_version) = latest_version(&metadata.versions) else {
            continue;
        };

        if latest_version > current_version {
            outdated.push(OutdatedItem {
                package_key,
                display_name: target.display_name,
                current_version: dependency.version,
                latest_version: latest_version.to_string(),
            });
        }
    }

    Ok(OutdatedOutput::Items(outdated))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RegistryTarget {
    kind: ResourceKind,
    account: String,
    slug: String,
    display_name: String,
}

fn parse_registry_target(package_key: &str) -> Option<RegistryTarget> {
    let resource = parse_package_key(package_key)?;

    Some(RegistryTarget {
        kind: resource.kind,
        account: resource.account,
        slug: resource.slug,
        display_name: display_package_key(package_key),
    })
}

fn latest_version(versions: &[RegistryVersion]) -> Option<SemanticVersion> {
    versions
        .iter()
        .filter_map(|version| parse_version(&version.version))
        .max()
}

fn parse_version(version: &str) -> Option<SemanticVersion> {
    version.parse::<SemanticVersion>().ok()
}

#[cfg(test)]
mod tests {
    use super::{
        OutdatedItem, OutdatedOutput, latest_version, load_outdated_dependencies,
        parse_registry_target,
    };
    use crate::config::credentials::Credentials;
    use crate::registry::client::RegistryVersion;
    use crate::resource_identifier::display_package_key;
    use axum::{
        Json, Router,
        extract::{Path as AxumPath, State},
        http::StatusCode,
        routing::get,
    };
    use serde_json::{Value, json};
    use serial_test::serial;
    use std::{collections::HashMap, ffi::OsString, path::PathBuf};
    use tempfile::TempDir;
    use tokio::{net::TcpListener, sync::oneshot, task::JoinHandle};

    #[test]
    fn formats_registry_display_names() {
        assert_eq!(
            display_package_key("bundle:acme/common"),
            "acme/bundles/common"
        );
        assert_eq!(display_package_key("schema:acme/types"), "acme/types");
    }

    #[test]
    fn parses_registry_targets() {
        let target = parse_registry_target("bundle:acme/common").expect("target should parse");

        assert_eq!(target.account, "acme");
        assert_eq!(target.slug, "common");
        assert_eq!(target.display_name, "acme/bundles/common");
        assert!(parse_registry_target("external:https://example.com/schema.json").is_none());
    }

    #[test]
    fn finds_latest_semantic_version() {
        let versions = vec![
            sample_version("1.2.0"),
            sample_version("2.0.0"),
            sample_version("not-a-version"),
            sample_version("1.5.1"),
        ];

        let latest = latest_version(&versions).expect("latest version should exist");
        assert_eq!(latest.to_string(), "2.0.0");
    }

    #[test]
    fn outdated_item_captures_render_data() {
        let item = OutdatedItem {
            package_key: "schema:acme/types".to_string(),
            display_name: "acme/types".to_string(),
            current_version: "1.0.0".to_string(),
            latest_version: "1.1.0".to_string(),
        };

        assert_eq!(item.display_name, "acme/types");
        assert_eq!(item.current_version, "1.0.0");
        assert_eq!(item.latest_version, "1.1.0");
    }

    fn sample_version(version: &str) -> RegistryVersion {
        RegistryVersion {
            version: version.to_string(),
            schemas: HashMap::new(),
            bundles: HashMap::new(),
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
                    "/resources/{account}/{slug}/metadata",
                    get(metadata_handler),
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

    struct EnvGuard {
        previous_home: Option<OsString>,
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
            let previous_api_url = std::env::var_os("RUSL_API_URL");
            let previous_dir = std::env::current_dir().expect("current dir");
            unsafe { std::env::set_var(home_var_name(), home_dir.as_os_str()) };
            unsafe { std::env::set_var("RUSL_API_URL", api_base_url) };
            std::env::set_current_dir(workspace_dir).expect("set current dir");
            Self {
                previous_home,
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
            match self.previous_api_url.as_ref() {
                Some(value) => unsafe { std::env::set_var("RUSL_API_URL", value) },
                None => unsafe { std::env::remove_var("RUSL_API_URL") },
            }
            std::env::set_current_dir(&self.previous_dir).expect("restore current dir");
        }
    }

    #[tokio::test]
    #[serial]
    async fn returns_missing_lockfile_when_no_lock_exists() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let home_dir = temp_dir.path().join("home");
        let workspace_dir = temp_dir.path().join("workspace");
        std::fs::create_dir_all(&home_dir).expect("create home dir");
        std::fs::create_dir_all(&workspace_dir).expect("create workspace dir");
        let _guard = EnvGuard::new(&home_dir, &workspace_dir, "https://api.example.test");

        let output = load_outdated_dependencies().await.expect("load outdated");

        assert!(matches!(output, OutdatedOutput::MissingLockfile));
    }

    #[tokio::test]
    #[serial]
    async fn loads_outdated_items_from_registry_metadata() {
        let server = TestServer::start().await;
        let temp_dir = TempDir::new().expect("create temp dir");
        let home_dir = temp_dir.path().join("home");
        let workspace_dir = temp_dir.path().join("workspace");
        std::fs::create_dir_all(&home_dir).expect("create home dir");
        std::fs::create_dir_all(&workspace_dir).expect("create workspace dir");
        let _guard = EnvGuard::new(&home_dir, &workspace_dir, &server.base_url);
        Credentials::clear().expect("clear creds");

        std::fs::write(
            workspace_dir.join("rusl.lock"),
            r#"
version = "1"

[dependencies."schema:hassox/root"]
version = "1.0.0"
integrity = "root"
source = "https://api.rusl.app"
"#,
        )
        .expect("write lockfile");

        let output = load_outdated_dependencies().await.expect("load outdated");

        assert_eq!(
            output,
            OutdatedOutput::Items(vec![OutdatedItem {
                package_key: "schema:hassox/root".to_string(),
                display_name: "hassox/root".to_string(),
                current_version: "1.0.0".to_string(),
                latest_version: "1.2.0".to_string(),
            }])
        );
    }

    async fn metadata_handler(
        State(_state): State<TestState>,
        AxumPath((account, slug)): AxumPath<(String, String)>,
    ) -> (StatusCode, Json<Value>) {
        let body = match (account.as_str(), slug.as_str()) {
            ("hassox", "root") => json!({
                "name": "root",
                "versions": [
                    { "version": "1.0.0", "schemas": {}, "bundles": {} },
                    { "version": "1.2.0", "schemas": {}, "bundles": {} }
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

    #[cfg(windows)]
    fn home_var_name() -> &'static str {
        "USERPROFILE"
    }

    #[cfg(not(windows))]
    fn home_var_name() -> &'static str {
        "HOME"
    }
}
