use crate::resource_identifier::RegistryResource;
use crate::schema_naming::{NamingConvention, installed_schema_path};
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// The project linker engine responsible for mapping CAS assets into the local project structure.
pub struct Linker {
    schema_dir: PathBuf,
    /// Suffix appended to schema identifiers when writing files (e.g. `.schema.json`).
    suffix: String,
    naming_convention: NamingConvention,
}

impl Linker {
    pub fn new(
        cwd: PathBuf,
        schema_dir: &str,
        suffix: &str,
        naming_convention: NamingConvention,
    ) -> Self {
        Self {
            schema_dir: cwd.join(schema_dir),
            suffix: suffix.to_string(),
            naming_convention,
        }
    }

    /// Purges the entire schema directory to eliminate orphan schema files.
    pub fn purge_all(&self) -> Result<bool> {
        if self.schema_dir.exists() {
            std::fs::remove_dir_all(&self.schema_dir).with_context(|| {
                format!("Failed to prune schema cache at {:?}", self.schema_dir)
            })?;
            return Ok(true);
        }
        Ok(false)
    }

    /// Copies a schema from the global content store into the project install tree.
    ///
    /// The target structure is `{schema_dir}/{relative_path}{suffix}`, where
    /// `relative_path` is derived from the resource identifier according to the
    /// configured naming convention. Installed files are portable regular files
    /// (never absolute symlinks into the machine-local store).
    pub fn link_schema(&self, resource: &RegistryResource, cas_path: &Path) -> Result<PathBuf> {
        let local_file = installed_schema_path(
            &self.schema_dir,
            self.naming_convention,
            resource,
            &self.suffix,
        );

        if let Some(parent) = local_file.parent()
            && !parent.exists()
        {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create local directory layer: {:?}", parent))?;
        }

        // Purge existing file/symlink to prevent stale retention
        if local_file.exists() || std::fs::symlink_metadata(&local_file).is_ok() {
            std::fs::remove_file(&local_file)
                .with_context(|| format!("Failed to purge existing file at {:?}", local_file))?;
        }

        std::fs::copy(cas_path, &local_file)
            .with_context(|| format!("Failed to copy {:?} -> {:?}", cas_path, local_file))?;

        Ok(local_file)
    }
}

#[cfg(test)]
mod tests {
    use super::Linker;
    use crate::config::{DEFAULT_SCHEMA_DIR, DEFAULT_SCHEMA_SUFFIX};
    use crate::resource_identifier::RegistryResource;
    use crate::schema_naming::NamingConvention;
    use tempfile::TempDir;

    #[test]
    fn materializes_regular_file_copy_for_default_schema_dir() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let cwd = temp_dir.path().to_path_buf();
        let store_dir = cwd.join("store").join("abc123");
        std::fs::create_dir_all(&store_dir).expect("create store dir");
        let cas_path = store_dir.join("schema.json");
        std::fs::write(&cas_path, r#"{"title":"payment"}"#).expect("write cas blob");

        let linker = Linker::new(
            cwd.clone(),
            DEFAULT_SCHEMA_DIR,
            DEFAULT_SCHEMA_SUFFIX,
            NamingConvention::Normal,
        );
        let resource = RegistryResource::schema("acme/schemas/payment").expect("schema resource");

        let installed = linker
            .link_schema(&resource, &cas_path)
            .expect("materialize schema");

        let expected = cwd.join("schemas").join("acme").join("payment.schema.json");
        assert_eq!(installed, expected);
        assert_eq!(
            std::fs::read_to_string(&installed).expect("read installed"),
            r#"{"title":"payment"}"#
        );

        let metadata = std::fs::symlink_metadata(&installed).expect("symlink metadata");
        assert!(metadata.file_type().is_file());
        assert!(!metadata.file_type().is_symlink());
    }

    #[test]
    fn replaces_stale_absolute_symlink_with_regular_file() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let cwd = temp_dir.path().to_path_buf();
        let store_dir = cwd.join("store").join("deadbeef");
        std::fs::create_dir_all(&store_dir).expect("create store dir");
        let cas_path = store_dir.join("schema.json");
        std::fs::write(&cas_path, r#"{"title":"fresh"}"#).expect("write cas blob");

        let resource = RegistryResource::schema("acme/schemas/payment").expect("schema resource");
        let stale_target = cwd.join("missing-store").join("schema.json");
        let install_parent = cwd.join("schemas").join("acme");
        std::fs::create_dir_all(&install_parent).expect("create install parent");
        let install_path = install_parent.join("payment.schema.json");

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&stale_target, &install_path).expect("create stale symlink");
            assert!(
                std::fs::symlink_metadata(&install_path)
                    .expect("stale metadata")
                    .file_type()
                    .is_symlink()
            );
        }
        #[cfg(not(unix))]
        {
            // On non-Unix platforms, seed a stale regular file instead of a symlink.
            std::fs::write(&install_path, r#"{"title":"stale"}"#).expect("write stale file");
        }

        let linker = Linker::new(
            cwd,
            DEFAULT_SCHEMA_DIR,
            DEFAULT_SCHEMA_SUFFIX,
            NamingConvention::Normal,
        );
        let installed = linker
            .link_schema(&resource, &cas_path)
            .expect("replace stale path");

        assert_eq!(installed, install_path);
        assert_eq!(
            std::fs::read_to_string(&installed).expect("read installed"),
            r#"{"title":"fresh"}"#
        );
        let metadata = std::fs::symlink_metadata(&installed).expect("symlink metadata");
        assert!(metadata.file_type().is_file());
        assert!(!metadata.file_type().is_symlink());
    }
}
