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
        // Compile-time fallback URLs dynamically routing traffic securely.
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

/// Partial config for overlay merging — all fields optional so we only
/// overwrite what the user explicitly set in a given config file.
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

/// Loads the config strictly via a hierarchical precedence sequence natively
pub fn load() -> anyhow::Result<Config> {
    let mut config = Config::default();

    // 1. Start with global config as the base layer
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "rusl") {
        let global_config = proj_dirs.config_dir().join("config.toml");
        if global_config.exists() {
            debug!("Found global config at: {:?}", global_config);
            let contents = std::fs::read_to_string(&global_config)?;
            let parsed: PartialConfig = toml::from_str(&contents)
                .with_context(|| format!("Configuration Parse Error: Global config at {:?} contains invalid TOML syntax.", global_config))?;
            apply_partial(&mut config, parsed);
        }
    }

    // 2. Overlay project config (search upward from cwd)
    let mut current_dir = std::env::current_dir().ok();
    while let Some(dir) = current_dir.as_ref() {
        let local_config = dir.join("rusl.config.toml");
        if local_config.exists() {
            debug!("Found local config at: {:?}", local_config);
            let contents = std::fs::read_to_string(&local_config)?;
            let parsed: PartialConfig = toml::from_str(&contents)
                .with_context(|| format!("Configuration Parse Error: File {:?} exists but contains invalid TOML syntax. Please ensure it follows the format documented in the README.", local_config))?;
            apply_partial(&mut config, parsed);
            break;
        }
        current_dir = dir.parent().map(|p| p.to_path_buf());
    }

    // 3. Runtime Environment Variable Override (highest priority)
    if let Ok(url) = std::env::var("RUSL_API_URL") {
        config.api_base_url = url;
    }
    if let Ok(url) = std::env::var("RUSL_WEBSITE_URL") {
        config.website_url = url;
    }

    Ok(config)
}

/// Apply a partial config overlay — only overwrite fields that were explicitly set.
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

    // 1. Global layer
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "rusl") {
        let global_config = proj_dirs.config_dir().join("config.toml");
        if global_config.exists() {
            let contents = std::fs::read_to_string(&global_config)?;
            let parsed: PartialConfig = toml::from_str(&contents)
                .with_context(|| "Failed to parse global config")?;
            for (name, gcfg) in parsed.generators {
                result.insert(name, EffectiveGenerator { config: gcfg, tier: ConfigTier::Global });
            }
        }
    }

    // 2. Project layer overwrites
    let mut current_dir = std::env::current_dir().ok();
    while let Some(dir) = current_dir.as_ref() {
        let local_config = dir.join("rusl.config.toml");
        if local_config.exists() {
            let contents = std::fs::read_to_string(&local_config)?;
            let parsed: PartialConfig = toml::from_str(&contents)
                .with_context(|| "Failed to parse project config")?;
            for (name, gcfg) in parsed.generators {
                result.insert(name, EffectiveGenerator { config: gcfg, tier: ConfigTier::Project });
            }
            break;
        }
        current_dir = dir.parent().map(|p| p.to_path_buf());
    }

    let mut sorted: Vec<_> = result.into_iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(sorted)
}
