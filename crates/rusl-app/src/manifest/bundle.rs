use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The primary manifest defined by the user (rusl.bundle.toml).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleManifest {
    /// Information about the bundle itself
    pub bundle: BundleMeta,

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleMeta {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
}
