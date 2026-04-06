use crate::install_service::{self, InstallResult};
use crate::registry::client::{RegistryClient, RegistryVersion};
use crate::resolver::graph::ProgressReporter;
use anyhow::{Context, Result, bail};
use pubgrub::SemanticVersion;
use std::{future::Future, pin::Pin};
use toml_edit::{DocumentMut, table, value};

type InstallFuture<'a> = Pin<Box<dyn Future<Output = Result<InstallResult>> + 'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    Schema,
    Bundle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDependencyRequest {
    pub kind: DependencyKind,
    pub slug: String,
    pub version_requirement: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddDependencyResult {
    pub slug: String,
    pub table_key: &'static str,
    pub version_requirement: String,
    pub install: InstallResult,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoveDependencyRequest {
    pub kind: DependencyKind,
    pub slug: String,
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

pub async fn add_dependency<P>(
    request: AddDependencyRequest,
    progress: &P,
) -> Result<AddDependencyResult>
where
    P: ProgressReporter,
{
    add_dependency_with_installer(request, progress, |progress| {
        Box::pin(install_service::install_project(progress))
    })
    .await
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

    let manifest_path = current_manifest_path()?;
    let mut document = read_manifest_document(&manifest_path)?;
    let table_key = table_key(request.kind);
    let version_requirement = match request.version_requirement {
        Some(version) => version,
        None => latest_version_requirement(request.kind, &request.slug, progress).await?,
    };

    if document.get(table_key).is_none() {
        document[table_key] = table();
    }

    let Some(table_ref) = document.get_mut(table_key) else {
        bail!("The [{table_key}] key exists but could not be loaded.");
    };
    let Some(table) = table_ref.as_table_mut() else {
        bail!("The [{table_key}] key exists but is not a TOML table.");
    };

    progress.set_message(format!(
        "Adding {}@{} to [{}]...",
        request.slug, version_requirement, table_key
    ));
    table.insert(&request.slug, value(&version_requirement));

    std::fs::write(&manifest_path, document.to_string())
        .context("Failed to persist rusl.bundle.toml")?;

    let install = install(progress).await?;
    Ok(AddDependencyResult {
        slug: request.slug,
        table_key,
        version_requirement,
        install,
    })
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

    let manifest_path = current_manifest_path()?;
    let mut document = read_manifest_document(&manifest_path)?;
    let table_key = table_key(request.kind);

    let removed = document
        .get_mut(table_key)
        .and_then(|item| item.as_table_mut())
        .and_then(|table| table.remove(&request.slug));

    if removed.is_none() {
        return Ok(RemoveDependencyResult::NotPresent {
            slug: request.slug,
            table_key,
        });
    }

    progress.set_message(format!("Removing {} from [{}]...", request.slug, table_key));
    std::fs::write(&manifest_path, document.to_string())
        .context("Failed to persist rusl.bundle.toml")?;

    let install = install(progress).await?;
    Ok(RemoveDependencyResult::Removed {
        slug: request.slug,
        table_key,
        install,
    })
}

fn current_manifest_path() -> Result<std::path::PathBuf> {
    let cwd = std::env::current_dir().context("Failed to get current working directory")?;
    let manifest_path = cwd.join("rusl.bundle.toml");
    if !manifest_path.exists() {
        bail!("No rusl.bundle.toml found in current directory! Please create one.");
    }
    Ok(manifest_path)
}

fn read_manifest_document(path: &std::path::Path) -> Result<DocumentMut> {
    let contents = std::fs::read_to_string(path).context("Failed to read rusl.bundle.toml")?;
    contents
        .parse::<DocumentMut>()
        .context("Failed to parse rusl.bundle.toml")
}

async fn latest_version_requirement<P>(
    kind: DependencyKind,
    slug: &str,
    progress: &P,
) -> Result<String>
where
    P: ProgressReporter,
{
    progress.set_message("Finding the latest version...".to_string());
    let Some((account, name)) = slug.split_once('/') else {
        bail!("Slug must be in format account/name");
    };

    let config = crate::config::load().context("Failed to load hierarchical configuration")?;
    let client = RegistryClient::new(config);
    let metadata = match kind {
        DependencyKind::Schema => client.fetch_schema_meta(account, name).await?,
        DependencyKind::Bundle => client.fetch_bundle_meta(account, name).await?,
    };

    let latest = latest_version(&metadata.versions)
        .context("The registry did not return any resolvable semantic versions for this slug.")?;
    Ok(format!(">={latest}"))
}

fn latest_version(versions: &[RegistryVersion]) -> Option<SemanticVersion> {
    versions
        .iter()
        .filter_map(|version| version.version.parse::<SemanticVersion>().ok())
        .max()
}

fn table_key(kind: DependencyKind) -> &'static str {
    match kind {
        DependencyKind::Schema => "schemas",
        DependencyKind::Bundle => "bundles",
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AddDependencyRequest, DependencyKind, RemoveDependencyRequest, RemoveDependencyResult,
        add_dependency_with_installer, latest_version, remove_dependency_with_installer, table_key,
    };
    use crate::install_service::InstallResult;
    use crate::registry::client::RegistryVersion;
    use crate::resolver::graph::ProgressReporter;
    use serial_test::serial;
    use std::{collections::HashMap, ffi::OsString, future, path::PathBuf, sync::Mutex};
    use tempfile::TempDir;

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
    fn maps_dependency_kinds_to_manifest_tables() {
        assert_eq!(table_key(DependencyKind::Schema), "schemas");
        assert_eq!(table_key(DependencyKind::Bundle), "bundles");
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
                kind: DependencyKind::Schema,
                slug: "hassox/test-schema".to_string(),
                version_requirement: Some(">=1.2.3".to_string()),
            },
            &progress,
            |_| Box::pin(future::ready(Ok(InstallResult { schema_count: 1 }))),
        )
        .await
        .expect("add dependency");

        assert_eq!(result.table_key, "schemas");
        assert_eq!(result.version_requirement, ">=1.2.3");
        let manifest = std::fs::read_to_string(temp_dir.path().join("rusl.bundle.toml"))
            .expect("read manifest");
        assert!(manifest.contains("[schemas]"));
        assert!(manifest.contains("\"hassox/test-schema\" = \">=1.2.3\""));
    }

    #[tokio::test]
    #[serial]
    async fn remove_dependency_deletes_existing_entry_and_runs_install() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        std::fs::write(
            temp_dir.path().join("rusl.bundle.toml"),
            r#"
[bundle]
name = "hassox/demo"
version = "0.1.0"

[schemas]
"hassox/test-schema" = ">=1.2.3"
"hassox/keep-schema" = ">=2.0.0"
"#,
        )
        .expect("write manifest");
        let progress = TestProgress::default();

        let result = remove_dependency_with_installer(
            RemoveDependencyRequest {
                kind: DependencyKind::Schema,
                slug: "hassox/test-schema".to_string(),
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
                table_key: "schemas",
                ..
            } if slug == "hassox/test-schema"
        ));
        let manifest = std::fs::read_to_string(temp_dir.path().join("rusl.bundle.toml"))
            .expect("read manifest");
        assert!(!manifest.contains("hassox/test-schema"));
        assert!(manifest.contains("hassox/keep-schema"));
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

[schemas]
"hassox/keep-schema" = ">=2.0.0"
"#,
        )
        .expect("write manifest");
        let before = std::fs::read_to_string(&manifest_path).expect("read manifest");
        let progress = TestProgress::default();

        let result = remove_dependency_with_installer(
            RemoveDependencyRequest {
                kind: DependencyKind::Schema,
                slug: "hassox/missing-schema".to_string(),
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
                table_key: "schemas",
            } if slug == "hassox/missing-schema"
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
schemas = "not-a-table"

[bundle]
name = "hassox/demo"
version = "0.1.0"
"#,
        )
        .expect("write manifest");
        let progress = TestProgress::default();

        let error = add_dependency_with_installer(
            AddDependencyRequest {
                kind: DependencyKind::Schema,
                slug: "hassox/test-schema".to_string(),
                version_requirement: Some(">=1.2.3".to_string()),
            },
            &progress,
            |_| Box::pin(future::ready(Ok(InstallResult { schema_count: 1 }))),
        )
        .await
        .expect_err("expected non-table error");

        assert!(
            error
                .to_string()
                .contains("The [schemas] key exists but is not a TOML table")
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
