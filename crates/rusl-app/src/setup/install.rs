use super::manifest::{PackManifest, PackSkill};
use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct SkillInstallResult {
    pub install_name: String,
    pub short_name: Option<String>,
    pub dest: PathBuf,
    pub action: InstallAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallAction {
    Copied,
    SkippedExisting,
    Replaced,
}

#[derive(Debug, Clone)]
pub struct SkillsInstallReport {
    pub skills: Vec<SkillInstallResult>,
    pub references: Option<SkillInstallResult>,
}

/// Copy pack skills into a harness skills directory.
///
/// Follows pack skill symlinks so destinations are real trees (not links into the kit).
pub fn install_skills(
    pack_root: &Path,
    manifest: &PackManifest,
    skills_root: &Path,
    force: bool,
    short_name_aliases: bool,
) -> Result<SkillsInstallReport> {
    std::fs::create_dir_all(skills_root).with_context(|| {
        format!(
            "Failed to create skills directory {}",
            skills_root.display()
        )
    })?;

    let mut skills = Vec::new();
    for skill in &manifest.skills {
        skills.push(install_one_skill(
            pack_root,
            manifest,
            skill,
            skills_root,
            force,
            short_name_aliases,
        )?);
    }

    let references = install_shared_references(pack_root, manifest, skills_root, force)?;

    Ok(SkillsInstallReport {
        skills,
        references: Some(references),
    })
}

fn install_one_skill(
    pack_root: &Path,
    manifest: &PackManifest,
    skill: &PackSkill,
    skills_root: &Path,
    force: bool,
    short_name_aliases: bool,
) -> Result<SkillInstallResult> {
    let src = resolve_source_tree(&manifest.skill_source_dir(pack_root, skill))?;
    let dest = skills_root.join(&skill.install_name);

    let action = copy_tree_to_dest(&src, &dest, force)?;

    if action != InstallAction::SkippedExisting && manifest.skills_naming.rewrite_frontmatter_name {
        rewrite_skill_frontmatter_name(&dest.join("SKILL.md"), &skill.install_name)?;
    }

    if short_name_aliases && let Some(short) = &skill.short_name {
        install_short_alias(skills_root, short, &skill.install_name)?;
    }

    Ok(SkillInstallResult {
        install_name: skill.install_name.clone(),
        short_name: skill.short_name.clone(),
        dest,
        action,
    })
}

fn install_shared_references(
    pack_root: &Path,
    manifest: &PackManifest,
    skills_root: &Path,
    force: bool,
) -> Result<SkillInstallResult> {
    let src = resolve_source_tree(&manifest.shared_source_dir(pack_root))?;
    let dest = skills_root.join(&manifest.skills_shared.install_name);
    let action = copy_tree_to_dest(&src, &dest, force)?;
    Ok(SkillInstallResult {
        install_name: manifest.skills_shared.install_name.clone(),
        short_name: None,
        dest,
        action,
    })
}

fn copy_tree_to_dest(src: &Path, dest: &Path, force: bool) -> Result<InstallAction> {
    if dest.exists() || dest.symlink_metadata().is_ok() {
        if !force {
            return Ok(InstallAction::SkippedExisting);
        }
        remove_path(dest)?;
        copy_dir_all(src, dest)?;
        return Ok(InstallAction::Replaced);
    }
    copy_dir_all(src, dest)?;
    Ok(InstallAction::Copied)
}

fn install_short_alias(skills_root: &Path, short_name: &str, install_name: &str) -> Result<()> {
    let short_path = skills_root.join(short_name);
    if short_path.exists() || short_path.symlink_metadata().is_ok() {
        remove_path(&short_path)?;
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(install_name, &short_path).with_context(|| {
            format!(
                "Failed to create short-name alias {} -> {}",
                short_path.display(),
                install_name
            )
        })?;
    }
    #[cfg(not(unix))]
    {
        // On Windows, fall back to a second copy (junctions need elevated rights).
        let src = skills_root.join(install_name);
        copy_dir_all(&src, &short_path)?;
    }
    Ok(())
}

/// Resolve a pack skill path to a physical directory (follow one level of symlink).
pub fn resolve_source_tree(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        bail!("Pack skill path does not exist: {}", path.display());
    }
    let meta = std::fs::symlink_metadata(path)
        .with_context(|| format!("Failed to stat {}", path.display()))?;
    if meta.file_type().is_symlink() {
        return path
            .canonicalize()
            .with_context(|| format!("Failed to resolve symlink {}", path.display()));
    }
    Ok(path.to_path_buf())
}

fn remove_path(path: &Path) -> Result<()> {
    let meta = std::fs::symlink_metadata(path)
        .with_context(|| format!("Failed to inspect {}", path.display()))?;
    if meta.file_type().is_dir() && !meta.file_type().is_symlink() {
        std::fs::remove_dir_all(path)
            .with_context(|| format!("Failed to remove directory {}", path.display()))?;
    } else {
        std::fs::remove_file(path)
            .or_else(|_| std::fs::remove_dir_all(path))
            .with_context(|| format!("Failed to remove {}", path.display()))?;
    }
    Ok(())
}

fn copy_dir_all(src: &Path, dest: &Path) -> Result<()> {
    std::fs::create_dir_all(dest)
        .with_context(|| format!("Failed to create {}", dest.display()))?;
    for entry in std::fs::read_dir(src)
        .with_context(|| format!("Failed to read directory {}", src.display()))?
    {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let from = entry.path();
        let to = dest.join(entry.file_name());
        if file_type.is_dir() {
            copy_dir_all(&from, &to)?;
        } else if file_type.is_symlink() {
            // Materialize symlink targets as real files/dirs in the destination.
            let target = std::fs::read_link(&from)?;
            let resolved = if target.is_absolute() {
                target
            } else {
                from.parent().unwrap_or(src).join(target)
            };
            if resolved.is_dir() {
                copy_dir_all(&resolved, &to)?;
            } else {
                std::fs::copy(&resolved, &to).with_context(|| {
                    format!("Failed to copy {} -> {}", resolved.display(), to.display())
                })?;
            }
        } else {
            std::fs::copy(&from, &to).with_context(|| {
                format!("Failed to copy {} -> {}", from.display(), to.display())
            })?;
        }
    }
    Ok(())
}

/// Ensure SKILL.md frontmatter `name:` matches the install name.
pub fn rewrite_skill_frontmatter_name(skill_md: &Path, install_name: &str) -> Result<()> {
    if !skill_md.is_file() {
        return Ok(());
    }
    let contents = std::fs::read_to_string(skill_md)
        .with_context(|| format!("Failed to read {}", skill_md.display()))?;
    let rewritten = rewrite_frontmatter_name(&contents, install_name);
    if rewritten != contents {
        std::fs::write(skill_md, rewritten)
            .with_context(|| format!("Failed to write {}", skill_md.display()))?;
    }
    Ok(())
}

fn rewrite_frontmatter_name(contents: &str, install_name: &str) -> String {
    let mut lines = contents.lines().peekable();
    let mut out = String::new();
    let Some(first) = lines.next() else {
        return contents.to_string();
    };
    if first.trim() != "---" {
        return contents.to_string();
    }
    out.push_str(first);
    out.push('\n');

    let mut in_frontmatter = true;
    let mut replaced = false;
    for line in lines {
        if in_frontmatter {
            if line.trim() == "---" {
                in_frontmatter = false;
                out.push_str(line);
                out.push('\n');
                continue;
            }
            if !replaced && line.starts_with("name:") {
                out.push_str("name: ");
                out.push_str(install_name);
                out.push('\n');
                replaced = true;
                continue;
            }
        }
        out.push_str(line);
        out.push('\n');
    }

    // Preserve lack of trailing newline only if original had none and we didn't change much.
    if !contents.ends_with('\n') && out.ends_with('\n') {
        out.pop();
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::manifest::PackManifest;
    use std::fs;
    use tempfile::TempDir;

    fn sample_pack(root: &Path) -> PathBuf {
        let pack = root.join("pack");
        let plugin_skill = root.join("plugins/rusl/skills/init");
        fs::create_dir_all(&plugin_skill).unwrap();
        fs::write(
            plugin_skill.join("SKILL.md"),
            "---\nname: rusl-init\ndescription: test\n---\n\n# Init\n",
        )
        .unwrap();
        fs::create_dir_all(plugin_skill.parent().unwrap().join("references")).unwrap();
        fs::write(
            plugin_skill.parent().unwrap().join("references/harness.md"),
            "# harness\n",
        )
        .unwrap();

        fs::create_dir_all(pack.join("skills")).unwrap();
        fs::create_dir_all(pack.join("mcp")).unwrap();
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink("../../plugins/rusl/skills/init", pack.join("skills/init"))
                .unwrap();
            std::os::unix::fs::symlink(
                "../../plugins/rusl/skills/references",
                pack.join("skills/references"),
            )
            .unwrap();
        }
        #[cfg(not(unix))]
        {
            copy_dir_all(&plugin_skill, &pack.join("skills/init")).unwrap();
            copy_dir_all(
                &plugin_skill.parent().unwrap().join("references"),
                &pack.join("skills/references"),
            )
            .unwrap();
        }

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
        fs::write(
            pack.join("mcp/default.json"),
            r#"{"command":"rusl","args":["mcp"]}"#,
        )
        .unwrap();
        pack
    }

    #[test]
    fn rewrite_frontmatter_replaces_name() {
        let input = "---\nname: rusl:init\ndescription: x\n---\n\nBody\n";
        let out = rewrite_frontmatter_name(input, "rusl-init");
        assert!(out.contains("name: rusl-init\n"));
        assert!(!out.contains("rusl:init"));
        assert!(out.contains("Body"));
    }

    #[test]
    fn install_follows_symlinks_and_creates_aliases() {
        let tmp = TempDir::new().unwrap();
        let pack = sample_pack(tmp.path());
        let manifest = PackManifest::load(&pack).unwrap();
        let dest = tmp.path().join("cursor/skills");

        let report = install_skills(&pack, &manifest, &dest, true, true).unwrap();
        assert_eq!(report.skills.len(), 1);
        assert_eq!(report.skills[0].action, InstallAction::Copied);

        let skill_md = dest.join("rusl-init/SKILL.md");
        assert!(skill_md.is_file());
        assert!(
            !dest
                .join("rusl-init")
                .symlink_metadata()
                .unwrap()
                .file_type()
                .is_symlink()
        );

        let alias = dest.join("init");
        assert!(alias.symlink_metadata().unwrap().file_type().is_symlink());
        assert!(dest.join("references/harness.md").is_file());
    }
}
