use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::fs::symlink as create_symlink;
#[cfg(windows)]
use std::os::windows::fs::symlink_file as create_symlink;

/// The project linker engine responsible for mapping CAS assets into the local project structure.
pub struct Linker {
    project_root: PathBuf,
}

impl Linker {
    pub fn new(cwd: PathBuf) -> Self {
        Self { project_root: cwd }
    }

    /// Brutally purges the entire local `.rusl/schemas` directory to eliminate orphan symlinks natively.
    pub fn purge_all(&self) -> Result<()> {
        let local_dir = self.project_root.join(".rusl").join("schemas");
        if local_dir.exists() {
            std::fs::remove_dir_all(&local_dir).with_context(|| {
                format!(
                    "Failed to securely prune legacy schema cache at {:?}",
                    local_dir
                )
            })?;
        }
        Ok(())
    }

    /// Maps a global schema directly into the local working directory namespace.
    /// This uses OS-native symbolic linking to guarantee zero-copy, instantly mirrored files.
    ///
    /// The target structure is `<cwd>/.rusl/schemas/<account>/<slug>.json`
    pub fn link_schema(&self, account: &str, slug: &str, cas_path: &Path) -> Result<PathBuf> {
        let local_dir = self
            .project_root
            .join(".rusl")
            .join("schemas")
            .join(account);

        if !local_dir.exists() {
            std::fs::create_dir_all(&local_dir).with_context(|| {
                format!("Failed to create local directory layer: {:?}", local_dir)
            })?;
        }

        let local_file = local_dir.join(format!("{}.json", slug));

        // Always purge existing symlinks forcefully to prevent stale graph retention
        if local_file.exists() || std::fs::symlink_metadata(&local_file).is_ok() {
            std::fs::remove_file(&local_file)
                .with_context(|| format!("Failed to purge existing symlink at {:?}", local_file))?;
        }

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

        Ok(local_file)
    }
}
