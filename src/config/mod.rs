use anyhow::Context;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub registry_url: String,
}

impl Default for Config {
    fn default() -> Self {
        // 4. Compile-time fallback URL (Overrideable via `RUSL_DEFAULT_REGISTRY` during `cargo build`)
        let compile_time_url =
            option_env!("RUSL_DEFAULT_REGISTRY").unwrap_or("https://registry.rusl.dev");
        Self {
            registry_url: compile_time_url.to_string(),
        }
    }
}

/// Loads the config strictly via a hierarchical precedence sequence
pub fn load() -> anyhow::Result<Config> {
    // 1. Runtime Environment Variable Override (Highest Priority)
    if let Ok(url) = std::env::var("RUSL_REGISTRY_URL") {
        debug!("Found runtime environment override: RUSL_REGISTRY_URL");
        return Ok(Config { registry_url: url });
    }

    // 2. Search locally upwards
    let mut current_dir = std::env::current_dir().ok();

    while let Some(dir) = current_dir.as_ref() {
        let local_config = dir.join("rusl.config.toml");
        if local_config.exists() {
            debug!("Found local config at: {:?}", local_config);
            let contents = std::fs::read_to_string(&local_config)?;
            let config: Config = toml::from_str(&contents)
                .with_context(|| format!("Failed to parse config at {:?}", local_config))?;
            return Ok(config);
        }
        // Move up one directory
        current_dir = dir.parent().map(|p| p.to_path_buf());
    }

    // 2. Global Fallback
    if let Some(proj_dirs) = directories::ProjectDirs::from("", "", "rusl") {
        let global_config = proj_dirs.config_dir().join("config.toml");
        if global_config.exists() {
            debug!("Found global config at: {:?}", global_config);
            let contents = std::fs::read_to_string(&global_config)?;
            let config: Config = toml::from_str(&contents)
                .with_context(|| format!("Failed to parse config at {:?}", global_config))?;
            return Ok(config);
        }
    }

    // 3. Absolute Default
    debug!("No configuration file found. Using default.");
    Ok(Config::default())
}
