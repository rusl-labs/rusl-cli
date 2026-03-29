use anyhow::Context;
use serde::{Deserialize, Serialize};
use tracing::debug;

pub mod credentials;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    #[serde(alias = "registry_url")]
    pub api_base_url: String,
    pub website_url: String,
}

impl Default for Config {
    fn default() -> Self {
        // Compile-time fallback URLs dynamically routing traffic securely.
        let default_api = option_env!("RUSL_DEFAULT_API_URL").unwrap_or("https://api.rusl.app");
        let default_web = option_env!("RUSL_DEFAULT_WEBSITE_URL").unwrap_or("https://rusl.app");

        Self {
            api_base_url: default_api.to_string(),
            website_url: default_web.to_string(),
        }
    }
}

/// Loads the config strictly via a hierarchical precedence sequence natively
pub fn load() -> anyhow::Result<Config> {
    let mut config = Config::default();

    // 1. Search locally upwards
    let mut current_dir = std::env::current_dir().ok();
    let mut found_toml = false;

    while let Some(dir) = current_dir.as_ref() {
        let local_config = dir.join("rusl.config.toml");
        if local_config.exists() {
            debug!("Found local config at: {:?}", local_config);
            let contents = std::fs::read_to_string(&local_config)?;
            let parsed: Config = toml::from_str(&contents)
                .with_context(|| format!("Configuration Parse Error: File {:?} exists but contains invalid TOML syntax. Please ensure it follows the format documented in the README.", local_config))?;
            config = parsed;
            found_toml = true;
            break;
        }
        // Move up one directory
        current_dir = dir.parent().map(|p| p.to_path_buf());
    }

    // 2. Global Fallback
    if !found_toml {
        if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "rusl") {
            let global_config = proj_dirs.config_dir().join("config.toml");
            if global_config.exists() {
                debug!("Found global config at: {:?}", global_config);
                let contents = std::fs::read_to_string(&global_config)?;
                let parsed: Config = toml::from_str(&contents)
                    .with_context(|| format!("Configuration Parse Error: Global config at {:?} contains strictly invalid TOML syntax.", global_config))?;
                config = parsed;
            }
        }
    }

    // 3. Runtime Environment Variable Override natively (Highest Priority mathematically)
    if let Ok(url) = std::env::var("RUSL_API_URL") {
        config.api_base_url = url;
    }
    if let Ok(url) = std::env::var("RUSL_WEBSITE_URL") {
        config.website_url = url;
    }

    Ok(config)
}
