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
    /// When true, copy files instead of symlinking (for committable output).
    copy_mode: bool,
}

impl Linker {
    pub fn new(cwd: PathBuf, schema_dir: &str) -> Self {
        let copy_mode = schema_dir != DEFAULT_SCHEMA_DIR;
        Self {
            schema_dir: cwd.join(schema_dir),
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
    /// The target structure is `<schema_dir>/<account>/<slug>.json`
    pub fn link_schema(&self, account: &str, slug: &str, cas_path: &Path) -> Result<PathBuf> {
        let local_dir = self.schema_dir.join(account);

        if !local_dir.exists() {
            std::fs::create_dir_all(&local_dir).with_context(|| {
                format!("Failed to create local directory layer: {:?}", local_dir)
            })?;
        }

        let local_file = local_dir.join(format!("{}.json", slug));

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
