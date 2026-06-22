use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The primary manifest defined by the user (rusl.bundle.toml).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleManifest {
    /// Information about the bundle itself
    #[serde(default)]
    pub bundle: BundleMeta,

    /// Rusl-specific configuration and resource dependencies
    #[serde(default)]
    pub rusl: RuslManifest,

    /// The schemas directly required by this project
    #[serde(default)]
    pub schemas: HashMap<String, String>,

    /// The nested bundles directly required by this project
    #[serde(default)]
    pub bundles: HashMap<String, String>,

    /// External schemas pulled by pure HTTP(S) URL
    #[serde(default)]
    pub external: HashMap<String, String>,

    /// Deep conflict resolutions
    #[serde(default)]
    pub overrides: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BundleMeta {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RuslManifest {
    /// Canonical resource identifiers directly required by this project
    #[serde(default)]
    pub resources: HashMap<String, String>,
}
