use crate::config::credentials::Credentials;
use anyhow::{Context, Result};

pub fn logout() -> Result<()> {
    Credentials::clear().context("Failed to clear stored credentials")
}

#[cfg(test)]
mod tests {
    use super::logout;
    use crate::config::credentials::Credentials;
    use serial_test::serial;
    use std::{ffi::OsString, path::Path};
    use tempfile::TempDir;

    struct HomeGuard {
        previous_home: Option<OsString>,
        previous_xdg_config_home: Option<OsString>,
        previous_xdg_data_home: Option<OsString>,
    }

    impl HomeGuard {
        fn new(home_dir: &Path) -> Self {
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
    fn logout_clears_stored_credentials() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = HomeGuard::new(temp_dir.path());

        Credentials {
            access_token: "access-token".to_string(),
            refresh_token: "refresh-token".to_string(),
        }
        .save()
        .expect("save credentials");

        logout().expect("logout");

        assert!(Credentials::load().is_none());
    }

    #[test]
    #[serial]
    fn logout_succeeds_without_stored_credentials() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let _guard = HomeGuard::new(temp_dir.path());

        logout().expect("logout");

        assert!(Credentials::load().is_none());
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
