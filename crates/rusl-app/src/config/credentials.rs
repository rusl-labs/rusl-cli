use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub access_token: String,
    pub refresh_token: String,
}

impl Credentials {
    /// Commits the structured credentials cryptographically into the user's isolated global filesystem securely
    pub fn save(&self) -> Result<()> {
        let proj_dirs = directories::ProjectDirs::from("", "", "rusl")
            .context("Attempting to isolate root directory access natively failed.")?;

        let path = proj_dirs.config_dir().join("credentials.toml");

        if !proj_dirs.config_dir().exists() {
            std::fs::create_dir_all(proj_dirs.config_dir())
                .context("Failed to safely map config path directory natively")?;
        }

        let toml_str = toml::to_string_pretty(self)?;
        std::fs::write(&path, toml_str)
            .context("Execution permissions rigidly blocked token storage natively.")?;

        // Guarantee rigid UNIX OS read boundaries natively isolated (rw-------)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&path)?.permissions();
            perms.set_mode(0o600);
            std::fs::set_permissions(&path, perms)?;
        }

        Ok(())
    }

    pub fn clear() -> Result<()> {
        let Some(proj_dirs) = directories::ProjectDirs::from("", "", "rusl") else {
            return Ok(());
        };

        let path = proj_dirs.config_dir().join("credentials.toml");
        if path.exists() {
            std::fs::remove_file(&path).context("Failed to remove stored credentials from disk")?;
        }

        Ok(())
    }

    /// Inherits credentials straight off the system disk transparently
    pub fn load() -> Option<Self> {
        let proj_dirs = directories::ProjectDirs::from("", "", "rusl")?;
        let path = proj_dirs.config_dir().join("credentials.toml");

        if path.exists() {
            let contents = std::fs::read_to_string(&path).ok()?;
            toml::from_str(&contents).ok()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Credentials;
    use serial_test::serial;
    use std::{ffi::OsString, path::PathBuf};
    use tempfile::TempDir;

    struct HomeGuard {
        previous_home: Option<OsString>,
        previous_xdg_config_home: Option<OsString>,
        previous_xdg_data_home: Option<OsString>,
    }

    impl HomeGuard {
        fn new(home_dir: &std::path::Path) -> Self {
            let previous_home = std::env::var_os(home_var_name());
            let previous_xdg_config_home = std::env::var_os("XDG_CONFIG_HOME");
            let previous_xdg_data_home = std::env::var_os("XDG_DATA_HOME");
            set_env_var(home_var_name(), home_dir.as_os_str());
            set_env_var("XDG_CONFIG_HOME", home_dir.join(".config"));
            set_env_var("XDG_DATA_HOME", home_dir.join(".local").join("share"));
            Self {
                previous_home,
                previous_xdg_config_home,
                previous_xdg_data_home,
            }
        }
    }

    impl Drop for HomeGuard {
        fn drop(&mut self) {
            restore_env_var(home_var_name(), self.previous_home.as_ref());
            restore_env_var("XDG_CONFIG_HOME", self.previous_xdg_config_home.as_ref());
            restore_env_var("XDG_DATA_HOME", self.previous_xdg_data_home.as_ref());
        }
    }

    #[test]
    #[serial]
    fn saves_and_loads_credentials_round_trip() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = HomeGuard::new(temp_dir.path());
        let credentials = Credentials {
            access_token: "access-token".to_string(),
            refresh_token: "refresh-token".to_string(),
        };

        credentials.save().expect("save credentials");

        let loaded = Credentials::load().expect("load credentials");
        assert_eq!(loaded.access_token, "access-token");
        assert_eq!(loaded.refresh_token, "refresh-token");
    }

    #[test]
    #[serial]
    fn clearing_credentials_removes_saved_file() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = HomeGuard::new(temp_dir.path());

        Credentials {
            access_token: "access-token".to_string(),
            refresh_token: "refresh-token".to_string(),
        }
        .save()
        .expect("save credentials");

        Credentials::clear().expect("clear credentials");

        assert!(Credentials::load().is_none());
        assert!(!credentials_path().exists());
    }

    #[test]
    #[serial]
    #[cfg(unix)]
    fn saves_credentials_with_user_only_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = HomeGuard::new(temp_dir.path());

        Credentials {
            access_token: "access-token".to_string(),
            refresh_token: "refresh-token".to_string(),
        }
        .save()
        .expect("save credentials");

        let mode = std::fs::metadata(credentials_path())
            .expect("credentials metadata")
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o600);
    }

    fn credentials_path() -> PathBuf {
        directories::ProjectDirs::from("", "", "rusl")
            .expect("project dirs")
            .config_dir()
            .join("credentials.toml")
    }

    #[cfg(windows)]
    fn home_var_name() -> &'static str {
        "USERPROFILE"
    }

    #[cfg(not(windows))]
    fn home_var_name() -> &'static str {
        "HOME"
    }

    fn set_env_var<K, V>(key: K, value: V)
    where
        K: AsRef<std::ffi::OsStr>,
        V: AsRef<std::ffi::OsStr>,
    {
        unsafe { std::env::set_var(key, value) }
    }

    fn restore_env_var(key: &str, value: Option<&OsString>) {
        match value {
            Some(value) => unsafe { std::env::set_var(key, value) },
            None => unsafe { std::env::remove_var(key) },
        }
    }
}
