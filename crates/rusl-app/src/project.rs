//! Project-root discovery for `rusl.bundle.toml` and its sibling `rusl.lock`.

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

pub const BUNDLE_MANIFEST_NAME: &str = "rusl.bundle.toml";
pub const LOCKFILE_NAME: &str = "rusl.lock";

/// Location of the nearest `rusl.bundle.toml` found by walking up from a start directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleProject {
    /// Directory that contains `rusl.bundle.toml` (and where `rusl.lock` belongs).
    pub root: PathBuf,
}

impl BundleProject {
    pub fn manifest_path(&self) -> PathBuf {
        self.root.join(BUNDLE_MANIFEST_NAME)
    }

    pub fn lock_path(&self) -> PathBuf {
        self.root.join(LOCKFILE_NAME)
    }
}

/// Walk up from `start` until `rusl.bundle.toml` is found. Nearest wins.
///
/// Fails with an error that lists every directory searched when none is found.
pub fn discover_bundle(start: &Path) -> Result<BundleProject> {
    let mut searched = Vec::new();
    let mut current = Some(start.to_path_buf());

    while let Some(dir) = current {
        searched.push(dir.clone());
        let candidate = dir.join(BUNDLE_MANIFEST_NAME);
        if candidate.is_file() {
            return Ok(BundleProject { root: dir });
        }
        current = dir.parent().map(|p| p.to_path_buf());
    }

    bail!(format_not_found_error(&searched));
}

/// Discover the nearest bundle starting at the process current working directory.
pub fn discover_bundle_from_cwd() -> Result<BundleProject> {
    let cwd = std::env::current_dir().context("Failed to get current working directory")?;
    discover_bundle(&cwd)
}

fn format_not_found_error(searched: &[PathBuf]) -> String {
    let mut message = String::from("No rusl.bundle.toml found. Searched:");
    for dir in searched {
        message.push_str("\n  ");
        message.push_str(&dir.display().to_string());
    }
    message
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn discovers_bundle_in_cwd() {
        let temp = TempDir::new().expect("temp");
        write_bundle(temp.path());

        let project = discover_bundle(temp.path()).expect("discover");
        assert_eq!(project.root, temp.path());
        assert_eq!(
            project.manifest_path(),
            temp.path().join(BUNDLE_MANIFEST_NAME)
        );
        assert_eq!(project.lock_path(), temp.path().join(LOCKFILE_NAME));
    }

    #[test]
    fn discovers_bundle_two_levels_up() {
        let temp = TempDir::new().expect("temp");
        write_bundle(temp.path());
        let nested = temp.path().join("packages").join("schemas");
        fs::create_dir_all(&nested).expect("mkdir");

        let project = discover_bundle(&nested).expect("discover");
        assert_eq!(project.root, temp.path());
    }

    #[test]
    fn nearest_bundle_wins_when_present_at_multiple_levels() {
        let temp = TempDir::new().expect("temp");
        write_bundle(temp.path());
        let nested = temp.path().join("packages").join("schemas");
        fs::create_dir_all(&nested).expect("mkdir");
        write_bundle(&nested);

        let project = discover_bundle(&nested).expect("discover");
        assert_eq!(project.root, nested);
    }

    #[test]
    fn missing_bundle_lists_searched_directories() {
        let temp = TempDir::new().expect("temp");
        let nested = temp.path().join("a").join("b");
        fs::create_dir_all(&nested).expect("mkdir");

        let err = discover_bundle(&nested).expect_err("should fail");
        let message = format!("{err:#}");
        assert!(message.contains("No rusl.bundle.toml found. Searched:"));
        assert!(message.contains(&nested.display().to_string()));
        assert!(message.contains(&temp.path().join("a").display().to_string()));
        assert!(message.contains(&temp.path().display().to_string()));
    }

    fn write_bundle(dir: &Path) {
        fs::write(
            dir.join(BUNDLE_MANIFEST_NAME),
            "[rusl.resources]\n\"acme/schemas/demo\" = \"*\"\n",
        )
        .expect("write bundle");
    }
}
