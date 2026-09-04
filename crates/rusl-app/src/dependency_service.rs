use crate::cache::linker::Linker;
use crate::config::Config;
use crate::install_service::{self, InstallResult};
use crate::registry::client::{RegistryClient, RegistryVersion, is_not_found};
use crate::resolver::graph::ProgressReporter;
use crate::resource_identifier::{RegistryResource, ResourceKind};
use anyhow::{Context, Result, bail};
use pubgrub::SemanticVersion;
use std::path::{Path, PathBuf};
use std::{future::Future, pin::Pin};
use toml_edit::{DocumentMut, InlineTable, Item, Table, table, value};

type InstallFuture<'a> = Pin<Box<dyn Future<Output = Result<InstallResult>> + 'a>>;

const UNCONSTRAINED_VERSION: &str = "*";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDependencyRequest {
    pub identifier: String,
    pub version_requirement: Option<String>,
    /// Write the entry as `{ version = "...", dev = true }` so the local file is kept.
    pub dev: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDependencyResult {
    pub slug: String,
    pub table_key: &'static str,
    pub version_requirement: String,
    pub dev: bool,
    /// Starter schema written by `add --dev` for an unpublished schema with no local
    /// file, relative to the project root.
    pub created_schema: Option<PathBuf>,
    pub install: InstallResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveDependencyRequest {
    pub identifier: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RemoveDependencyResult {
    Removed {
        slug: String,
        table_key: &'static str,
        install: InstallResult,
    },
    NotPresent {
        slug: String,
        table_key: &'static str,
    },
}

const RESOURCES_TABLE_KEY: &str = "rusl.resources";

pub async fn add_dependency<P>(
    request: AddDependencyRequest,
    progress: &P,
) -> Result<AddDependencyResult>
where
    P: ProgressReporter,
{
    let created_schema = if request.dev {
        let resource = parse_resource(&request.identifier)?;
        let config = crate::config::load().context("Failed to load hierarchical configuration")?;
        let cwd = std::env::current_dir().context("Failed to get current working directory")?;
        seed_dev_schema(&resource, &config, &cwd, progress).await?
    } else {
        None
    };

    let mut result = add_dependency_with_installer(request, progress, |progress| {
        Box::pin(install_service::install_project(progress))
    })
    .await?;
    result.created_schema = created_schema;
    Ok(result)
}

/// For `add --dev`: when the schema is not published and has no local file, write a
/// starter JSON Schema at its install path so the first install resolves it locally.
///
/// Only a definitive 404 or an empty version list counts as "not published"; any other
/// registry failure is returned so an outage never scaffolds over a real schema.
async fn seed_dev_schema<P>(
    resource: &RegistryResource,
    config: &Config,
    cwd: &Path,
    progress: &P,
) -> Result<Option<PathBuf>>
where
    P: ProgressReporter,
{
    if resource.kind != ResourceKind::Schema {
        return Ok(None);
    }

    let linker = Linker::for_project(cwd.to_path_buf(), config);
    let path = linker.installed_path(resource);
    if path.exists() {
        return Ok(None);
    }

    progress.set_message(format!(
        "Checking whether {} is published...",
        resource.identifier()
    ));
    let client = RegistryClient::new(config.clone());
    let published = match client
        .fetch_schema_meta(&resource.account, &resource.slug)
        .await
    {
        Ok(metadata) => !metadata.versions.is_empty(),
        Err(error) if is_not_found(&error) => false,
        Err(error) => return Err(error),
    };
    if published {
        return Ok(None);
    }

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }
    let contents = serde_json::to_string_pretty(&dev_schema_stub(resource, config))
        .context("Failed to render starter schema")?;
    std::fs::write(&path, format!("{contents}\n"))
        .with_context(|| format!("Failed to write starter schema at {}", path.display()))?;

    let relative = match path.strip_prefix(cwd) {
        Ok(relative) => relative.to_path_buf(),
        Err(_) => path,
    };
    Ok(Some(relative))
}

/// Minimal draft 2020-12 document that follows the repository schema standard:
/// one concept, explicit `required`, and undeclared properties rejected.
fn dev_schema_stub(resource: &RegistryResource, config: &Config) -> serde_json::Value {
    serde_json::json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": format!(
            "{}/schemas/{}/{}",
            config.website_url.trim_end_matches('/'),
            resource.account,
            resource.slug
        ),
        "title": resource.slug,
        "description": format!("TODO: describe {}.", resource.identifier()),
        "type": "object",
        "properties": {},
        "required": [],
        "additionalProperties": false
    })
}

async fn add_dependency_with_installer<P, I>(
    request: AddDependencyRequest,
    progress: &P,
    install: I,
) -> Result<AddDependencyResult>
where
    P: ProgressReporter,
    I: for<'a> Fn(&'a P) -> InstallFuture<'a>,
{
    progress.set_message("Reading rusl.bundle.toml...".to_string());

    let resource = parse_resource(&request.identifier)?;
    let manifest_path = current_manifest_path()?;
    let mut document = read_manifest_document(&manifest_path)?;
    let table_key = RESOURCES_TABLE_KEY;
    let slug = resource.identifier();
    // A dev schema may not be published yet, so the registry is not consulted for
    // a default version requirement.
    let version_requirement = match (request.version_requirement, request.dev) {
        (Some(version), _) => version,
        (None, true) => UNCONSTRAINED_VERSION.to_string(),
        (None, false) => latest_version_requirement(&resource, progress).await?,
    };

    progress.set_message(format!(
        "Adding {}@{} to [{}]...",
        slug, version_requirement, table_key
    ));
    let table = resources_table_mut(&mut document)?;
    table.insert(
        &slug,
        resource_requirement_item(&version_requirement, request.dev),
    );

    std::fs::write(&manifest_path, document.to_string())
        .context("Failed to persist rusl.bundle.toml")?;

    let install = install(progress).await?;
    Ok(AddDependencyResult {
        slug,
        table_key,
        version_requirement,
        dev: request.dev,
        created_schema: None,
        install,
    })
}

fn resource_requirement_item(version_requirement: &str, dev: bool) -> Item {
    if !dev {
        return value(version_requirement);
    }

    let mut inline = InlineTable::new();
    inline.insert("version", version_requirement.into());
    inline.insert("dev", true.into());
    value(inline)
}

pub async fn remove_dependency<P>(
    request: RemoveDependencyRequest,
    progress: &P,
) -> Result<RemoveDependencyResult>
where
    P: ProgressReporter,
{
    remove_dependency_with_installer(request, progress, |progress| {
        Box::pin(install_service::install_project(progress))
    })
    .await
}

async fn remove_dependency_with_installer<P, I>(
    request: RemoveDependencyRequest,
    progress: &P,
    install: I,
) -> Result<RemoveDependencyResult>
where
    P: ProgressReporter,
    I: for<'a> Fn(&'a P) -> InstallFuture<'a>,
{
    progress.set_message("Reading rusl.bundle.toml...".to_string());

    let resource = parse_resource(&request.identifier)?;
    let manifest_path = current_manifest_path()?;
    let mut document = read_manifest_document(&manifest_path)?;
    let canonical_slug = resource.identifier();

    let removed = match existing_resources_table_mut(&mut document)? {
        Some(table) => table.remove(&canonical_slug).is_some(),
        None => false,
    };

    if !removed {
        return Ok(RemoveDependencyResult::NotPresent {
            slug: canonical_slug,
            table_key: RESOURCES_TABLE_KEY,
        });
    }

    progress.set_message(format!(
        "Removing {} from [{}]...",
        canonical_slug, RESOURCES_TABLE_KEY
    ));
    std::fs::write(&manifest_path, document.to_string())
        .context("Failed to persist rusl.bundle.toml")?;

    let install = install(progress).await?;
    Ok(RemoveDependencyResult::Removed {
        slug: canonical_slug,
        table_key: RESOURCES_TABLE_KEY,
        install,
    })
}

fn current_manifest_path() -> Result<std::path::PathBuf> {
    let cwd = std::env::current_dir().context("Failed to get current working directory")?;
    Ok(cwd.join("rusl.bundle.toml"))
}

fn read_manifest_document(path: &std::path::Path) -> Result<DocumentMut> {
    if !path.exists() {
        return Ok(DocumentMut::new());
    }

    let contents = std::fs::read_to_string(path).context("Failed to read rusl.bundle.toml")?;
    contents
        .parse::<DocumentMut>()
        .context("Failed to parse rusl.bundle.toml")
}

fn resources_table_mut(document: &mut DocumentMut) -> Result<&mut Table> {
    if document.get("rusl").is_none() {
        document["rusl"] = table();
    }

    let Some(rusl) = document.get_mut("rusl") else {
        bail!("The [rusl] key exists but could not be loaded.");
    };
    let Some(rusl_table) = rusl.as_table_mut() else {
        bail!("The [rusl] key exists but is not a TOML table.");
    };

    if rusl_table.get("resources").is_none() {
        rusl_table["resources"] = table();
    }

    let Some(resources) = rusl_table.get_mut("resources") else {
        bail!("The [rusl.resources] key exists but could not be loaded.");
    };
    let Some(resources_table) = resources.as_table_mut() else {
        bail!("The [rusl.resources] key exists but is not a TOML table.");
    };

    Ok(resources_table)
}

fn existing_resources_table_mut(document: &mut DocumentMut) -> Result<Option<&mut Table>> {
    let Some(rusl) = document.get_mut("rusl") else {
        return Ok(None);
    };
    let Some(rusl_table) = rusl.as_table_mut() else {
        bail!("The [rusl] key exists but is not a TOML table.");
    };
    let Some(resources) = rusl_table.get_mut("resources") else {
        return Ok(None);
    };
    let Some(resources_table) = resources.as_table_mut() else {
        bail!("The [rusl.resources] key exists but is not a TOML table.");
    };

    Ok(Some(resources_table))
}

async fn latest_version_requirement<P>(resource: &RegistryResource, progress: &P) -> Result<String>
where
    P: ProgressReporter,
{
    progress.set_message("Finding the latest version...".to_string());

    let config = crate::config::load().context("Failed to load hierarchical configuration")?;
    let client = RegistryClient::new(config);
    let metadata = match resource.kind {
        ResourceKind::Schema => {
            client
                .fetch_schema_meta(&resource.account, &resource.slug)
                .await?
        }
        ResourceKind::Bundle => {
            client
                .fetch_bundle_meta(&resource.account, &resource.slug)
                .await?
        }
    };

    let latest = latest_version(&metadata.versions)
        .context("The registry did not return any resolvable semantic versions for this slug.")?;
    Ok(format!(">={latest}"))
}

/// Parse a user-supplied identifier, inferring schema/bundle kind from the string itself.
fn parse_resource(identifier: &str) -> Result<RegistryResource> {
    RegistryResource::from_identifier(identifier).with_context(|| {
        format!(
            "'{}' is not a valid resource identifier. Use account/schemas/name or account/bundles/name.",
            identifier.trim()
        )
    })
}

fn latest_version(versions: &[RegistryVersion]) -> Option<SemanticVersion> {
    versions
        .iter()
        .filter_map(|version| version.version.parse::<SemanticVersion>().ok())
        .max()
}

#[cfg(test)]
mod tests {
    use super::{
        AddDependencyRequest, RESOURCES_TABLE_KEY, RemoveDependencyRequest, RemoveDependencyResult,
        add_dependency_with_installer, latest_version, remove_dependency_with_installer,
        seed_dev_schema,
    };
    use crate::config::Config;
    use crate::install_service::InstallResult;
    use crate::registry::client::RegistryVersion;
    use crate::resolver::graph::ProgressReporter;
    use crate::resource_identifier::RegistryResource;
    use axum::{Json, Router, extract::Path as AxumPath, http::StatusCode, routing::get};
    use serde_json::{Value, json};
    use serial_test::serial;
    use std::{collections::HashMap, ffi::OsString, future, path::PathBuf, sync::Mutex};
    use tempfile::TempDir;
    use tokio::{net::TcpListener, sync::oneshot, task::JoinHandle};

    struct MetadataServer {
        base_url: String,
        shutdown: Option<oneshot::Sender<()>>,
        task: JoinHandle<()>,
    }

    impl MetadataServer {
        /// Serves `hassox/schemas/root` as published, `hassox/schemas/empty` with no
        /// versions, and 404 for everything else.
        async fn start() -> Self {
            let app = Router::new().route(
                "/resources/{account}/schemas/{slug}/metadata",
                get(metadata_handler),
            );
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

        fn config(&self) -> Config {
            Config {
                api_base_url: self.base_url.clone(),
                website_url: "https://rusl.example".to_string(),
                ..Config::default()
            }
        }
    }

    impl Drop for MetadataServer {
        fn drop(&mut self) {
            if let Some(shutdown) = self.shutdown.take() {
                let _ = shutdown.send(());
            }
            self.task.abort();
        }
    }

    async fn metadata_handler(
        AxumPath((account, slug)): AxumPath<(String, String)>,
    ) -> (StatusCode, Json<Value>) {
        match (account.as_str(), slug.as_str()) {
            ("hassox", "root") => (
                StatusCode::OK,
                Json(json!({
                    "name": "root",
                    "versions": [{ "version": "1.0.0", "schemas": {}, "bundles": {} }]
                })),
            ),
            ("hassox", "empty") => (
                StatusCode::OK,
                Json(json!({ "name": "empty", "versions": [] })),
            ),
            _ => (StatusCode::NOT_FOUND, Json(json!({ "error": "not found" }))),
        }
    }

    #[derive(Default)]
    struct TestProgress {
        messages: Mutex<Vec<String>>,
    }

    impl ProgressReporter for TestProgress {
        fn set_message(&self, message: String) {
            self.messages.lock().expect("lock messages").push(message);
        }

        fn println(&self, message: String) {
            self.messages.lock().expect("lock messages").push(message);
        }
    }

    struct DirGuard {
        previous_dir: PathBuf,
        previous_home: Option<OsString>,
    }

    impl DirGuard {
        fn new(dir: &std::path::Path) -> Self {
            let previous_dir = std::env::current_dir().expect("current dir");
            let previous_home = std::env::var_os(home_var_name());
            std::env::set_current_dir(dir).expect("set current dir");
            unsafe { std::env::set_var(home_var_name(), dir.as_os_str()) };
            Self {
                previous_dir,
                previous_home,
            }
        }
    }

    impl Drop for DirGuard {
        fn drop(&mut self) {
            std::env::set_current_dir(&self.previous_dir).expect("restore current dir");
            match self.previous_home.as_ref() {
                Some(value) => unsafe { std::env::set_var(home_var_name(), value) },
                None => unsafe { std::env::remove_var(home_var_name()) },
            }
        }
    }

    #[test]
    fn picks_highest_semantic_version() {
        let versions = vec![
            sample_version("1.0.0"),
            sample_version("2.1.0"),
            sample_version("not-a-version"),
            sample_version("2.0.1"),
        ];

        let latest = latest_version(&versions).expect("latest version should exist");
        assert_eq!(latest.to_string(), "2.1.0");
    }

    fn sample_version(version: &str) -> RegistryVersion {
        RegistryVersion {
            version: version.to_string(),
            schemas: HashMap::new(),
            bundles: HashMap::new(),
        }
    }

    #[tokio::test]
    #[serial]
    async fn add_dependency_creates_missing_table_and_persists_manifest() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        std::fs::write(
            temp_dir.path().join("rusl.bundle.toml"),
            r#"
[bundle]
name = "hassox/demo"
version = "0.1.0"
"#,
        )
        .expect("write manifest");
        let progress = TestProgress::default();

        let result = add_dependency_with_installer(
            AddDependencyRequest {
                identifier: "hassox/schemas/test-schema".to_string(),
                version_requirement: Some(">=1.2.3".to_string()),
                dev: false,
            },
            &progress,
            |_| Box::pin(future::ready(Ok(InstallResult { schema_count: 1 }))),
        )
        .await
        .expect("add dependency");

        assert_eq!(result.table_key, RESOURCES_TABLE_KEY);
        assert_eq!(result.version_requirement, ">=1.2.3");
        assert_eq!(result.slug, "hassox/schemas/test-schema");
        let manifest = std::fs::read_to_string(temp_dir.path().join("rusl.bundle.toml"))
            .expect("read manifest");
        assert!(manifest.contains("[rusl.resources]"));
        assert!(manifest.contains("\"hassox/schemas/test-schema\" = \">=1.2.3\""));
    }

    #[tokio::test]
    #[serial]
    async fn add_dependency_creates_missing_manifest() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        let progress = TestProgress::default();

        let result = add_dependency_with_installer(
            AddDependencyRequest {
                identifier: "hassox/schemas/test-schema".to_string(),
                version_requirement: Some(">=1.2.3".to_string()),
                dev: false,
            },
            &progress,
            |_| Box::pin(future::ready(Ok(InstallResult { schema_count: 1 }))),
        )
        .await
        .expect("add dependency");

        assert_eq!(result.table_key, RESOURCES_TABLE_KEY);
        assert_eq!(result.slug, "hassox/schemas/test-schema");
        let manifest = std::fs::read_to_string(temp_dir.path().join("rusl.bundle.toml"))
            .expect("read manifest");
        assert!(manifest.contains("[rusl.resources]"));
        assert!(manifest.contains("\"hassox/schemas/test-schema\" = \">=1.2.3\""));
    }

    #[tokio::test]
    #[serial]
    async fn add_bundle_dependency_infers_kind_from_canonical_identifier() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        std::fs::write(
            temp_dir.path().join("rusl.bundle.toml"),
            r#"
[bundle]
name = "hassox/bundles/demo"
version = "0.1.0"
"#,
        )
        .expect("write manifest");
        let progress = TestProgress::default();

        let result = add_dependency_with_installer(
            AddDependencyRequest {
                identifier: "hassox/bundles/common".to_string(),
                version_requirement: Some(">=1.2.3".to_string()),
                dev: false,
            },
            &progress,
            |_| Box::pin(future::ready(Ok(InstallResult { schema_count: 1 }))),
        )
        .await
        .expect("add dependency");

        assert_eq!(result.slug, "hassox/bundles/common");
        assert_eq!(result.table_key, RESOURCES_TABLE_KEY);
        let manifest = std::fs::read_to_string(temp_dir.path().join("rusl.bundle.toml"))
            .expect("read manifest");
        assert!(manifest.contains("[rusl.resources]"));
        assert!(manifest.contains("\"hassox/bundles/common\" = \">=1.2.3\""));
    }

    #[tokio::test]
    #[serial]
    async fn add_dev_dependency_writes_inline_table_and_defaults_version_without_registry() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        let progress = TestProgress::default();

        let result = add_dependency_with_installer(
            AddDependencyRequest {
                identifier: "hassox/schemas/draft".to_string(),
                version_requirement: None,
                dev: true,
            },
            &progress,
            |_| Box::pin(future::ready(Ok(InstallResult { schema_count: 1 }))),
        )
        .await
        .expect("add dev dependency");

        assert!(result.dev);
        assert_eq!(result.version_requirement, "*");
        let manifest = std::fs::read_to_string(temp_dir.path().join("rusl.bundle.toml"))
            .expect("read manifest");
        assert!(manifest.contains(r#""hassox/schemas/draft" = { version = "*", dev = true }"#));
        assert!(
            !progress
                .messages
                .lock()
                .expect("lock messages")
                .iter()
                .any(|message| message.contains("Finding the latest version"))
        );
    }

    #[tokio::test]
    #[serial]
    async fn add_dev_dependency_keeps_explicit_version_and_upgrades_string_entry() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        std::fs::write(
            temp_dir.path().join("rusl.bundle.toml"),
            r#"
[rusl.resources]
"hassox/schemas/draft" = ">=1.0.0"
"hassox/schemas/other" = "*"
"#,
        )
        .expect("write manifest");
        let progress = TestProgress::default();

        add_dependency_with_installer(
            AddDependencyRequest {
                identifier: "hassox/schemas/draft".to_string(),
                version_requirement: Some(">=2.0.0".to_string()),
                dev: true,
            },
            &progress,
            |_| Box::pin(future::ready(Ok(InstallResult { schema_count: 1 }))),
        )
        .await
        .expect("add dev dependency");

        let manifest = std::fs::read_to_string(temp_dir.path().join("rusl.bundle.toml"))
            .expect("read manifest");
        assert!(
            manifest.contains(r#""hassox/schemas/draft" = { version = ">=2.0.0", dev = true }"#)
        );
        assert!(!manifest.contains(r#""hassox/schemas/draft" = ">=1.0.0""#));
        assert!(manifest.contains(r#""hassox/schemas/other" = "*""#));

        let parsed: crate::manifest::bundle::BundleManifest =
            toml::from_str(&manifest).expect("manifest round-trips");
        assert!(parsed.rusl.resources["hassox/schemas/draft"].is_dev());
    }

    #[tokio::test]
    #[serial]
    async fn seed_dev_schema_writes_starter_file_for_unpublished_schema() {
        let server = MetadataServer::start().await;
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        let resource = RegistryResource::schema("hassox/schemas/draft").expect("schema");

        let created = seed_dev_schema(
            &resource,
            &server.config(),
            temp_dir.path(),
            &TestProgress::default(),
        )
        .await
        .expect("seed dev schema");

        let expected = PathBuf::from("schemas")
            .join("hassox")
            .join("draft.schema.json");
        assert_eq!(created, Some(expected.clone()));
        let written: Value = serde_json::from_str(
            &std::fs::read_to_string(temp_dir.path().join(&expected)).expect("read stub"),
        )
        .expect("stub is valid JSON");
        assert_eq!(
            written["$schema"],
            "https://json-schema.org/draft/2020-12/schema"
        );
        assert_eq!(written["$id"], "https://rusl.example/schemas/hassox/draft");
        assert_eq!(written["title"], "draft");
        assert_eq!(written["type"], "object");
        assert_eq!(written["additionalProperties"], false);
        assert_eq!(written["required"], json!([]));
    }

    #[tokio::test]
    #[serial]
    async fn seed_dev_schema_treats_empty_version_list_as_unpublished() {
        let server = MetadataServer::start().await;
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        let resource = RegistryResource::schema("hassox/schemas/empty").expect("schema");

        let created = seed_dev_schema(
            &resource,
            &server.config(),
            temp_dir.path(),
            &TestProgress::default(),
        )
        .await
        .expect("seed dev schema");

        assert!(created.is_some());
    }

    #[tokio::test]
    #[serial]
    async fn seed_dev_schema_leaves_existing_file_alone_without_calling_registry() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        let schema_dir = temp_dir.path().join("schemas").join("hassox");
        std::fs::create_dir_all(&schema_dir).expect("create schema dir");
        let existing = schema_dir.join("draft.schema.json");
        std::fs::write(&existing, r#"{"title":"mine"}"#).expect("write existing");
        let resource = RegistryResource::schema("hassox/schemas/draft").expect("schema");
        let unreachable = Config {
            api_base_url: "http://127.0.0.1:9".to_string(),
            website_url: "https://rusl.example".to_string(),
            ..Config::default()
        };

        let created = seed_dev_schema(
            &resource,
            &unreachable,
            temp_dir.path(),
            &TestProgress::default(),
        )
        .await
        .expect("seed dev schema");

        assert_eq!(created, None);
        assert_eq!(
            std::fs::read_to_string(&existing).expect("read existing"),
            r#"{"title":"mine"}"#
        );
    }

    #[tokio::test]
    #[serial]
    async fn seed_dev_schema_skips_published_schema() {
        let server = MetadataServer::start().await;
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        let resource = RegistryResource::schema("hassox/schemas/root").expect("schema");

        let created = seed_dev_schema(
            &resource,
            &server.config(),
            temp_dir.path(),
            &TestProgress::default(),
        )
        .await
        .expect("seed dev schema");

        assert_eq!(created, None);
        assert!(!temp_dir.path().join("schemas").exists());
    }

    #[tokio::test]
    #[serial]
    async fn seed_dev_schema_propagates_registry_failures() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        let resource = RegistryResource::schema("hassox/schemas/draft").expect("schema");
        let unreachable = Config {
            api_base_url: "http://127.0.0.1:9".to_string(),
            website_url: "https://rusl.example".to_string(),
            ..Config::default()
        };

        let error = seed_dev_schema(
            &resource,
            &unreachable,
            temp_dir.path(),
            &TestProgress::default(),
        )
        .await
        .expect_err("expected transport failure");

        assert!(
            error
                .to_string()
                .contains("Failed to fetch schema metadata")
        );
        assert!(!temp_dir.path().join("schemas").exists());
    }

    #[tokio::test]
    #[serial]
    async fn seed_dev_schema_ignores_bundles() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        let resource = RegistryResource::bundle("hassox/bundles/common").expect("bundle");
        let unreachable = Config {
            api_base_url: "http://127.0.0.1:9".to_string(),
            website_url: "https://rusl.example".to_string(),
            ..Config::default()
        };

        let created = seed_dev_schema(
            &resource,
            &unreachable,
            temp_dir.path(),
            &TestProgress::default(),
        )
        .await
        .expect("seed dev schema");

        assert_eq!(created, None);
    }

    #[tokio::test]
    #[serial]
    async fn remove_dependency_deletes_existing_resource_entry_and_runs_install() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        std::fs::write(
            temp_dir.path().join("rusl.bundle.toml"),
            r#"
[bundle]
name = "hassox/demo"
version = "0.1.0"

[rusl.resources]
"hassox/schemas/test-schema" = ">=1.2.3"
"hassox/schemas/keep-schema" = ">=2.0.0"
"#,
        )
        .expect("write manifest");
        let progress = TestProgress::default();

        let result = remove_dependency_with_installer(
            RemoveDependencyRequest {
                identifier: "hassox/schemas/test-schema".to_string(),
            },
            &progress,
            |_| Box::pin(future::ready(Ok(InstallResult { schema_count: 1 }))),
        )
        .await
        .expect("remove dependency");

        assert!(matches!(
            result,
            RemoveDependencyResult::Removed {
                slug,
                table_key: RESOURCES_TABLE_KEY,
                ..
            } if slug == "hassox/schemas/test-schema"
        ));
        let manifest = std::fs::read_to_string(temp_dir.path().join("rusl.bundle.toml"))
            .expect("read manifest");
        assert!(!manifest.contains("hassox/schemas/test-schema"));
        assert!(manifest.contains("hassox/schemas/keep-schema"));
    }

    #[tokio::test]
    #[serial]
    async fn remove_dependency_returns_not_present_without_rewriting_manifest() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        let manifest_path = temp_dir.path().join("rusl.bundle.toml");
        std::fs::write(
            &manifest_path,
            r#"
[bundle]
name = "hassox/demo"
version = "0.1.0"

[rusl.resources]
"hassox/schemas/keep-schema" = ">=2.0.0"
"#,
        )
        .expect("write manifest");
        let before = std::fs::read_to_string(&manifest_path).expect("read manifest");
        let progress = TestProgress::default();

        let result = remove_dependency_with_installer(
            RemoveDependencyRequest {
                identifier: "hassox/schemas/missing-schema".to_string(),
            },
            &progress,
            |_| Box::pin(future::ready(Ok(InstallResult { schema_count: 1 }))),
        )
        .await
        .expect("remove dependency");

        assert!(matches!(
            result,
            RemoveDependencyResult::NotPresent {
                slug,
                table_key: RESOURCES_TABLE_KEY,
            } if slug == "hassox/schemas/missing-schema"
        ));
        let after = std::fs::read_to_string(&manifest_path).expect("read manifest");
        assert_eq!(before, after);
    }

    #[tokio::test]
    #[serial]
    async fn add_dependency_rejects_non_table_dependency_section() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        std::fs::write(
            temp_dir.path().join("rusl.bundle.toml"),
            r#"
[rusl]
resources = "not-a-table"

[bundle]
name = "hassox/demo"
version = "0.1.0"
"#,
        )
        .expect("write manifest");
        let progress = TestProgress::default();

        let error = add_dependency_with_installer(
            AddDependencyRequest {
                identifier: "hassox/schemas/test-schema".to_string(),
                version_requirement: Some(">=1.2.3".to_string()),
                dev: false,
            },
            &progress,
            |_| Box::pin(future::ready(Ok(InstallResult { schema_count: 1 }))),
        )
        .await
        .expect_err("expected non-table error");

        assert!(
            error
                .to_string()
                .contains("The [rusl.resources] key exists but is not a TOML table")
        );
    }

    #[tokio::test]
    #[serial]
    async fn add_dependency_rejects_invalid_identifier() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        let progress = TestProgress::default();

        let error = add_dependency_with_installer(
            AddDependencyRequest {
                identifier: "not-an-identifier".to_string(),
                version_requirement: Some(">=1.2.3".to_string()),
                dev: false,
            },
            &progress,
            |_| Box::pin(future::ready(Ok(InstallResult { schema_count: 1 }))),
        )
        .await
        .expect_err("expected invalid identifier error");

        assert!(
            error
                .to_string()
                .contains("not a valid resource identifier")
        );
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
