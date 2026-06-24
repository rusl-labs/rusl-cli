use anyhow::Context;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub mod credentials;

pub const DEFAULT_API_BASE_URL: &str = "https://resources.rusl.com";
pub const DEFAULT_WEBSITE_URL: &str = "https://rusl.com";
pub const DEFAULT_SCHEMA_DIR: &str = "./schemas";
pub const DEFAULT_SCHEMA_SUFFIX: &str = ".schema.json";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    #[serde(alias = "registry_url")]
    pub api_base_url: String,
    pub website_url: String,
    /// Directory where installed schemas are linked. Defaults to `./schemas`.
    pub schema_dir: String,
    /// Output configuration controlling how installed schemas are written to disk.
    pub output: OutputConfig,
    pub acting_account: Option<String>,
}

impl Config {
    /// Returns the effective schema directory.
    pub fn schema_dir(&self) -> &str {
        &self.schema_dir
    }

    /// Returns the suffix appended to schema identifiers when writing files.
    pub fn output_suffix(&self) -> &str {
        &self.output.suffix
    }
}

impl Default for Config {
    fn default() -> Self {
        let default_api = option_env!("RUSL_DEFAULT_API_URL").unwrap_or(DEFAULT_API_BASE_URL);
        let default_web = option_env!("RUSL_DEFAULT_WEBSITE_URL").unwrap_or(DEFAULT_WEBSITE_URL);

        Self {
            api_base_url: default_api.to_string(),
            website_url: default_web.to_string(),
            schema_dir: DEFAULT_SCHEMA_DIR.to_string(),
            output: OutputConfig::default(),
            acting_account: None,
        }
    }
}

/// Output configuration paired with [`Config::schema_dir`] that controls how
/// installed schemas are materialized on disk.
///
/// Schemas are written to `{schema_dir}/{identifier}{suffix}`, where
/// `identifier` is the canonical registry path (e.g. `acme/schemas/payment`).
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct OutputConfig {
    /// Suffix appended to the schema identifier when writing the file.
    /// Defaults to `.schema.json`.
    pub suffix: String,
}

impl Default for OutputConfig {
    fn default() -> Self {
        Self {
            suffix: DEFAULT_SCHEMA_SUFFIX.to_string(),
        }
    }
}

/// Partial config for overlay merging.
#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct PartialConfig {
    #[serde(alias = "registry_url")]
    pub api_base_url: Option<String>,
    pub website_url: Option<String>,
    pub schema_dir: Option<String>,
    pub output: Option<PartialOutputConfig>,
    pub acting_account: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct PartialOutputConfig {
    pub suffix: Option<String>,
}

/// Load config using the standard precedence order.
pub fn load() -> anyhow::Result<Config> {
    let mut config = Config::default();

    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "rusl") {
        let global_config = proj_dirs.config_dir().join("config.toml");
        if global_config.exists() {
            debug!("Found global config at: {:?}", global_config);
            let contents = std::fs::read_to_string(&global_config)?;
            let parsed: PartialConfig = toml::from_str(&contents).with_context(|| {
                format!(
                    "Global config at {:?} contains invalid TOML.",
                    global_config
                )
            })?;
            apply_partial(&mut config, parsed, None);
        }
    }

    let mut current_dir = std::env::current_dir().ok();
    while let Some(dir) = current_dir.as_ref() {
        let local_config = dir.join("rusl.config.toml");
        if local_config.exists() {
            debug!("Found local config at: {:?}", local_config);
            let contents = std::fs::read_to_string(&local_config)?;
            let parsed: PartialConfig = toml::from_str(&contents).with_context(|| {
                format!(
                    "Project config at {:?} contains invalid TOML.",
                    local_config
                )
            })?;
            apply_partial(&mut config, parsed, local_config.parent());
            break;
        }
        current_dir = dir.parent().map(|p| p.to_path_buf());
    }

    if let Ok(url) = std::env::var("RUSL_API_URL") {
        config.api_base_url = url;
    }
    if let Ok(url) = std::env::var("RUSL_WEBSITE_URL") {
        config.website_url = url;
    }

    Ok(config)
}

/// Apply a partial config overlay.
fn apply_partial(
    config: &mut Config,
    partial: PartialConfig,
    schema_dir_base: Option<&std::path::Path>,
) {
    if let Some(url) = partial.api_base_url {
        config.api_base_url = url;
    }
    if let Some(url) = partial.website_url {
        config.website_url = url;
    }
    if let Some(schema_dir) = partial.schema_dir {
        config.schema_dir = resolve_schema_dir(schema_dir, schema_dir_base);
    }
    if let Some(output) = partial.output
        && let Some(suffix) = output.suffix
    {
        config.output.suffix = suffix;
    }
    if let Some(acting_account) = partial.acting_account {
        config.acting_account = Some(acting_account);
    }
}

fn resolve_schema_dir(schema_dir: String, base_dir: Option<&std::path::Path>) -> String {
    let path = std::path::PathBuf::from(&schema_dir);
    if path.is_absolute() {
        return schema_dir;
    }

    match base_dir {
        Some(base_dir) => base_dir.join(path).to_string_lossy().into_owned(),
        None => schema_dir,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Config, DEFAULT_API_BASE_URL, DEFAULT_SCHEMA_DIR, DEFAULT_SCHEMA_SUFFIX,
        DEFAULT_WEBSITE_URL, load,
    };
    use serial_test::serial;
    use std::{ffi::OsString, path::PathBuf};
    use tempfile::TempDir;

    struct EnvGuard {
        previous_home: Option<OsString>,
        previous_xdg_config_home: Option<OsString>,
        previous_xdg_data_home: Option<OsString>,
        previous_api_url: Option<OsString>,
        previous_website_url: Option<OsString>,
        previous_dir: PathBuf,
    }

    impl EnvGuard {
        fn new(home_dir: &std::path::Path, workspace_dir: &std::path::Path) -> Self {
            let previous_home = std::env::var_os(home_var_name());
            let previous_xdg_config_home = std::env::var_os("XDG_CONFIG_HOME");
            let previous_xdg_data_home = std::env::var_os("XDG_DATA_HOME");
            let previous_api_url = std::env::var_os("RUSL_API_URL");
            let previous_website_url = std::env::var_os("RUSL_WEBSITE_URL");
            let previous_dir = std::env::current_dir().expect("current dir");

            set_env_var(home_var_name(), home_dir.as_os_str());
            set_env_var("XDG_CONFIG_HOME", home_dir.join(".config"));
            set_env_var("XDG_DATA_HOME", home_dir.join(".local").join("share"));
            std::env::set_current_dir(workspace_dir).expect("set workspace dir");

            Self {
                previous_home,
                previous_xdg_config_home,
                previous_xdg_data_home,
                previous_api_url,
                previous_website_url,
                previous_dir,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            restore_env_var(home_var_name(), self.previous_home.as_ref());
            restore_env_var("XDG_CONFIG_HOME", self.previous_xdg_config_home.as_ref());
            restore_env_var("XDG_DATA_HOME", self.previous_xdg_data_home.as_ref());
            restore_env_var("RUSL_API_URL", self.previous_api_url.as_ref());
            restore_env_var("RUSL_WEBSITE_URL", self.previous_website_url.as_ref());
            std::env::set_current_dir(&self.previous_dir).expect("restore current dir");
        }
    }

    #[test]
    fn production_domain_defaults_use_rusl_dot_com() {
        assert_eq!(DEFAULT_API_BASE_URL, "https://resources.rusl.com");
        assert_eq!(DEFAULT_WEBSITE_URL, "https://rusl.com");
    }

    #[test]
    fn schema_dir_defaults_to_schemas() {
        assert_eq!(DEFAULT_SCHEMA_DIR, "./schemas");
        assert_eq!(Config::default().schema_dir(), "./schemas");
    }

    #[test]
    fn output_suffix_defaults_to_schema_json() {
        assert_eq!(DEFAULT_SCHEMA_SUFFIX, ".schema.json");
        assert_eq!(Config::default().output_suffix(), ".schema.json");
    }

    #[test]
    #[serial]
    fn load_applies_global_then_project_then_environment_overrides() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let home_dir = temp_dir.path().join("home");
        let workspace_dir = temp_dir.path().join("workspace");
        let child_dir = workspace_dir.join("nested");
        std::fs::create_dir_all(&home_dir).expect("create home dir");
        std::fs::create_dir_all(&workspace_dir).expect("create workspace dir");
        std::fs::create_dir_all(&child_dir).expect("create nested dir");
        let _guard = EnvGuard::new(&home_dir, &child_dir);

        let global_config_path = project_config_dir().join("config.toml");
        std::fs::create_dir_all(global_config_path.parent().expect("config dir"))
            .expect("create global config dir");
        std::fs::write(
            &global_config_path,
            r#"
api_base_url = "https://global-api.example"
website_url = "https://global-web.example"
schema_dir = "global-schemas"
"#,
        )
        .expect("write global config");

        std::fs::write(
            workspace_dir.join("rusl.config.toml"),
            r#"
website_url = "https://project-web.example"
schema_dir = "project-schemas"

[output]
suffix = ".json"
"#,
        )
        .expect("write project config");

        set_env_var("RUSL_API_URL", "https://env-api.example");

        let config = load().expect("load config");

        assert_eq!(config.api_base_url, "https://env-api.example");
        assert_eq!(config.website_url, "https://project-web.example");
        assert_eq!(config.output_suffix(), ".json");
        assert_eq!(
            std::path::PathBuf::from(config.schema_dir()),
            workspace_dir
                .canonicalize()
                .expect("canonical workspace dir")
                .join("project-schemas")
        );
    }

    fn project_config_dir() -> PathBuf {
        directories::ProjectDirs::from("", "", "rusl")
            .expect("project dirs")
            .config_dir()
            .to_path_buf()
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
