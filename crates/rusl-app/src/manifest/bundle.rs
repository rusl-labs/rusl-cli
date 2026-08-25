use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// The primary manifest defined by the user (rusl.bundle.toml).
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleManifest {
    /// Information about the bundle itself
    #[serde(default)]
    pub bundle: BundleMeta,

    /// Rusl-specific configuration and resource dependencies
    #[serde(default)]
    pub rusl: RuslManifest,

    /// External schemas pulled by pure HTTP(S) URL
    #[serde(default)]
    pub external: HashMap<String, String>,

    /// Deep conflict resolutions
    #[serde(default)]
    pub overrides: HashMap<String, String>,
}

impl BundleManifest {
    /// Canonical identifiers marked `dev` in `[rusl.resources]`.
    ///
    /// Install and cache-clear will not delete or overwrite these schema files.
    pub fn protected_resource_ids(&self) -> HashSet<String> {
        self.rusl
            .resources
            .iter()
            .filter(|(_, requirement)| requirement.is_dev())
            .map(|(identifier, _)| identifier.clone())
            .collect()
    }
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
    pub resources: HashMap<String, ResourceRequirement>,
}

/// A `[rusl.resources]` value: a version string, or an inline table with `dev`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ResourceRequirement {
    Version(String),
    Detailed(ResourceRequirementSpec),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceRequirementSpec {
    #[serde(default = "unconstrained_version")]
    pub version: String,
    #[serde(default)]
    pub dev: bool,
}

fn unconstrained_version() -> String {
    "*".to_string()
}

impl ResourceRequirement {
    pub fn version(&self) -> &str {
        match self {
            Self::Version(version) => version,
            Self::Detailed(spec) => &spec.version,
        }
    }

    pub fn is_dev(&self) -> bool {
        matches!(self, Self::Detailed(spec) if spec.dev)
    }
}

#[cfg(test)]
mod tests {
    use super::{BundleManifest, ResourceRequirement};

    #[test]
    fn parses_string_version_requirements() {
        let manifest: BundleManifest = toml::from_str(
            r#"
[rusl.resources]
"acme/schemas/user-profile" = "*"
"acme/bundles/common" = ">=1.2.0"
"#,
        )
        .expect("parse manifest");

        assert_eq!(
            manifest.rusl.resources["acme/schemas/user-profile"],
            ResourceRequirement::Version("*".to_string())
        );
        assert_eq!(
            manifest.rusl.resources["acme/bundles/common"].version(),
            ">=1.2.0"
        );
        assert!(!manifest.rusl.resources["acme/schemas/user-profile"].is_dev());
        assert!(manifest.protected_resource_ids().is_empty());
    }

    #[test]
    fn parses_dev_inline_table_and_defaults_version() {
        let manifest: BundleManifest = toml::from_str(
            r#"
[rusl.resources]
"acme/schemas/user-profile" = "*"
"acme/schemas/experiment" = { dev = true }
"acme/schemas/pinned-local" = { version = ">=1.0.0", dev = true }
"#,
        )
        .expect("parse manifest");

        assert!(manifest.rusl.resources["acme/schemas/experiment"].is_dev());
        assert_eq!(
            manifest.rusl.resources["acme/schemas/experiment"].version(),
            "*"
        );
        assert!(manifest.rusl.resources["acme/schemas/pinned-local"].is_dev());
        assert_eq!(
            manifest.rusl.resources["acme/schemas/pinned-local"].version(),
            ">=1.0.0"
        );

        let protected = manifest.protected_resource_ids();
        assert!(protected.contains("acme/schemas/experiment"));
        assert!(protected.contains("acme/schemas/pinned-local"));
        assert!(!protected.contains("acme/schemas/user-profile"));
    }
}
