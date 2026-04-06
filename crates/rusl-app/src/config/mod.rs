use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

pub mod credentials;

/// How a generator's command is specified in config.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommandSpec {
    /// A single string split on whitespace at runtime: `"bunx rusl-gen-typescript"`
    Simple(String),
    /// An explicit argv array used as-is: `["python3", "-m", "rusl_gen_python"]`
    Explicit(Vec<String>),
}

impl CommandSpec {
    /// Resolve into an argv vector (program + arguments).
    pub fn to_argv(&self) -> Vec<String> {
        match self {
            CommandSpec::Simple(s) => s.split_whitespace().map(String::from).collect(),
            CommandSpec::Explicit(v) => v.clone(),
        }
    }

    /// Human-readable display of the command.
    pub fn display(&self) -> String {
        match self {
            CommandSpec::Simple(s) => s.clone(),
            CommandSpec::Explicit(v) => v.join(" "),
        }
    }
}

/// Configuration for a single code-generation plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct GeneratorConfig {
    /// Plugin transport — only `"stdio"` is supported in v1.
    #[serde(rename = "type")]
    pub plugin_type: String,
    /// The executable (and optional arguments) to spawn.
    pub command: Option<CommandSpec>,
    /// Relative path from project root where generated files are written.
    pub output_dir: Option<String>,
    /// Whether this generator is invoked by bare `rusl generate`.
    pub default: bool,
    /// Set `false` to suppress a globally-configured generator in this project.
    pub enabled: bool,
    /// Wipe output_dir before generating.
    pub clean: bool,
    /// Glob patterns for schema selection (empty = all).
    pub filter: Vec<String>,
    /// Opaque key-value bag forwarded to the plugin as `options`.
    pub args: serde_json::Value,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            plugin_type: "stdio".to_string(),
            command: None,
            output_dir: None,
            default: false,
            enabled: true,
            clean: true,
            filter: Vec::new(),
            args: serde_json::Value::Object(serde_json::Map::new()),
        }
    }
}

/// Tracks which config tier a generator was loaded from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigTier {
    Global,
    Project,
}

/// A generator with its resolved source tier.
#[derive(Debug, Clone)]
pub struct EffectiveGenerator {
    pub config: GeneratorConfig,
    pub tier: ConfigTier,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    #[serde(alias = "registry_url")]
    pub api_base_url: String,
    pub website_url: String,
    /// Directory where installed schemas are linked. Defaults to `.rusl/schemas`.
    pub schema_dir: Option<String>,
    #[serde(default)]
    pub generators: HashMap<String, GeneratorConfig>,
}

impl Config {
    /// Returns the effective schema directory, defaulting to `.rusl/schemas`.
    pub fn schema_dir(&self) -> &str {
        self.schema_dir.as_deref().unwrap_or(".rusl/schemas")
    }
}

impl Default for Config {
    fn default() -> Self {
        let default_api = option_env!("RUSL_DEFAULT_API_URL").unwrap_or("https://api.rusl.app");
        let default_web = option_env!("RUSL_DEFAULT_WEBSITE_URL").unwrap_or("https://rusl.app");

        Self {
            api_base_url: default_api.to_string(),
            website_url: default_web.to_string(),
            schema_dir: None,
            generators: HashMap::new(),
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
    #[serde(default)]
    pub generators: HashMap<String, GeneratorConfig>,
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
            apply_partial(&mut config, parsed);
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
            apply_partial(&mut config, parsed);
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
fn apply_partial(config: &mut Config, partial: PartialConfig) {
    if let Some(url) = partial.api_base_url {
        config.api_base_url = url;
    }
    if let Some(url) = partial.website_url {
        config.website_url = url;
    }
    if partial.schema_dir.is_some() {
        config.schema_dir = partial.schema_dir;
    }
    for (name, gcfg) in partial.generators {
        config.generators.insert(name, gcfg);
    }
}

/// Load generators with tier information for `--list` display.
pub fn load_generators_with_tiers() -> anyhow::Result<Vec<(String, EffectiveGenerator)>> {
    let mut result: HashMap<String, EffectiveGenerator> = HashMap::new();

    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "rusl") {
        let global_config = proj_dirs.config_dir().join("config.toml");
        if global_config.exists() {
            let contents = std::fs::read_to_string(&global_config)?;
            let parsed: PartialConfig =
                toml::from_str(&contents).with_context(|| "Failed to parse global config")?;
            for (name, gcfg) in parsed.generators {
                result.insert(
                    name,
                    EffectiveGenerator {
                        config: gcfg,
                        tier: ConfigTier::Global,
                    },
                );
            }
        }
    }

    let mut current_dir = std::env::current_dir().ok();
    while let Some(dir) = current_dir.as_ref() {
        let local_config = dir.join("rusl.config.toml");
        if local_config.exists() {
            let contents = std::fs::read_to_string(&local_config)?;
            let parsed: PartialConfig =
                toml::from_str(&contents).with_context(|| "Failed to parse project config")?;
            for (name, gcfg) in parsed.generators {
                result.insert(
                    name,
                    EffectiveGenerator {
                        config: gcfg,
                        tier: ConfigTier::Project,
                    },
                );
            }
            break;
        }
        current_dir = dir.parent().map(|p| p.to_path_buf());
    }

    let mut sorted: Vec<_> = result.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(sorted)
}

#[cfg(test)]
mod tests {
    use super::{ConfigTier, load, load_generators_with_tiers};
    use serial_test::serial;
    use std::{ffi::OsString, path::PathBuf};
    use tempfile::TempDir;

    struct EnvGuard {
        previous_home: Option<OsString>,
        previous_api_url: Option<OsString>,
        previous_website_url: Option<OsString>,
        previous_dir: PathBuf,
    }

    impl EnvGuard {
        fn new(home_dir: &std::path::Path, workspace_dir: &std::path::Path) -> Self {
            let previous_home = std::env::var_os(home_var_name());
            let previous_api_url = std::env::var_os("RUSL_API_URL");
            let previous_website_url = std::env::var_os("RUSL_WEBSITE_URL");
            let previous_dir = std::env::current_dir().expect("current dir");

            set_env_var(home_var_name(), home_dir.as_os_str());
            std::env::set_current_dir(workspace_dir).expect("set workspace dir");

            Self {
                previous_home,
                previous_api_url,
                previous_website_url,
                previous_dir,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            restore_env_var(home_var_name(), self.previous_home.as_ref());
            restore_env_var("RUSL_API_URL", self.previous_api_url.as_ref());
            restore_env_var("RUSL_WEBSITE_URL", self.previous_website_url.as_ref());
            std::env::set_current_dir(&self.previous_dir).expect("restore current dir");
        }
    }

    #[test]
    #[serial]
    fn load_applies_global_then_project_then_environment_overrides() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let home_dir = temp_dir.path().join("home");
        let workspace_dir = temp_dir.path().join("workspace");
        std::fs::create_dir_all(&home_dir).expect("create home dir");
        std::fs::create_dir_all(&workspace_dir).expect("create workspace dir");
        let _guard = EnvGuard::new(&home_dir, &workspace_dir);

        let global_config_path = project_config_dir().join("config.toml");
        std::fs::create_dir_all(global_config_path.parent().expect("config dir"))
            .expect("create global config dir");
        std::fs::write(
            &global_config_path,
            r#"
api_base_url = "https://global-api.example"
website_url = "https://global-web.example"
schema_dir = "global-schemas"

[generators.global]
type = "stdio"
command = "global-gen"
output_dir = "global-out"
"#,
        )
        .expect("write global config");

        std::fs::write(
            workspace_dir.join("rusl.config.toml"),
            r#"
website_url = "https://project-web.example"
schema_dir = "project-schemas"

[generators.project]
type = "stdio"
command = "project-gen"
output_dir = "project-out"
"#,
        )
        .expect("write project config");

        set_env_var("RUSL_API_URL", "https://env-api.example");

        let config = load().expect("load config");

        assert_eq!(config.api_base_url, "https://env-api.example");
        assert_eq!(config.website_url, "https://project-web.example");
        assert_eq!(config.schema_dir(), "project-schemas");
        assert!(config.generators.contains_key("global"));
        assert!(config.generators.contains_key("project"));
    }

    #[test]
    #[serial]
    fn load_generators_with_tiers_prefers_project_over_global() {
        let temp_dir = TempDir::new().expect("create temp dir");
        let home_dir = temp_dir.path().join("home");
        let workspace_dir = temp_dir.path().join("workspace");
        std::fs::create_dir_all(&home_dir).expect("create home dir");
        std::fs::create_dir_all(&workspace_dir).expect("create workspace dir");
        let _guard = EnvGuard::new(&home_dir, &workspace_dir);

        let global_config_path = project_config_dir().join("config.toml");
        std::fs::create_dir_all(global_config_path.parent().expect("config dir"))
            .expect("create global config dir");
        std::fs::write(
            &global_config_path,
            r#"
[generators.shared]
type = "stdio"
command = "global-gen"
output_dir = "global-out"

[generators.only_global]
type = "stdio"
command = "global-only"
output_dir = "global-only-out"
"#,
        )
        .expect("write global config");

        std::fs::write(
            workspace_dir.join("rusl.config.toml"),
            r#"
[generators.shared]
type = "stdio"
command = "project-gen"
output_dir = "project-out"

[generators.only_project]
type = "stdio"
command = "project-only"
output_dir = "project-only-out"
"#,
        )
        .expect("write project config");

        let generators = load_generators_with_tiers().expect("load generators");

        assert_eq!(generators.len(), 3);
        let shared = generators
            .iter()
            .find(|(name, _)| name == "shared")
            .expect("shared generator");
        assert_eq!(shared.1.tier, ConfigTier::Project);
        assert_eq!(
            shared.1.config.command.as_ref().expect("command").display(),
            "project-gen"
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
