use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::path::{Path, PathBuf};

/// Parsed `pack/manifest.toml` install contract.
#[derive(Debug, Clone, Deserialize)]
pub struct PackManifest {
    pub schema_version: u32,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub source: PackSource,
    pub skills: Vec<PackSkill>,
    pub skills_shared: PackSkillsShared,
    #[serde(default)]
    pub skills_naming: PackSkillsNaming,
    pub mcp: PackMcp,
    #[serde(default)]
    pub harnesses: PackHarnesses,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PackSource {
    pub github_repo: Option<String>,
    pub repo_path: Option<String>,
    pub tag_prefix: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PackSkill {
    pub id: String,
    pub dir: String,
    pub install_name: String,
    pub short_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PackSkillsShared {
    pub dir: String,
    pub install_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PackSkillsNaming {
    #[serde(default = "default_true")]
    pub rewrite_frontmatter_name: bool,
}

impl Default for PackSkillsNaming {
    fn default() -> Self {
        Self {
            rewrite_frontmatter_name: true,
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize)]
pub struct PackMcp {
    pub file: String,
    pub server_key: String,
    #[serde(default = "default_rusl_command")]
    pub default_command: String,
    #[serde(default = "default_mcp_args")]
    pub default_args: Vec<String>,
}

fn default_rusl_command() -> String {
    "rusl".to_string()
}

fn default_mcp_args() -> Vec<String> {
    vec!["mcp".to_string()]
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct PackHarnesses {
    pub cursor: Option<HarnessPaths>,
    pub claude: Option<HarnessPaths>,
    pub codex: Option<HarnessPaths>,
    pub opencode: Option<HarnessPaths>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HarnessPaths {
    pub skills_global: Option<String>,
    pub skills_project: Option<String>,
    pub mcp_global: Option<String>,
    pub mcp_project: Option<String>,
    pub plugin_hint: Option<String>,
    #[serde(default = "default_true")]
    pub require_hyphen_skill_names: bool,
    #[serde(default = "default_true")]
    pub install_short_name_aliases: bool,
}

impl PackManifest {
    pub fn load(pack_root: &Path) -> Result<Self> {
        let path = pack_root.join("manifest.toml");
        let contents = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read pack manifest at {}", path.display()))?;
        let manifest: Self = toml::from_str(&contents)
            .with_context(|| format!("Invalid pack manifest at {}", path.display()))?;
        if manifest.schema_version != 1 {
            bail!(
                "Unsupported pack schema_version {} (expected 1) in {}",
                manifest.schema_version,
                path.display()
            );
        }
        if manifest.skills.is_empty() {
            bail!(
                "Pack manifest has no [[skills]] entries: {}",
                path.display()
            );
        }
        Ok(manifest)
    }

    pub fn skill_source_dir(&self, pack_root: &Path, skill: &PackSkill) -> PathBuf {
        pack_root.join(&skill.dir)
    }

    pub fn shared_source_dir(&self, pack_root: &Path) -> PathBuf {
        pack_root.join(&self.skills_shared.dir)
    }

    pub fn mcp_fragment_path(&self, pack_root: &Path) -> PathBuf {
        pack_root.join(&self.mcp.file)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn loads_agent_kit_style_manifest() {
        let tmp = TempDir::new().unwrap();
        let pack = tmp.path().join("pack");
        fs::create_dir_all(pack.join("skills/init")).unwrap();
        fs::write(
            pack.join("manifest.toml"),
            r#"
schema_version = 1
version = "0.1.0"

[[skills]]
id = "init"
dir = "skills/init"
install_name = "rusl-init"
short_name = "init"

[skills_shared]
dir = "skills/references"
install_name = "references"

[mcp]
file = "mcp/default.json"
server_key = "rusl"
"#,
        )
        .unwrap();

        let m = PackManifest::load(&pack).unwrap();
        assert_eq!(m.version, "0.1.0");
        assert_eq!(m.skills.len(), 1);
        assert_eq!(m.skills[0].install_name, "rusl-init");
        assert!(m.skills_naming.rewrite_frontmatter_name);
        assert_eq!(m.mcp.server_key, "rusl");
        assert_eq!(m.mcp.default_args, vec!["mcp"]);
    }
}
