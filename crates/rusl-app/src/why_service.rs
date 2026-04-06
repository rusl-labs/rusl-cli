use crate::manifest::bundle::BundleManifest;
use crate::manifest::lock::LockManifest;
use anyhow::{Context, Result, bail};
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhyOutput {
    MissingLockfile,
    TargetNotFound { package: String },
    Unreachable { display_name: String },
    Tree(WhyTreeView),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhyTreeView {
    pub target_display_name: String,
    pub target_version: Option<String>,
    pub root_name: String,
    pub root_version: String,
    pub paths: Vec<WhyTreeNode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhyTreeNode {
    pub display_name: String,
    pub version: Option<String>,
    pub is_target: bool,
    pub children: Vec<WhyTreeNode>,
}

pub fn load_dependency_paths(package: &str) -> Result<WhyOutput> {
    let cwd = env::current_dir().context("Failed to get current working directory")?;
    let lock_path = cwd.join("rusl.lock");

    if !lock_path.exists() {
        return Ok(WhyOutput::MissingLockfile);
    }

    let lock_str = fs::read_to_string(&lock_path).context("Failed to read rusl.lock")?;
    let lock: LockManifest = toml::from_str(&lock_str).context("Failed to parse rusl.lock")?;

    let target = match resolve_search_key(package, &lock) {
        Some(key) => key,
        None => {
            return Ok(WhyOutput::TargetNotFound {
                package: package.to_string(),
            });
        }
    };

    let manifest_path = cwd.join("rusl.bundle.toml");
    if !manifest_path.exists() {
        bail!("No rusl.bundle.toml found. Cannot trace dependency paths.");
    }

    let manifest_str = fs::read_to_string(&manifest_path)?;
    let manifest: BundleManifest = toml::from_str(&manifest_str)?;

    let mut root_deps: Vec<String> = manifest
        .schemas
        .keys()
        .map(|name| format!("schema:{name}"))
        .chain(manifest.bundles.keys().map(|name| format!("bundle:{name}")))
        .collect();
    root_deps.sort();

    let ancestors = find_ancestors(&target, &lock);
    let relevant_roots: Vec<&str> = root_deps
        .iter()
        .map(String::as_str)
        .filter(|dep| ancestors.contains(*dep))
        .collect();

    if relevant_roots.is_empty() {
        return Ok(WhyOutput::Unreachable {
            display_name: display_name(&target),
        });
    }

    let paths = relevant_roots
        .into_iter()
        .map(|dep| build_search_node(dep, &target, &lock, &ancestors))
        .collect();

    Ok(WhyOutput::Tree(WhyTreeView {
        target_display_name: display_name(&target),
        target_version: lock
            .dependencies
            .get(&target)
            .map(|dep| dep.version.clone()),
        root_name: manifest.bundle.name,
        root_version: manifest.bundle.version,
        paths,
    }))
}

fn build_search_node(
    key: &str,
    target: &str,
    lock: &LockManifest,
    ancestors: &HashSet<String>,
) -> WhyTreeNode {
    let children = lock
        .dependencies
        .get(key)
        .map(|dep| {
            dep.dependencies
                .iter()
                .filter(|child| ancestors.contains(child.as_str()))
                .map(|child| build_search_node(child, target, lock, ancestors))
                .collect()
        })
        .unwrap_or_default();

    WhyTreeNode {
        display_name: display_name(key),
        version: lock.dependencies.get(key).map(|dep| dep.version.clone()),
        is_target: key == target,
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

fn resolve_search_key(search: &str, lock: &LockManifest) -> Option<String> {
    if lock.dependencies.contains_key(search) {
        return Some(search.to_string());
    }

    let schema_key = format!("schema:{search}");
    if lock.dependencies.contains_key(&schema_key) {
        return Some(schema_key);
    }

    let bundle_key = format!("bundle:{search}");
    if lock.dependencies.contains_key(&bundle_key) {
        return Some(bundle_key);
    }

    lock.dependencies
        .keys()
        .find(|key| display_name(key) == search)
        .cloned()
}

fn find_ancestors(target: &str, lock: &LockManifest) -> HashSet<String> {
    let mut reverse: HashMap<&str, Vec<&str>> = HashMap::new();
    for (key, dep) in &lock.dependencies {
        for child in &dep.dependencies {
            reverse
                .entry(child.as_str())
                .or_default()
                .push(key.as_str());
        }
    }

    let mut ancestors = HashSet::from([target.to_string()]);
    let mut queue = VecDeque::from([target.to_string()]);
    while let Some(node) = queue.pop_front() {
        if let Some(parents) = reverse.get(node.as_str()) {
            for &parent in parents {
                if ancestors.insert(parent.to_string()) {
                    queue.push_back(parent.to_string());
                }
            }
        }
    }

    ancestors
}

#[cfg(test)]
mod tests {
    use super::{WhyOutput, display_name, load_dependency_paths, resolve_search_key};
    use crate::manifest::lock::{LockDependency, LockManifest};
    use serial_test::serial;
    use std::{collections::BTreeMap, ffi::OsString, path::PathBuf};
    use tempfile::TempDir;

    #[test]
    fn display_name_formats_bundle_and_schema_keys() {
        assert_eq!(display_name("bundle:hassox/demo"), "bundles/hassox/demo");
        assert_eq!(display_name("schema:rusl/common"), "rusl/common");
    }

    #[test]
    fn resolve_search_key_matches_display_names() {
        let lock = LockManifest {
            version: "1".to_string(),
            dependencies: BTreeMap::from([(
                "bundle:hassox/demo".to_string(),
                LockDependency {
                    version: "1.0.0".to_string(),
                    integrity: "sha256:demo".to_string(),
                    source: "https://example.test".to_string(),
                    dependencies: Vec::new(),
                },
            )]),
        };

        assert_eq!(
            resolve_search_key("bundles/hassox/demo", &lock),
            Some("bundle:hassox/demo".to_string())
        );
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
    fn load_dependency_paths_returns_tree_for_reachable_target() {
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
source = "https://api.rusl.app"
"#,
        )
        .expect("write lockfile");

        let output = load_dependency_paths("acme/shared").expect("load dependency paths");

        let WhyOutput::Tree(tree) = output else {
            panic!("expected tree output");
        };
        assert_eq!(tree.target_display_name, "acme/shared");
        assert_eq!(tree.paths[0].display_name, "acme/root");
        assert!(tree.paths[0].children[0].is_target);
    }

    #[test]
    #[serial]
    fn load_dependency_paths_reports_unreachable_target() {
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

[dependencies."schema:acme/other"]
version = "1.0.0"
integrity = "other"
source = "https://api.rusl.app"
"#,
        )
        .expect("write lockfile");

        let output = load_dependency_paths("acme/other").expect("load dependency paths");

        assert_eq!(
            output,
            WhyOutput::Unreachable {
                display_name: "acme/other".to_string(),
            }
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
