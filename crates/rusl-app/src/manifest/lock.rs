use crate::resource_identifier::{RegistryResource, ResourceKind, parse_package_key};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use std::path::Path;

/// The auto-generated lockfile ensuring deterministic builds (rusl.lock).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockManifest {
    /// Schema version for the lockfile format (currently "1")
    pub version: String,

    /// The exact, deeply-resolved graph of dependencies
    pub dependencies: BTreeMap<String, LockDependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockDependency {
    /// The exact semantic version solved by pubgrub
    pub version: String,

    /// The SHA-256 hash of the content-addressable artifact
    pub integrity: String,

    /// The registry URL from which this dependency was originally downloaded
    pub source: String,

    /// The direct dependencies of this package (e.g., ["schema:acme/schemas/types", "bundle:acme/bundles/common"])
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<String>,
}

impl Default for LockManifest {
    fn default() -> Self {
        Self {
            version: "1".to_string(),
            dependencies: BTreeMap::new(),
        }
    }
}

impl LockManifest {
    /// Loads `rusl.lock` from `dir`, or an empty lockfile when none exists.
    pub fn load_from_dir(dir: &Path) -> Result<Self> {
        let path = dir.join("rusl.lock");
        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        toml::from_str(&contents).with_context(|| format!("Failed to parse {}", path.display()))
    }

    /// Previously downloaded schemas that are safe to delete on the next install or cache clear.
    pub fn removable_schemas(&self, protected_ids: &HashSet<String>) -> Vec<RegistryResource> {
        self.dependencies
            .keys()
            .filter_map(|package_key| parse_package_key(package_key))
            .filter(|resource| resource.kind == ResourceKind::Schema)
            .filter(|resource| !protected_ids.contains(&resource.identifier()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{LockDependency, LockManifest};
    use std::collections::{BTreeMap, HashSet};

    fn sample_lock() -> LockManifest {
        let mut dependencies = BTreeMap::new();
        dependencies.insert(
            "schema:acme/schemas/thing".to_string(),
            LockDependency {
                version: "1.0.0".to_string(),
                integrity: "abc".to_string(),
                source: "https://example.test".to_string(),
                dependencies: Vec::new(),
            },
        );
        dependencies.insert(
            "schema:acme/schemas/experiment".to_string(),
            LockDependency {
                version: "1.0.0".to_string(),
                integrity: "def".to_string(),
                source: "https://example.test".to_string(),
                dependencies: Vec::new(),
            },
        );
        dependencies.insert(
            "bundle:acme/bundles/common".to_string(),
            LockDependency {
                version: "1.0.0".to_string(),
                integrity: String::new(),
                source: "https://example.test".to_string(),
                dependencies: vec!["schema:acme/schemas/thing".to_string()],
            },
        );
        LockManifest {
            version: "1".to_string(),
            dependencies,
        }
    }

    #[test]
    fn removable_schemas_skips_bundles_and_protected_ids() {
        let lock = sample_lock();
        let protected = HashSet::from(["acme/schemas/experiment".to_string()]);
        let removable = lock.removable_schemas(&protected);

        assert_eq!(removable.len(), 1);
        assert_eq!(removable[0].identifier(), "acme/schemas/thing");
    }

    #[test]
    fn load_from_dir_returns_empty_lock_when_missing() {
        let temp_dir = tempfile::TempDir::new().expect("create temp dir");
        let lock = LockManifest::load_from_dir(temp_dir.path()).expect("load missing lock");
        assert!(lock.dependencies.is_empty());
    }
}
