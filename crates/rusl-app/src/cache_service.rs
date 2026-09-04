use crate::cache::linker::Linker;
use crate::cache::store::GlobalStore;
use crate::config;
use crate::manifest::bundle::BundleManifest;
use crate::manifest::lock::LockManifest;
use anyhow::{Context, Result};
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CacheClearResult {
    pub global_store_cleared: bool,
    pub local_schema_cache_cleared: bool,
}

pub async fn clear_cache() -> Result<CacheClearResult> {
    let config = config::load().context("Failed to load hierarchical configuration")?;
    let cwd = std::env::current_dir().context("Failed to get current working directory")?;
    let project = crate::project::discover_bundle(&cwd)?;

    let (global_store_cleared, _) = GlobalStore::clear_default()
        .await
        .context("Failed to clear global cache store")?;

    let lock = LockManifest::load_from_dir(&project.root).context("Failed to read rusl.lock")?;
    let protected_ids = load_protected_resource_ids(&project.root)?;
    let linker = Linker::for_project(cwd, &config);
    let local_schema_cache_cleared =
        linker.purge_installed(&lock.removable_schemas(&protected_ids))?;

    Ok(CacheClearResult {
        global_store_cleared,
        local_schema_cache_cleared,
    })
}

fn load_protected_resource_ids(root: &Path) -> Result<HashSet<String>> {
    let path = root.join("rusl.bundle.toml");
    let contents = std::fs::read_to_string(&path).context("Failed to read rusl.bundle.toml")?;
    let manifest: BundleManifest =
        toml::from_str(&contents).context("Failed to parse rusl.bundle.toml")?;
    Ok(manifest.protected_resource_ids())
}

#[cfg(test)]
mod tests {
    use super::clear_cache;
    use crate::cache::store::GlobalStore;
    use serial_test::serial;
    use std::{ffi::OsString, path::PathBuf};
    use tempfile::TempDir;

    struct EnvGuard {
        previous_home: Option<OsString>,
        previous_xdg_config_home: Option<OsString>,
        previous_xdg_data_home: Option<OsString>,
        previous_dir: PathBuf,
    }

    impl EnvGuard {
        fn new(home_dir: &std::path::Path, workspace_dir: &std::path::Path) -> Self {
            let previous_home = std::env::var_os(home_var_name());
            let previous_xdg_config_home = std::env::var_os("XDG_CONFIG_HOME");
            let previous_xdg_data_home = std::env::var_os("XDG_DATA_HOME");
            let previous_dir = std::env::current_dir().expect("current dir");

            set_env_var(home_var_name(), home_dir.as_os_str());
            set_env_var("XDG_CONFIG_HOME", home_dir.join(".config"));
            set_env_var("XDG_DATA_HOME", home_dir.join(".local").join("share"));
            std::env::set_current_dir(workspace_dir).expect("set workspace dir");

            Self {
                previous_home,
                previous_xdg_config_home,
                previous_xdg_data_home,
                previous_dir,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            restore_env_var(home_var_name(), self.previous_home.as_ref());
            restore_env_var("XDG_CONFIG_HOME", self.previous_xdg_config_home.as_ref());
            restore_env_var("XDG_DATA_HOME", self.previous_xdg_data_home.as_ref());
            std::env::set_current_dir(&self.previous_dir).expect("restore current dir");
        }
    }

    #[tokio::test]
    #[serial]
    async fn clear_cache_removes_global_store_and_local_schema_links_only() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let home_dir = temp_dir.path().join("home");
        let workspace_dir = temp_dir.path().join("workspace");
        std::fs::create_dir_all(&home_dir).expect("create home dir");
        std::fs::create_dir_all(&workspace_dir).expect("create workspace dir");
        let _guard = EnvGuard::new(&home_dir, &workspace_dir);

        let store = GlobalStore::new().await.expect("create store");
        let store_root = store.root_dir().to_path_buf();
        let (_, blob_path) = store
            .put(br#"{"type":"object"}"#)
            .await
            .expect("write blob");
        assert!(blob_path.exists());

        let schema_dir = workspace_dir.join("schemas");
        std::fs::create_dir_all(schema_dir.join("acme")).expect("create schema dir");
        std::fs::create_dir_all(schema_dir.join("local")).expect("create local schema dir");
        std::fs::write(schema_dir.join("acme").join("thing.schema.json"), "{}")
            .expect("write downloaded schema");
        std::fs::write(
            schema_dir.join("acme").join("experiment.schema.json"),
            "{\"dev\":true}",
        )
        .expect("write dev schema");
        std::fs::write(
            schema_dir.join("local").join("mine.schema.json"),
            "{\"local\":true}",
        )
        .expect("write unmanaged schema");
        std::fs::write(
            workspace_dir.join("rusl.bundle.toml"),
            r#"
[rusl.resources]
"acme/schemas/thing" = "*"
"acme/schemas/experiment" = { dev = true }
"#,
        )
        .expect("write manifest");
        std::fs::write(
            workspace_dir.join("rusl.lock"),
            r#"
version = "1"

[dependencies."schema:acme/schemas/thing"]
version = "1.0.0"
integrity = "abc"
source = "https://example.test"

[dependencies."schema:acme/schemas/experiment"]
version = "1.0.0"
integrity = "def"
source = "https://example.test"
"#,
        )
        .expect("write lockfile");

        let result = clear_cache().await.expect("clear cache");

        assert!(result.global_store_cleared);
        assert!(result.local_schema_cache_cleared);
        assert!(!store_root.exists());
        assert!(!schema_dir.join("acme").join("thing.schema.json").exists());
        assert_eq!(
            std::fs::read_to_string(schema_dir.join("acme").join("experiment.schema.json"))
                .expect("read dev schema"),
            "{\"dev\":true}"
        );
        assert_eq!(
            std::fs::read_to_string(schema_dir.join("local").join("mine.schema.json"))
                .expect("read unmanaged schema"),
            "{\"local\":true}"
        );
        assert!(workspace_dir.join("rusl.bundle.toml").exists());
        assert!(workspace_dir.join("rusl.lock").exists());
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
