use crate::resource_identifier::{RegistryResource, ResourceKind};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Maps a resolved schema resource to a relative path within `schema_dir`.
pub trait SchemaPathNaming {
    fn relative_path(&self, resource: &RegistryResource) -> String;
}

/// Built-in naming conventions selectable via config.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NamingConvention {
    /// Mirror the canonical identifier (e.g. `rusl/schemas/common`).
    Full,
    /// Strip the constant `schemas` segment (e.g. `rusl/common`).
    #[default]
    Normal,
    /// Single-directory, collision-safe names (e.g. `rusl_common`).
    Flat,
}

impl SchemaPathNaming for NamingConvention {
    fn relative_path(&self, resource: &RegistryResource) -> String {
        // `resource.slug` is the final identifier segment, treated opaquely. For a
        // packaged schema it is the compound `package.leaf` (e.g.
        // `payments.checkout`) and therefore contains dots — the dots become part
        // of the file stem, and the compound keeps same-leaf schemas in different
        // packages from colliding under every convention.
        match self {
            Self::Full => resource.identifier(),
            Self::Normal => format!("{}/{}", resource.account, resource.slug),
            Self::Flat => format!("{}_{}", resource.account, resource.slug),
        }
    }
}

/// Returns the relative path (without suffix) for an installed schema file.
pub fn installed_schema_relative_path(
    naming: NamingConvention,
    resource: &RegistryResource,
) -> String {
    assert_eq!(
        resource.kind,
        ResourceKind::Schema,
        "installed schema paths apply only to schema resources"
    );
    naming.relative_path(resource)
}

/// Returns the absolute path where an installed schema file is materialized.
pub fn installed_schema_path(
    schema_dir: &Path,
    naming: NamingConvention,
    resource: &RegistryResource,
    suffix: &str,
) -> PathBuf {
    schema_dir.join(format!(
        "{}{}",
        installed_schema_relative_path(naming, resource),
        suffix
    ))
}

#[cfg(test)]
mod tests {
    use super::{NamingConvention, SchemaPathNaming, installed_schema_path};
    use crate::resource_identifier::RegistryResource;
    use std::path::{Path, PathBuf};

    fn rusl_common() -> RegistryResource {
        RegistryResource::schema("rusl/schemas/common").expect("schema resource")
    }

    fn user_profile() -> RegistryResource {
        RegistryResource::schema("acme/schemas/user-profile").expect("schema resource")
    }

    #[test]
    fn full_mirrors_canonical_identifier() {
        let resource = rusl_common();
        assert_eq!(
            NamingConvention::Full.relative_path(&resource),
            "rusl/schemas/common"
        );
    }

    #[test]
    fn normal_strips_schemas_segment() {
        let resource = rusl_common();
        assert_eq!(
            NamingConvention::Normal.relative_path(&resource),
            "rusl/common"
        );
    }

    #[test]
    fn flat_uses_account_and_slug_with_underscore() {
        let resource = rusl_common();
        assert_eq!(
            NamingConvention::Flat.relative_path(&resource),
            "rusl_common"
        );
    }

    #[test]
    fn flat_preserves_multi_segment_slugs() {
        let resource = user_profile();
        assert_eq!(
            NamingConvention::Flat.relative_path(&resource),
            "acme_user-profile"
        );
    }

    #[test]
    fn packaged_compound_slug_is_preserved_in_paths() {
        let resource =
            RegistryResource::schema("acme/schemas/payments.checkout").expect("schema resource");
        assert_eq!(
            NamingConvention::Full.relative_path(&resource),
            "acme/schemas/payments.checkout"
        );
        assert_eq!(
            NamingConvention::Normal.relative_path(&resource),
            "acme/payments.checkout"
        );
        assert_eq!(
            NamingConvention::Flat.relative_path(&resource),
            "acme_payments.checkout"
        );
    }

    #[test]
    fn same_leaf_in_different_packages_never_collides() {
        // Two schemas share the leaf `checkout` but live in different packages.
        // The compound final segment must keep their install paths distinct under
        // every naming convention, or one would overwrite the other on disk.
        let payments =
            RegistryResource::schema("acme/schemas/payments.checkout").expect("schema resource");
        let billing =
            RegistryResource::schema("acme/schemas/billing.checkout").expect("schema resource");

        for convention in [
            NamingConvention::Full,
            NamingConvention::Normal,
            NamingConvention::Flat,
        ] {
            assert_ne!(
                convention.relative_path(&payments),
                convention.relative_path(&billing),
                "{convention:?} collided two same-leaf schemas from different packages"
            );
        }
    }

    #[test]
    fn installed_schema_path_appends_suffix() {
        let resource = rusl_common();
        let path = installed_schema_path(
            Path::new("./schemas"),
            NamingConvention::Normal,
            &resource,
            ".schema.json",
        );
        assert_eq!(path, PathBuf::from("./schemas/rusl/common.schema.json"));
    }

    #[test]
    fn default_convention_is_normal() {
        assert_eq!(NamingConvention::default(), NamingConvention::Normal);
    }
}
