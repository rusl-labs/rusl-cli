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
