use super::harness::HarnessId;
use super::paths::agent_skills_lock_path;
use super::resolve::PackSourceKind;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSkillsLock {
    pub version: String,
    pub source: String,
    pub pack_root: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kit_root: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symlink_target: Option<String>,
    pub installed_at: String,
    pub harnesses: Vec<String>,
    pub scope: String,
}

impl AgentSkillsLock {
    pub fn from_install(
        version: &str,
        source: PackSourceKind,
        pack_root: &std::path::Path,
        kit_root: Option<&std::path::Path>,
        symlink_target: Option<&std::path::Path>,
        harnesses: &[HarnessId],
        scope: &str,
    ) -> Self {
        let source = match source {
            PackSourceKind::Explicit => "local",
            PackSourceKind::Cache if symlink_target.is_some() => "local",
            PackSourceKind::Cache => "cache",
            PackSourceKind::Cloned => "github",
        };
        Self {
            version: version.to_string(),
            source: source.to_string(),
            pack_root: pack_root.display().to_string(),
            kit_root: kit_root.map(|p| p.display().to_string()),
            symlink_target: symlink_target.map(|p| p.display().to_string()),
            installed_at: chrono_like_now(),
            harnesses: harnesses.iter().map(|h| h.as_str().to_string()).collect(),
            scope: scope.to_string(),
        }
    }

    pub fn load() -> Result<Option<Self>> {
        let path = agent_skills_lock_path()?;
        if !path.is_file() {
            return Ok(None);
        }
        let text = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let lock: Self = toml::from_str(&text)
            .with_context(|| format!("Invalid agent skills lock at {}", path.display()))?;
        Ok(Some(lock))
    }

    pub fn save(&self) -> Result<PathBuf> {
        let path = agent_skills_lock_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create {}", parent.display()))?;
        }
        let text = toml::to_string_pretty(self).context("Failed to serialize agent skills lock")?;
        std::fs::write(&path, text)
            .with_context(|| format!("Failed to write {}", path.display()))?;
        Ok(path)
    }
}

fn chrono_like_now() -> String {
    // Avoid pulling chrono just for an ISO-ish stamp; RFC3339-ish local isn't critical.
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("unix:{secs}")
}
