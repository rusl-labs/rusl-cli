use super::manifest::PackManifest;
use super::paths::{agent_kit_cache_dir, cache_dir, pack_dir_in_kit};
use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use std::process::Command;

pub const DEFAULT_GITHUB_REPO: &str = "rusl-labs/rusl-agent-kit";
pub const DEFAULT_CLONE_URL: &str = "https://github.com/rusl-labs/rusl-agent-kit.git";
pub const PACK_DIR_ENV: &str = "RUSL_SKILLS_PACK";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackSourceKind {
    /// Explicit `--pack-dir` or `RUSL_SKILLS_PACK`.
    Explicit,
    /// Existing `~/.rusl/cache/rusl-agent-kit` (directory or symlink).
    Cache,
    /// Fresh `git clone` into the cache path.
    Cloned,
}

#[derive(Debug, Clone)]
pub struct ResolvedPack {
    /// Absolute path to the pack root (`…/pack`).
    pub pack_root: PathBuf,
    pub source: PackSourceKind,
    /// Kit checkout root when known (cache path); None for bare `--pack-dir`.
    pub kit_root: Option<PathBuf>,
    /// If cache path is a symlink, the resolved target.
    pub symlink_target: Option<PathBuf>,
    pub manifest: PackManifest,
}

#[derive(Debug, Clone, Default)]
pub struct ResolvePackOptions {
    /// Explicit pack directory (CLI `--pack-dir`).
    pub pack_dir: Option<PathBuf>,
    /// Optional git tag to checkout after clone/update (real clones only).
    pub tag: Option<String>,
    /// Override clone URL (tests / enterprise forks).
    pub clone_url: Option<String>,
    /// Skip network clone (tests).
    pub allow_clone: bool,
}

impl ResolvePackOptions {
    pub fn production() -> Self {
        Self {
            pack_dir: None,
            tag: None,
            clone_url: None,
            allow_clone: true,
        }
    }
}

/// Resolve the skills pack root and load its manifest.
///
/// Order:
/// 1. `opts.pack_dir` or `RUSL_SKILLS_PACK`
/// 2. `~/.rusl/cache/rusl-agent-kit` if present (dir or symlink) → `pack/`
/// 3. `git clone` into that cache path (when `allow_clone`)
pub fn resolve_pack(opts: &ResolvePackOptions) -> Result<ResolvedPack> {
    if let Some(pack_root) = explicit_pack_dir(opts)? {
        return load_resolved(pack_root, PackSourceKind::Explicit, None, None);
    }

    let kit_cache = agent_kit_cache_dir()?;
    if path_exists(&kit_cache) {
        let symlink_target = read_symlink_target(&kit_cache);
        let pack_root = pack_dir_in_kit(&kit_cache);
        if !pack_root.join("manifest.toml").is_file() {
            bail!(
                "Agent kit cache exists at {} but pack manifest is missing (expected {}).",
                kit_cache.display(),
                pack_root.join("manifest.toml").display()
            );
        }
        // Do not git-checkout through a symlink to a user working tree.
        if opts.tag.is_some() && symlink_target.is_none() {
            checkout_tag(&kit_cache, opts.tag.as_deref().unwrap())?;
        }
        return load_resolved(
            pack_root,
            PackSourceKind::Cache,
            Some(kit_cache),
            symlink_target,
        );
    }

    if !opts.allow_clone {
        bail!(
            "No skills pack found. Set --pack-dir or {}, or clone {} into {}.",
            PACK_DIR_ENV,
            DEFAULT_GITHUB_REPO,
            kit_cache.display()
        );
    }

    let cache = cache_dir()?;
    std::fs::create_dir_all(&cache)
        .with_context(|| format!("Failed to create cache directory {}", cache.display()))?;

    let url = opts
        .clone_url
        .clone()
        .unwrap_or_else(|| DEFAULT_CLONE_URL.to_string());
    clone_repo(&url, &kit_cache)?;

    if let Some(tag) = &opts.tag {
        checkout_tag(&kit_cache, tag)?;
    }

    let pack_root = pack_dir_in_kit(&kit_cache);
    if !pack_root.join("manifest.toml").is_file() {
        bail!(
            "Cloned {} but pack/manifest.toml is missing at {}.",
            url,
            pack_root.display()
        );
    }

    load_resolved(pack_root, PackSourceKind::Cloned, Some(kit_cache), None)
}

fn explicit_pack_dir(opts: &ResolvePackOptions) -> Result<Option<PathBuf>> {
    if let Some(dir) = &opts.pack_dir {
        let path = canonicalize_existing(dir)?;
        ensure_pack_root(&path)?;
        return Ok(Some(path));
    }
    if let Ok(env_path) = std::env::var(PACK_DIR_ENV)
        && !env_path.trim().is_empty()
    {
        let path = canonicalize_existing(Path::new(env_path.trim()))?;
        ensure_pack_root(&path)?;
        return Ok(Some(path));
    }
    Ok(None)
}

fn ensure_pack_root(path: &Path) -> Result<()> {
    if !path.join("manifest.toml").is_file() {
        bail!(
            "Pack directory {} is missing manifest.toml.",
            path.display()
        );
    }
    Ok(())
}

fn load_resolved(
    pack_root: PathBuf,
    source: PackSourceKind,
    kit_root: Option<PathBuf>,
    symlink_target: Option<PathBuf>,
) -> Result<ResolvedPack> {
    let pack_root = if pack_root.is_absolute() {
        pack_root
    } else {
        std::env::current_dir()?.join(pack_root)
    };
    let manifest = PackManifest::load(&pack_root)?;
    Ok(ResolvedPack {
        pack_root,
        source,
        kit_root,
        symlink_target,
        manifest,
    })
}

fn path_exists(path: &Path) -> bool {
    // symlink_metadata succeeds for broken symlinks; prefer "usable" existence.
    path.exists()
}

fn read_symlink_target(path: &Path) -> Option<PathBuf> {
    let meta = std::fs::symlink_metadata(path).ok()?;
    if !meta.file_type().is_symlink() {
        return None;
    }
    std::fs::read_link(path).ok().and_then(|target| {
        if target.is_absolute() {
            Some(target)
        } else {
            path.parent().map(|p| p.join(target))
        }
    })
}

fn canonicalize_existing(path: &Path) -> Result<PathBuf> {
    path.canonicalize()
        .with_context(|| format!("Pack path does not exist: {}", path.display()))
}

fn clone_repo(url: &str, dest: &Path) -> Result<()> {
    if dest.exists() {
        bail!(
            "Cache path {} already exists; refusing to clone over it.",
            dest.display()
        );
    }
    let status = Command::new("git")
        .args(["clone", "--depth", "1", url])
        .arg(dest)
        .status()
        .with_context(|| format!("Failed to run git clone for {url}"))?;
    if !status.success() {
        bail!("git clone {url} failed with status {status}");
    }
    Ok(())
}

fn checkout_tag(kit_root: &Path, tag: &str) -> Result<()> {
    let fetch = Command::new("git")
        .args(["fetch", "--tags", "--depth", "1", "origin", tag])
        .current_dir(kit_root)
        .status()
        .context("Failed to run git fetch for skills tag")?;
    if !fetch.success() {
        // Shallow clone may not support that fetch form; try plain checkout.
        tracing::debug!("git fetch tags failed; trying local checkout of {tag}");
    }
    let status = Command::new("git")
        .args(["checkout", tag])
        .current_dir(kit_root)
        .status()
        .with_context(|| format!("Failed to checkout tag {tag}"))?;
    if !status.success() {
        bail!("git checkout {tag} failed with status {status}");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::ffi::OsString;
    use std::fs;
    use tempfile::TempDir;

    struct EnvGuard {
        home: Option<OsString>,
        pack_env: Option<OsString>,
    }

    impl EnvGuard {
        fn new(home: &Path) -> Self {
            let home_prev = std::env::var_os("HOME");
            let pack_prev = std::env::var_os(PACK_DIR_ENV);
            unsafe {
                std::env::set_var("HOME", home);
                std::env::remove_var(PACK_DIR_ENV);
            }
            Self {
                home: home_prev,
                pack_env: pack_prev,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            unsafe {
                match &self.home {
                    Some(v) => std::env::set_var("HOME", v),
                    None => std::env::remove_var("HOME"),
                }
                match &self.pack_env {
                    Some(v) => std::env::set_var(PACK_DIR_ENV, v),
                    None => std::env::remove_var(PACK_DIR_ENV),
                }
            }
        }
    }

    fn write_minimal_pack(pack: &Path) {
        fs::create_dir_all(pack.join("skills/init")).unwrap();
        fs::create_dir_all(pack.join("skills/references")).unwrap();
        fs::create_dir_all(pack.join("mcp")).unwrap();
        fs::write(
            pack.join("manifest.toml"),
            r#"
schema_version = 1
version = "0.1.0"
[[skills]]
id = "init"
dir = "skills/init"
install_name = "rusl-init"
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
        fs::write(
            pack.join("skills/init/SKILL.md"),
            "---\nname: rusl-init\n---\n",
        )
        .unwrap();
    }

    #[test]
    #[serial]
    fn resolves_explicit_pack_dir() {
        let tmp = TempDir::new().unwrap();
        let _g = EnvGuard::new(tmp.path());
        let pack = tmp.path().join("my-pack");
        write_minimal_pack(&pack);

        let resolved = resolve_pack(&ResolvePackOptions {
            pack_dir: Some(pack.clone()),
            allow_clone: false,
            ..Default::default()
        })
        .unwrap();

        assert_eq!(resolved.source, PackSourceKind::Explicit);
        assert_eq!(resolved.manifest.version, "0.1.0");
        assert!(resolved.pack_root.ends_with("my-pack"));
    }

    #[test]
    #[serial]
    fn resolves_cache_symlink_to_kit() {
        let tmp = TempDir::new().unwrap();
        let _g = EnvGuard::new(tmp.path());

        let kit = tmp.path().join("kit-checkout");
        let pack = kit.join("pack");
        write_minimal_pack(&pack);

        let cache = tmp.path().join(".rusl/cache");
        fs::create_dir_all(&cache).unwrap();
        std::os::unix::fs::symlink(&kit, cache.join("rusl-agent-kit")).unwrap();

        let resolved = resolve_pack(&ResolvePackOptions {
            allow_clone: false,
            ..Default::default()
        })
        .unwrap();

        assert_eq!(resolved.source, PackSourceKind::Cache);
        assert!(resolved.symlink_target.is_some());
        assert_eq!(resolved.manifest.version, "0.1.0");
    }

    #[test]
    #[serial]
    fn errors_when_missing_without_clone() {
        let tmp = TempDir::new().unwrap();
        let _g = EnvGuard::new(tmp.path());

        let err = resolve_pack(&ResolvePackOptions {
            allow_clone: false,
            ..Default::default()
        })
        .unwrap_err();
        assert!(err.to_string().contains("No skills pack found"));
    }
}
