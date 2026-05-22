use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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

    /// The direct dependencies of this package (e.g., ["schema:acme/types", "bundle:acme/bundles/common"])
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
