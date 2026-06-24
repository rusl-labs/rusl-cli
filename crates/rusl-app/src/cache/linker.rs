use crate::config::DEFAULT_SCHEMA_DIR;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::symlink as create_symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_file as create_symlink;

/// The project linker engine responsible for mapping CAS assets into the local project structure.
pub struct Linker {
    schema_dir: PathBuf,
    /// Suffix appended to schema identifiers when writing files (e.g. `.schema.json`).
    suffix: String,
    /// When true, copy files instead of symlinking (for committable output).
    copy_mode: bool,
}

impl Linker {
    pub fn new(cwd: PathBuf, schema_dir: &str, suffix: &str) -> Self {
        let copy_mode = schema_dir != DEFAULT_SCHEMA_DIR;
        Self {
            schema_dir: cwd.join(schema_dir),
            suffix: suffix.to_string(),
            copy_mode,
        }
    }

    /// Purges the entire schema directory to eliminate orphan symlinks.
    pub fn purge_all(&self) -> Result<bool> {
        if self.schema_dir.exists() {
            std::fs::remove_dir_all(&self.schema_dir).with_context(|| {
                format!("Failed to prune schema cache at {:?}", self.schema_dir)
            })?;
            return Ok(true);
        }
        Ok(false)
    }

    /// Maps a global schema directly into the local working directory namespace.
    /// This uses OS-native symbolic linking to guarantee zero-copy, instantly mirrored files.
    ///
    /// The target structure is `{schema_dir}/{identifier}{suffix}`, where `identifier`
    /// is the canonical registry path (e.g. `acme/schemas/payment`). Intermediate
    /// directories are created as needed.
    pub fn link_schema(&self, identifier: &str, cas_path: &Path) -> Result<PathBuf> {
        let local_file = self
            .schema_dir
            .join(format!("{}{}", identifier, self.suffix));

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

        if self.copy_mode {
            std::fs::copy(cas_path, &local_file)
                .with_context(|| format!("Failed to copy {:?} -> {:?}", cas_path, local_file))?;
        } else {
            #[cfg(windows)]
            let link_res = create_symlink(cas_path, &local_file)
                .or_else(|_| std::fs::copy(cas_path, &local_file).map(|_| ()));

            #[cfg(not(windows))]
            let link_res = create_symlink(cas_path, &local_file);

            link_res.with_context(|| {
                format!(
                    "Symlink operation failed for {:?} -> {:?}",
                    cas_path, local_file
                )
            })?;
        }

        Ok(local_file)
    }
}
