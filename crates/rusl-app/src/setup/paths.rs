use anyhow::{Context, Result};
use std::path::PathBuf;

/// Root of Rusl user state used by agent setup (`~/.rusl`).
pub fn rusl_home() -> Result<PathBuf> {
    let home = dirs_home().context("Could not determine home directory for ~/.rusl")?;
    Ok(home.join(".rusl"))
}

pub fn cache_dir() -> Result<PathBuf> {
    Ok(rusl_home()?.join("cache"))
}

/// Cached agent-kit checkout (real clone or symlink to a local working tree).
pub fn agent_kit_cache_dir() -> Result<PathBuf> {
    Ok(cache_dir()?.join("rusl-agent-kit"))
}

/// Pack root inside a kit checkout: `<kit>/pack`.
pub fn pack_dir_in_kit(kit_root: impl AsRef<std::path::Path>) -> PathBuf {
    kit_root.as_ref().join("pack")
}

pub fn agent_skills_lock_path() -> Result<PathBuf> {
    Ok(rusl_home()?.join("agent-skills-lock.toml"))
}

fn dirs_home() -> Option<PathBuf> {
    // Prefer HOME so tests can override with EnvGuard-style HOME injection.
    if let Ok(home) = std::env::var("HOME")
        && !home.is_empty()
    {
        return Some(PathBuf::from(home));
    }
    #[cfg(windows)]
    {
        if let Ok(home) = std::env::var("USERPROFILE") {
            if !home.is_empty() {
                return Some(PathBuf::from(home));
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::ffi::OsString;
    use tempfile::TempDir;

    struct HomeGuard {
        previous: Option<OsString>,
    }

    impl HomeGuard {
        fn set(home: &std::path::Path) -> Self {
            let previous = std::env::var_os("HOME");
            unsafe { std::env::set_var("HOME", home) };
            Self { previous }
        }
    }

    impl Drop for HomeGuard {
        fn drop(&mut self) {
            match &self.previous {
                Some(v) => unsafe { std::env::set_var("HOME", v) },
                None => unsafe { std::env::remove_var("HOME") },
            }
        }
    }

    #[test]
    #[serial]
    fn rusl_paths_live_under_home_dot_rusl() {
        let tmp = TempDir::new().unwrap();
        let _guard = HomeGuard::set(tmp.path());

        assert_eq!(rusl_home().unwrap(), tmp.path().join(".rusl"));
        assert_eq!(
            agent_kit_cache_dir().unwrap(),
            tmp.path().join(".rusl/cache/rusl-agent-kit")
        );
        assert_eq!(
            agent_skills_lock_path().unwrap(),
            tmp.path().join(".rusl/agent-skills-lock.toml")
        );
    }
}
