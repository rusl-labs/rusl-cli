use crate::manifest::bundle::BundleManifest;
use crate::manifest::lock::{LOCAL_SOURCE, LockManifest};
use crate::resource_identifier::{display_package_key, package_key_from_identifier};
use anyhow::{Context, Result, bail};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

const LOCAL_BUNDLE_NAME: &str = "local bundle";
const LOCAL_BUNDLE_VERSION: &str = "unversioned";
const FIRST_PARTY_SOURCE_MARKERS: [&str; 3] =
    ["resources.rusl.com", "resources.rusl.app", "localhost"];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListOutput {
    MissingLockfile,
    EmptyLockfile,
    Flat(ListFlatView),
    Tree(ListTreeView),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListFlatView {
    /// Resolved path of the lockfile that was listed.
    pub lock_path: String,
    pub items: Vec<ListFlatItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListFlatItem {
    pub display_name: String,
    pub version: String,
    pub source: Option<String>,
    /// Marked `dev` in `rusl.bundle.toml`; the local file is kept across installs.
    pub dev: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTreeView {
    pub root_name: String,
    pub root_version: String,
    pub dependencies: Vec<ListTreeNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListTreeNode {
    pub display_name: String,
    pub version: Option<String>,
    pub repeated: bool,
    /// Marked `dev` in `rusl.bundle.toml`; the local file is kept across installs.
    pub dev: bool,
    pub children: Vec<ListTreeNode>,
}

pub fn load_dependencies(tree: bool) -> Result<ListOutput> {
    let project = crate::project::discover_bundle_from_cwd()?;
    let lock_path = project.lock_path();

    if !lock_path.exists() {
        return Ok(ListOutput::MissingLockfile);
    }

    let lock_str = fs::read_to_string(&lock_path).context("Failed to read rusl.lock")?;
    let lock_manifest: LockManifest =
        toml::from_str(&lock_str).context("Failed to parse rusl.lock")?;

    if lock_manifest.dependencies.is_empty() {
        return Ok(ListOutput::EmptyLockfile);
    }

    if tree {
        build_tree_view(&project.root, &lock_manifest).map(ListOutput::Tree)
    } else {
        let lock_display = lock_path.display().to_string();
        let dev_ids = load_manifest(&project.root)?
            .map(|manifest| manifest.protected_resource_ids())
            .unwrap_or_default();
        Ok(ListOutput::Flat(ListFlatView {
            lock_path: lock_display,
            items: build_flat_items(&lock_manifest, &dev_ids),
        }))
    }
}

fn load_manifest(cwd: &Path) -> Result<Option<BundleManifest>> {
    let manifest_path = cwd.join("rusl.bundle.toml");
    if !manifest_path.exists() {
        return Ok(None);
    }

    let manifest_str =
        fs::read_to_string(&manifest_path).context("Failed to read rusl.bundle.toml")?;
    let manifest: BundleManifest =
        toml::from_str(&manifest_str).context("Failed to parse rusl.bundle.toml")?;
    Ok(Some(manifest))
}

fn build_flat_items(lock: &LockManifest, dev_ids: &HashSet<String>) -> Vec<ListFlatItem> {
    lock.dependencies
        .iter()
        .map(|(name, dep)| {
            let display_name = display_package_key(name);
            ListFlatItem {
                dev: dev_ids.contains(&display_name),
                display_name,
                version: dep.version.clone(),
                source: if is_external_source(&dep.source) {
                    Some(dep.source.clone())
                } else {
                    None
                },
            }
        })
        .collect()
}

#[cfg(test)]
fn build_flat_view(lock: &LockManifest, dev_ids: &HashSet<String>) -> ListFlatView {
    ListFlatView {
        lock_path: "rusl.lock".to_string(),
        items: build_flat_items(lock, dev_ids),
    }
}

fn build_tree_view(cwd: &Path, lock: &LockManifest) -> Result<ListTreeView> {
    let Some(manifest) = load_manifest(cwd)? else {
        bail!("No rusl.bundle.toml found. Cannot display dependency tree.");
    };
    let dev_ids = manifest.protected_resource_ids();

    let mut root_deps = manifest
        .rusl
        .resources
        .keys()
        .filter_map(|identifier| package_key_from_identifier(identifier))
        .collect::<Vec<_>>();
    root_deps.sort();
    root_deps.dedup();

    let mut seen = HashSet::new();
    let dependencies = root_deps
        .iter()
        .map(|dep_key| build_tree_node(dep_key, lock, &dev_ids, &mut seen))
        .collect();

    Ok(ListTreeView {
        root_name: manifest
            .bundle
            .name
            .clone()
            .unwrap_or_else(|| LOCAL_BUNDLE_NAME.to_string()),
        root_version: manifest
            .bundle
            .version
            .unwrap_or_else(|| LOCAL_BUNDLE_VERSION.to_string()),
        dependencies,
    })
}

fn build_tree_node(
    key: &str,
    lock: &LockManifest,
    dev_ids: &HashSet<String>,
    seen: &mut HashSet<String>,
) -> ListTreeNode {
    let version = lock.dependencies.get(key).map(|dep| dep.version.clone());
    let display_name = display_package_key(key);
    let dev = dev_ids.contains(&display_name);

    if seen.contains(key) {
        return ListTreeNode {
            display_name,
            version,
            repeated: true,
            dev,
            children: Vec::new(),
        };
    }

    seen.insert(key.to_string());
    let children = lock
        .dependencies
        .get(key)
        .map(|dep| {
            dep.dependencies
                .iter()
                .map(|child| build_tree_node(child, lock, dev_ids, seen))
                .collect()
        })
        .unwrap_or_default();

    ListTreeNode {
        display_name,
        version,
        repeated: false,
        dev,
        children,
    }
}

fn is_external_source(source: &str) -> bool {
    source != LOCAL_SOURCE
        && !FIRST_PARTY_SOURCE_MARKERS
            .iter()
            .any(|marker| source.contains(marker))
}

#[cfg(test)]
mod tests {
    use super::{
        ListOutput, build_flat_view, build_tree_node, is_external_source, load_dependencies,
    };
    use crate::manifest::lock::{LockDependency, LockManifest};
    use crate::resource_identifier::display_package_key;
    use serial_test::serial;
    use std::{
        collections::{BTreeMap, HashSet},
        ffi::OsString,
        path::PathBuf,
    };
    use tempfile::TempDir;

    #[test]
    fn formats_dependency_display_names() {
        assert_eq!(
            display_package_key("bundle:acme/bundles/common"),
            "acme/bundles/common"
        );
        assert_eq!(
            display_package_key("schema:acme/schemas/types"),
            "acme/schemas/types"
        );
    }

    #[test]
    fn marks_external_sources_in_flat_view() {
        let lock = sample_lock();
        let ListOutput::Flat(flat) = ListOutput::Flat(build_flat_view(&lock, &HashSet::new()))
        else {
            unreachable!();
        };

        let bundle = flat
            .items
            .iter()
            .find(|item| item.display_name == "acme/bundles/common")
            .expect("bundle item should exist");
        let external = flat
            .items
            .iter()
            .find(|item| item.display_name == "acme/schemas/shared")
            .expect("external item should exist");

        assert_eq!(bundle.source, None);
        assert_eq!(
            external.source.as_deref(),
            Some("https://example.com/schema.json")
        );
        assert!(is_external_source("https://example.com/schema.json"));
        assert!(!is_external_source(
            "https://resources.rusl.com/resources/acme/schemas/common"
        ));
        assert!(!is_external_source(
            "https://resources.rusl.app/resources/acme/schemas/common"
        ));
        assert!(!is_external_source("local"));
    }

    #[test]
    fn marks_dev_resources_in_flat_view() {
        let lock = sample_lock();
        let dev_ids = HashSet::from(["acme/schemas/shared".to_string()]);
        let flat = build_flat_view(&lock, &dev_ids);

        let shared = flat
            .items
            .iter()
            .find(|item| item.display_name == "acme/schemas/shared")
            .expect("shared item should exist");
        let root = flat
            .items
            .iter()
            .find(|item| item.display_name == "acme/schemas/root")
            .expect("root item should exist");

        assert!(shared.dev);
        assert!(!root.dev);
    }

    #[test]
    fn collapses_repeated_nodes_in_tree_view() {
        let lock = sample_lock();
        let dev_ids = HashSet::from(["acme/schemas/shared".to_string()]);
        let mut seen = HashSet::new();
        let tree = build_tree_node("schema:acme/schemas/root", &lock, &dev_ids, &mut seen);

        assert_eq!(tree.children.len(), 2);
        assert!(!tree.dev);
        assert!(!tree.children[0].repeated);
        assert!(tree.children[1].repeated);
        assert!(tree.children[1].dev);
    }

    fn sample_lock() -> LockManifest {
        let mut dependencies = BTreeMap::new();
        dependencies.insert(
            "bundle:acme/bundles/common".to_string(),
            LockDependency {
                version: "2.0.0".to_string(),
                integrity: "sha256-bundle".to_string(),
                source: "https://resources.rusl.com/resources/acme/bundles/common".to_string(),
                dependencies: vec!["schema:acme/schemas/shared".to_string()],
            },
        );
        dependencies.insert(
            "schema:acme/schemas/root".to_string(),
            LockDependency {
                version: "1.0.0".to_string(),
                integrity: "sha256-root".to_string(),
                source: "https://resources.rusl.com/resources/acme/schemas/root".to_string(),
                dependencies: vec![
                    "bundle:acme/bundles/common".to_string(),
                    "schema:acme/schemas/shared".to_string(),
                ],
            },
        );
        dependencies.insert(
            "schema:acme/schemas/shared".to_string(),
            LockDependency {
                version: "1.2.0".to_string(),
                integrity: "sha256-shared".to_string(),
                source: "https://example.com/schema.json".to_string(),
                dependencies: Vec::new(),
            },
        );

        LockManifest {
            version: "1".to_string(),
            dependencies,
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
    #[serial]
    fn load_dependencies_flat_view_shows_resolved_lock_path_from_walk_up() {
        let temp_dir = TempDir::new().expect("create temp dir");
        write_dev_fixtures(temp_dir.path());
        let nested = temp_dir.path().join("packages").join("schemas");
        std::fs::create_dir_all(&nested).expect("mkdir");
        let _guard = DirGuard::new(&nested);

        let output = load_dependencies(false).expect("load dependencies");
        let ListOutput::Flat(flat) = output else {
            panic!("expected flat output");
        };
        assert_eq!(
            flat.lock_path,
            temp_dir.path().join("rusl.lock").display().to_string()
        );
    }

    #[test]
    #[serial]
    fn load_dependencies_reports_missing_lockfile() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        std::fs::write(
            temp_dir.path().join("rusl.bundle.toml"),
            "[rusl.resources]\n",
        )
        .expect("write bundle");

        let output = load_dependencies(false).expect("load dependencies");

        assert!(matches!(output, ListOutput::MissingLockfile));
    }

    #[test]
    #[serial]
    fn load_dependencies_builds_tree_view_from_manifest_and_lockfile() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        write_dev_fixtures(temp_dir.path());

        let output = load_dependencies(true).expect("load dependencies");

        let ListOutput::Tree(tree) = output else {
            panic!("expected tree output");
        };
        assert_eq!(tree.root_name, "local bundle");
        assert_eq!(tree.root_version, "unversioned");
        assert_eq!(tree.dependencies.len(), 2);
        assert_eq!(tree.dependencies[0].display_name, "acme/schemas/draft");
        assert!(tree.dependencies[0].dev);
        assert_eq!(tree.dependencies[1].display_name, "acme/schemas/root");
        assert!(!tree.dependencies[1].dev);
        assert_eq!(
            tree.dependencies[1].children[0].display_name,
            "acme/schemas/shared"
        );
    }

    #[test]
    #[serial]
    fn load_dependencies_marks_dev_and_hides_local_source_in_flat_view() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        write_dev_fixtures(temp_dir.path());

        let output = load_dependencies(false).expect("load dependencies");

        let ListOutput::Flat(flat) = output else {
            panic!("expected flat output");
        };
        let draft = flat
            .items
            .iter()
            .find(|item| item.display_name == "acme/schemas/draft")
            .expect("draft item should exist");
        assert!(draft.dev);
        assert_eq!(draft.version, "0.0.0");
        assert_eq!(draft.source, None);
    }

    fn write_dev_fixtures(dir: &std::path::Path) {
        std::fs::write(
            dir.join("rusl.bundle.toml"),
            r#"
[rusl.resources]
"acme/schemas/root" = ">=1.0.0"
"acme/schemas/draft" = { dev = true }
"#,
        )
        .expect("write manifest");
        std::fs::write(
            dir.join("rusl.lock"),
            r#"
version = "1"

[dependencies."schema:acme/schemas/root"]
version = "1.0.0"
integrity = "root"
source = "https://resources.rusl.com"
dependencies = ["schema:acme/schemas/shared"]

[dependencies."schema:acme/schemas/shared"]
version = "1.2.0"
integrity = "shared"
source = "https://example.com/schema.json"

[dependencies."schema:acme/schemas/draft"]
version = "0.0.0"
integrity = ""
source = "local"
"#,
        )
        .expect("write lockfile");
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
