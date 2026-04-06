use crate::manifest::bundle::BundleManifest;
use crate::manifest::lock::LockManifest;
use anyhow::{Context, Result, bail};
use std::collections::HashSet;
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListOutput {
    MissingLockfile,
    EmptyLockfile,
    Flat(ListFlatView),
    Tree(ListTreeView),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListFlatView {
    pub items: Vec<ListFlatItem>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListFlatItem {
    pub display_name: String,
    pub version: String,
    pub source: Option<String>,
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
    pub children: Vec<ListTreeNode>,
}

pub fn load_dependencies(tree: bool) -> Result<ListOutput> {
    let cwd = env::current_dir().context("Failed to get current working directory")?;
    let lock_path = cwd.join("rusl.lock");

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
        build_tree_view(&cwd, &lock_manifest).map(ListOutput::Tree)
    } else {
        Ok(ListOutput::Flat(build_flat_view(&lock_manifest)))
    }
}

fn build_flat_view(lock: &LockManifest) -> ListFlatView {
    let items = lock
        .dependencies
        .iter()
        .map(|(name, dep)| ListFlatItem {
            display_name: display_name(name),
            version: dep.version.clone(),
            source: if is_external_source(&dep.source) {
                Some(dep.source.clone())
            } else {
                None
            },
        })
        .collect();

    ListFlatView { items }
}

fn build_tree_view(cwd: &std::path::Path, lock: &LockManifest) -> Result<ListTreeView> {
    let manifest_path = cwd.join("rusl.bundle.toml");
    if !manifest_path.exists() {
        bail!("No rusl.bundle.toml found. Cannot display dependency tree.");
    }

    let manifest_str =
        fs::read_to_string(&manifest_path).context("Failed to read rusl.bundle.toml")?;
    let manifest: BundleManifest =
        toml::from_str(&manifest_str).context("Failed to parse rusl.bundle.toml")?;

    let mut root_deps = manifest
        .schemas
        .keys()
        .map(|name| format!("schema:{name}"))
        .chain(manifest.bundles.keys().map(|name| format!("bundle:{name}")))
        .collect::<Vec<_>>();
    root_deps.sort();

    let mut seen = HashSet::new();
    let dependencies = root_deps
        .iter()
        .map(|dep_key| build_tree_node(dep_key, lock, &mut seen))
        .collect();

    Ok(ListTreeView {
        root_name: manifest.bundle.name,
        root_version: manifest.bundle.version,
        dependencies,
    })
}

fn build_tree_node(key: &str, lock: &LockManifest, seen: &mut HashSet<String>) -> ListTreeNode {
    let version = lock.dependencies.get(key).map(|dep| dep.version.clone());

    if seen.contains(key) {
        return ListTreeNode {
            display_name: display_name(key),
            version,
            repeated: true,
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
                .map(|child| build_tree_node(child, lock, seen))
                .collect()
        })
        .unwrap_or_default();

    ListTreeNode {
        display_name: display_name(key),
        version,
        repeated: false,
        children,
    }
}

fn display_name(key: &str) -> String {
    if let Some(path) = key.strip_prefix("bundle:") {
        format!("bundles/{path}")
    } else if let Some(path) = key.strip_prefix("schema:") {
        path.to_string()
    } else {
        key.to_string()
    }
}

fn is_external_source(source: &str) -> bool {
    !source.contains("rusl.app") && !source.contains("localhost")
}

#[cfg(test)]
mod tests {
    use super::{
        ListOutput, build_flat_view, build_tree_node, display_name, is_external_source,
        load_dependencies,
    };
    use crate::manifest::lock::{LockDependency, LockManifest};
    use serial_test::serial;
    use std::{
        collections::{BTreeMap, HashSet},
        ffi::OsString,
        path::PathBuf,
    };
    use tempfile::TempDir;

    #[test]
    fn formats_dependency_display_names() {
        assert_eq!(display_name("bundle:acme/common"), "bundles/acme/common");
        assert_eq!(display_name("schema:acme/types"), "acme/types");
    }

    #[test]
    fn marks_external_sources_in_flat_view() {
        let lock = sample_lock();
        let ListOutput::Flat(flat) = ListOutput::Flat(build_flat_view(&lock)) else {
            unreachable!();
        };

        let bundle = flat
            .items
            .iter()
            .find(|item| item.display_name == "bundles/acme/common")
            .expect("bundle item should exist");
        let external = flat
            .items
            .iter()
            .find(|item| item.display_name == "acme/shared")
            .expect("external item should exist");

        assert_eq!(bundle.source, None);
        assert_eq!(
            external.source.as_deref(),
            Some("https://example.com/schema.json")
        );
        assert!(is_external_source("https://example.com/schema.json"));
        assert!(!is_external_source(
            "https://api.rusl.app/schemas/acme/common"
        ));
    }

    #[test]
    fn collapses_repeated_nodes_in_tree_view() {
        let lock = sample_lock();
        let mut seen = HashSet::new();
        let tree = build_tree_node("schema:acme/root", &lock, &mut seen);

        assert_eq!(tree.children.len(), 2);
        assert!(!tree.children[0].repeated);
        assert!(tree.children[1].repeated);
    }

    fn sample_lock() -> LockManifest {
        let mut dependencies = BTreeMap::new();
        dependencies.insert(
            "bundle:acme/common".to_string(),
            LockDependency {
                version: "2.0.0".to_string(),
                integrity: "sha256-bundle".to_string(),
                source: "https://api.rusl.app/bundles/acme/common".to_string(),
                dependencies: vec!["schema:acme/shared".to_string()],
            },
        );
        dependencies.insert(
            "schema:acme/root".to_string(),
            LockDependency {
                version: "1.0.0".to_string(),
                integrity: "sha256-root".to_string(),
                source: "https://api.rusl.app/schemas/acme/root".to_string(),
                dependencies: vec![
                    "bundle:acme/common".to_string(),
                    "schema:acme/shared".to_string(),
                ],
            },
        );
        dependencies.insert(
            "schema:acme/shared".to_string(),
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
    fn load_dependencies_reports_missing_lockfile() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());

        let output = load_dependencies(false).expect("load dependencies");

        assert!(matches!(output, ListOutput::MissingLockfile));
    }

    #[test]
    #[serial]
    fn load_dependencies_builds_tree_view_from_manifest_and_lockfile() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = DirGuard::new(temp_dir.path());
        std::fs::write(
            temp_dir.path().join("rusl.bundle.toml"),
            r#"
[bundle]
name = "hassox/demo"
version = "0.1.0"

[schemas]
"acme/root" = ">=1.0.0"
"#,
        )
        .expect("write manifest");
        std::fs::write(
            temp_dir.path().join("rusl.lock"),
            r#"
version = "1"

[dependencies."schema:acme/root"]
version = "1.0.0"
integrity = "root"
source = "https://api.rusl.app"
dependencies = ["schema:acme/shared"]

[dependencies."schema:acme/shared"]
version = "1.2.0"
integrity = "shared"
source = "https://example.com/schema.json"
"#,
        )
        .expect("write lockfile");

        let output = load_dependencies(true).expect("load dependencies");

        let ListOutput::Tree(tree) = output else {
            panic!("expected tree output");
        };
        assert_eq!(tree.root_name, "hassox/demo");
        assert_eq!(tree.dependencies.len(), 1);
        assert_eq!(tree.dependencies[0].display_name, "acme/root");
        assert_eq!(tree.dependencies[0].children[0].display_name, "acme/shared");
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
