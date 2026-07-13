use super::cursor_sandbox::{CursorSandboxMergeResult, merge_cursor_sandbox};
use super::install::{SkillsInstallReport, install_skills};
use super::manifest::{HarnessPaths, PackManifest};
use super::mcp::{
    McpFormat, McpMergeResult, command_and_args, load_mcp_fragment, mcp_entry_present, merge_mcp,
    resolve_rusl_command,
};
use super::resolve::ResolvedPack;
use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HarnessId {
    Cursor,
    Claude,
    Codex,
    OpenCode,
    Grok,
}

impl HarnessId {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cursor => "cursor",
            Self::Claude => "claude",
            Self::Codex => "codex",
            Self::OpenCode => "opencode",
            Self::Grok => "grok",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().as_str() {
            "cursor" => Some(Self::Cursor),
            "claude" => Some(Self::Claude),
            "codex" => Some(Self::Codex),
            "opencode" => Some(Self::OpenCode),
            "grok" => Some(Self::Grok),
            _ => None,
        }
    }

    pub fn all() -> &'static [Self] {
        &[
            Self::Cursor,
            Self::Claude,
            Self::Codex,
            Self::OpenCode,
            Self::Grok,
        ]
    }

    /// Harnesses installed by default when none are detected.
    pub fn default_install_set() -> &'static [Self] {
        &[Self::Cursor, Self::Claude]
    }

    pub fn adapter(self) -> &'static HarnessAdapter {
        match self {
            Self::Cursor => &CURSOR,
            Self::Claude => &CLAUDE,
            Self::Codex => &CODEX,
            Self::OpenCode => &OPENCODE,
            Self::Grok => &GROK,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SetupScope {
    Global,
    /// Default: install into the current project directory.
    #[default]
    Project,
}

/// Built-in per-agent install contract (skills paths + MCP format).
///
/// Pack `manifest.toml` `[harnesses.*]` can override paths; MCP format stays native.
#[derive(Debug, Clone)]
pub struct HarnessAdapter {
    pub id: HarnessId,
    /// Home-relative dirs that mean "this harness is installed".
    pub detect_home: &'static [&'static str],
    /// Binaries on PATH that mean "this harness is installed".
    pub detect_bins: &'static [&'static str],
    /// Project-relative markers for detection in project scope.
    pub detect_project: &'static [&'static str],
    pub skills_global: &'static str,
    pub skills_project: &'static str,
    pub mcp_global: Option<&'static str>,
    pub mcp_project: Option<&'static str>,
    pub mcp_format: Option<McpFormat>,
    /// Extra one-liner printed after install (e.g. Claude plugin hooks).
    pub plugin_hint: Option<&'static str>,
    /// Shown when MCP is intentionally skipped for a scope.
    pub mcp_skip_global: Option<&'static str>,
    pub mcp_skip_project: Option<&'static str>,
    pub install_short_name_aliases: bool,
}

static CURSOR: HarnessAdapter = HarnessAdapter {
    id: HarnessId::Cursor,
    detect_home: &[".cursor"],
    detect_bins: &["cursor"],
    detect_project: &[".cursor"],
    skills_global: "~/.cursor/skills",
    skills_project: ".cursor/skills",
    mcp_global: Some("~/.cursor/mcp.json"),
    mcp_project: Some(".cursor/mcp.json"),
    mcp_format: Some(McpFormat::McpServersJson),
    plugin_hint: None,
    mcp_skip_global: None,
    mcp_skip_project: None,
    install_short_name_aliases: true,
};

static CLAUDE: HarnessAdapter = HarnessAdapter {
    id: HarnessId::Claude,
    detect_home: &[".claude"],
    detect_bins: &["claude"],
    detect_project: &[".claude"],
    skills_global: "~/.claude/skills",
    skills_project: ".claude/skills",
    // Project MCP: Claude Code reads `.mcp.json` at the project root.
    // Global MCP is handled by the plugin marketplace install.
    mcp_global: None,
    mcp_project: Some(".mcp.json"),
    mcp_format: Some(McpFormat::McpServersJson),
    plugin_hint: Some("/plugin marketplace add rusl-labs/rusl-agent-kit && /plugin install rusl"),
    mcp_skip_global: Some(
        "global MCP comes from the Claude plugin; use the marketplace install for hooks + MCP",
    ),
    mcp_skip_project: None,
    install_short_name_aliases: true,
};

static CODEX: HarnessAdapter = HarnessAdapter {
    id: HarnessId::Codex,
    detect_home: &[".codex"],
    detect_bins: &["codex"],
    detect_project: &[".codex", ".agents"],
    skills_global: "~/.codex/skills",
    // Shared multi-agent project skills root (Codex + others scan `.agents/skills`).
    skills_project: ".agents/skills",
    mcp_global: Some("~/.codex/config.toml"),
    mcp_project: None,
    mcp_format: Some(McpFormat::McpServersToml),
    plugin_hint: None,
    mcp_skip_global: None,
    mcp_skip_project: Some(
        "Codex MCP is user-global (~/.codex/config.toml); skills install into .agents/skills",
    ),
    install_short_name_aliases: true,
};

static OPENCODE: HarnessAdapter = HarnessAdapter {
    id: HarnessId::OpenCode,
    detect_home: &[".config/opencode"],
    detect_bins: &["opencode"],
    detect_project: &[".opencode", "opencode.json", "opencode.jsonc"],
    skills_global: "~/.config/opencode/skills",
    skills_project: ".opencode/skills",
    mcp_global: Some("~/.config/opencode/opencode.json"),
    mcp_project: Some("opencode.json"),
    mcp_format: Some(McpFormat::OpenCodeJson),
    plugin_hint: None,
    mcp_skip_global: None,
    mcp_skip_project: None,
    install_short_name_aliases: true,
};

static GROK: HarnessAdapter = HarnessAdapter {
    id: HarnessId::Grok,
    detect_home: &[".grok"],
    detect_bins: &["grok"],
    detect_project: &[".grok"],
    skills_global: "~/.grok/skills",
    skills_project: ".grok/skills",
    mcp_global: Some("~/.grok/config.toml"),
    mcp_project: Some(".grok/config.toml"),
    mcp_format: Some(McpFormat::McpServersToml),
    plugin_hint: None,
    mcp_skip_global: None,
    mcp_skip_project: None,
    install_short_name_aliases: true,
};

#[derive(Debug, Clone)]
pub struct HarnessInstallReport {
    pub harness: HarnessId,
    pub scope: SetupScope,
    pub skills_root: PathBuf,
    pub skills: Option<SkillsInstallReport>,
    pub mcp: Option<McpMergeResult>,
    pub mcp_note: Option<String>,
    /// Cursor-only: merge result for `~/.cursor/sandbox.json`.
    pub sandbox: Option<CursorSandboxMergeResult>,
    pub plugin_hint: Option<String>,
}

#[derive(Debug, Clone)]
pub struct InstallOptions {
    pub scope: SetupScope,
    pub project_dir: PathBuf,
    pub skills: bool,
    pub mcp: bool,
    pub force: bool,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            scope: SetupScope::Project,
            project_dir: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            skills: true,
            mcp: true,
            force: false,
        }
    }
}

pub fn detect_harnesses() -> Vec<HarnessId> {
    detect_harnesses_in(None)
}

/// Detect harnesses from home install markers, PATH binaries, and optional project markers.
pub fn detect_harnesses_in(project_dir: Option<&Path>) -> Vec<HarnessId> {
    let mut found = Vec::new();
    for id in HarnessId::all() {
        if id.adapter().is_detected(project_dir) {
            found.push(*id);
        }
    }
    found
}

impl HarnessAdapter {
    fn is_detected(&self, project_dir: Option<&Path>) -> bool {
        if let Ok(home) = std::env::var("HOME") {
            let home = Path::new(&home);
            for rel in self.detect_home {
                if home.join(rel).exists() {
                    return true;
                }
            }
        }
        for bin in self.detect_bins {
            if which_on_path(bin) {
                return true;
            }
        }
        if let Some(project) = project_dir {
            for rel in self.detect_project {
                if project.join(rel).exists() {
                    return true;
                }
            }
        }
        false
    }

    fn skills_path(&self, scope: SetupScope, project_dir: &Path) -> Result<PathBuf> {
        match scope {
            SetupScope::Global => expand_user_path(self.skills_global),
            SetupScope::Project => Ok(project_dir.join(self.skills_project)),
        }
    }

    fn mcp_path(&self, scope: SetupScope, project_dir: &Path) -> Result<Option<PathBuf>> {
        let configured = match scope {
            SetupScope::Global => self.mcp_global,
            SetupScope::Project => self.mcp_project,
        };
        match configured {
            Some(p) if scope == SetupScope::Global => Ok(Some(expand_user_path(p)?)),
            Some(p) => Ok(Some(project_dir.join(p))),
            None => Ok(None),
        }
    }

    pub fn mcp_skip_note(&self, scope: SetupScope) -> Option<&'static str> {
        match scope {
            SetupScope::Global => self.mcp_skip_global,
            SetupScope::Project => self.mcp_skip_project,
        }
    }
}

fn which_on_path(binary: &str) -> bool {
    std::env::var_os("PATH")
        .map(|paths| {
            std::env::split_paths(&paths).any(|dir| {
                let candidate = dir.join(binary);
                candidate.is_file()
            })
        })
        .unwrap_or(false)
}

pub fn install_harness(
    harness: HarnessId,
    pack: &ResolvedPack,
    opts: &InstallOptions,
) -> Result<HarnessInstallReport> {
    let adapter = harness.adapter();
    let pack_paths = harness_paths_from_manifest(harness, &pack.manifest);

    let skills_root = resolve_skills_root(adapter, &pack_paths, opts)?;
    let short_aliases = pack_paths
        .as_ref()
        .map(|p| p.install_short_name_aliases)
        .unwrap_or(adapter.install_short_name_aliases);

    let skills = if opts.skills {
        Some(install_skills(
            &pack.pack_root,
            &pack.manifest,
            &skills_root,
            opts.force,
            short_aliases,
        )?)
    } else {
        None
    };

    let (mcp, mcp_note) = if opts.mcp {
        install_mcp(adapter, &pack_paths, pack, opts)?
    } else {
        (None, None)
    };

    // Cursor sandbox is user-global (credentials live outside the workspace).
    // Always merge when setting up Cursor, regardless of project vs global skills scope.
    let sandbox = if harness == HarnessId::Cursor {
        Some(merge_cursor_sandbox()?)
    } else {
        None
    };

    let plugin_hint = pack_paths
        .as_ref()
        .and_then(|p| p.plugin_hint.clone())
        .or_else(|| adapter.plugin_hint.map(str::to_string));

    Ok(HarnessInstallReport {
        harness,
        scope: opts.scope,
        skills_root,
        skills,
        mcp,
        mcp_note,
        sandbox,
        plugin_hint,
    })
}

fn install_mcp(
    adapter: &HarnessAdapter,
    pack_paths: &Option<HarnessPaths>,
    pack: &ResolvedPack,
    opts: &InstallOptions,
) -> Result<(Option<McpMergeResult>, Option<String>)> {
    let path = resolve_mcp_path(adapter, pack_paths, opts)?;
    let Some(mcp_path) = path else {
        let note = adapter.mcp_skip_note(opts.scope).map(str::to_string);
        return Ok((None, note));
    };

    let Some(format) = adapter.mcp_format else {
        return Ok((
            None,
            Some(format!(
                "{} has no MCP merge format configured",
                adapter.id.as_str()
            )),
        ));
    };

    // OpenCode may already use opencode.jsonc globally — prefer existing file name.
    let mcp_path = prefer_existing_opencode_config(adapter, mcp_path, opts)?;

    let fragment_path = pack.manifest.mcp_fragment_path(&pack.pack_root);
    let fragment = load_mcp_fragment(&fragment_path)?;
    let command = resolve_rusl_command();
    let (command, args) = command_and_args(&fragment, Some(&command))?;
    let server_key = pack.manifest.mcp.server_key.as_str();

    let result = merge_mcp(&mcp_path, format, server_key, &command, &args)?;
    Ok((Some(result), None))
}

/// Prefer `opencode.jsonc` when it already exists (global or project).
fn prefer_existing_opencode_config(
    adapter: &HarnessAdapter,
    mcp_path: PathBuf,
    opts: &InstallOptions,
) -> Result<PathBuf> {
    if adapter.id != HarnessId::OpenCode {
        return Ok(mcp_path);
    }
    let candidates = match opts.scope {
        SetupScope::Global => {
            let home = home_dir()?;
            vec![
                home.join(".config/opencode/opencode.jsonc"),
                home.join(".config/opencode/opencode.json"),
            ]
        }
        SetupScope::Project => vec![
            opts.project_dir.join("opencode.jsonc"),
            opts.project_dir.join("opencode.json"),
        ],
    };
    for c in &candidates {
        if c.is_file() {
            return Ok(c.clone());
        }
    }
    Ok(mcp_path)
}

fn harness_paths_from_manifest(
    harness: HarnessId,
    manifest: &PackManifest,
) -> Option<HarnessPaths> {
    match harness {
        HarnessId::Cursor => manifest.harnesses.cursor.clone(),
        HarnessId::Claude => manifest.harnesses.claude.clone(),
        HarnessId::Codex => manifest.harnesses.codex.clone(),
        HarnessId::OpenCode => manifest.harnesses.opencode.clone(),
        HarnessId::Grok => None, // pack may not list grok yet; use built-in adapter
    }
}

fn resolve_skills_root(
    adapter: &HarnessAdapter,
    pack_paths: &Option<HarnessPaths>,
    opts: &InstallOptions,
) -> Result<PathBuf> {
    match opts.scope {
        SetupScope::Global => {
            if let Some(p) = pack_paths.as_ref().and_then(|h| h.skills_global.as_ref()) {
                return expand_user_path(p);
            }
            adapter.skills_path(SetupScope::Global, &opts.project_dir)
        }
        SetupScope::Project => {
            if let Some(p) = pack_paths.as_ref().and_then(|h| h.skills_project.as_ref()) {
                return Ok(opts.project_dir.join(p));
            }
            adapter.skills_path(SetupScope::Project, &opts.project_dir)
        }
    }
}

fn resolve_mcp_path(
    adapter: &HarnessAdapter,
    pack_paths: &Option<HarnessPaths>,
    opts: &InstallOptions,
) -> Result<Option<PathBuf>> {
    match opts.scope {
        SetupScope::Global => {
            if let Some(p) = pack_paths.as_ref().and_then(|h| h.mcp_global.as_ref()) {
                return Ok(Some(expand_user_path(p)?));
            }
            adapter.mcp_path(SetupScope::Global, &opts.project_dir)
        }
        SetupScope::Project => {
            if let Some(p) = pack_paths.as_ref().and_then(|h| h.mcp_project.as_ref()) {
                return Ok(Some(opts.project_dir.join(p)));
            }
            adapter.mcp_path(SetupScope::Project, &opts.project_dir)
        }
    }
}

fn home_dir() -> Result<PathBuf> {
    std::env::var("HOME")
        .map(PathBuf::from)
        .context("HOME is required for global harness paths")
}

fn expand_user_path(path: &str) -> Result<PathBuf> {
    if let Some(rest) = path.strip_prefix("~/") {
        return Ok(home_dir()?.join(rest));
    }
    if path == "~" {
        return home_dir();
    }
    Ok(PathBuf::from(path))
}

pub fn parse_targets(targets: &[String], all: bool) -> Result<Vec<HarnessId>> {
    parse_targets_in(targets, all, None)
}

pub fn parse_targets_in(
    targets: &[String],
    all: bool,
    project_dir: Option<&Path>,
) -> Result<Vec<HarnessId>> {
    if all {
        return Ok(HarnessId::all().to_vec());
    }
    if targets.is_empty() {
        let detected = detect_harnesses_in(project_dir);
        if detected.is_empty() {
            return Ok(HarnessId::default_install_set().to_vec());
        }
        return Ok(detected);
    }
    let mut out = Vec::new();
    for t in targets {
        match HarnessId::parse(t) {
            Some(h) => {
                if !out.contains(&h) {
                    out.push(h);
                }
            }
            None => bail!(
                "Unknown setup target '{t}'. Expected one of: cursor, claude, codex, opencode, grok."
            ),
        }
    }
    Ok(out)
}

/// Skills root for doctor checks (adapter defaults; pack overrides not re-applied).
pub fn doctor_skills_root(
    harness: HarnessId,
    scope: SetupScope,
    project_dir: &Path,
) -> Result<PathBuf> {
    harness.adapter().skills_path(scope, project_dir)
}

/// MCP path + format for doctor checks.
pub fn doctor_mcp(
    harness: HarnessId,
    scope: SetupScope,
    project_dir: &Path,
    server_key: &str,
) -> Result<Option<(PathBuf, McpFormat, bool)>> {
    let adapter = harness.adapter();
    let Some(format) = adapter.mcp_format else {
        return Ok(None);
    };
    let Some(path) = adapter.mcp_path(scope, project_dir)? else {
        return Ok(None);
    };
    let path = prefer_existing_opencode_config(
        adapter,
        path,
        &InstallOptions {
            scope,
            project_dir: project_dir.to_path_buf(),
            ..InstallOptions::default()
        },
    )?;
    let present = mcp_entry_present(&path, format, server_key);
    Ok(Some((path, format, present)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::setup::manifest::PackManifest;
    use crate::setup::resolve::{PackSourceKind, ResolvedPack};
    use serial_test::serial;
    use std::ffi::OsString;
    use std::fs;
    use tempfile::TempDir;

    struct HomeGuard {
        previous: Option<OsString>,
    }

    impl HomeGuard {
        fn set(home: &Path) -> Self {
            let previous = std::env::var_os("HOME");
            unsafe { std::env::set_var("HOME", home) };
            Self { previous }
        }
    }

    impl Drop for HomeGuard {
        fn drop(&mut self) {
            match &self.previous {
                Some(v) => unsafe { std::env::set_var("HOME", v) },
                None => unsafe { std::env::remove_var("HOME") },
            }
        }
    }

    fn minimal_pack(root: &Path) -> ResolvedPack {
        let pack = root.join("pack");
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
        fs::write(
            pack.join("skills/init/SKILL.md"),
            "---\nname: rusl-init\n---\n",
        )
        .unwrap();
        fs::write(pack.join("skills/references/harness.md"), "# h\n").unwrap();
        let manifest = PackManifest::load(&pack).unwrap();
        ResolvedPack {
            pack_root: pack,
            source: PackSourceKind::Explicit,
            kit_root: None,
            symlink_target: None,
            manifest,
        }
    }

    #[test]
    #[serial]
    fn cursor_project_and_global_paths() {
        let tmp = TempDir::new().unwrap();
        let home = tmp.path().join("home");
        let project = tmp.path().join("project");
        fs::create_dir_all(&home).unwrap();
        fs::create_dir_all(&project).unwrap();
        let _g = HomeGuard::set(&home);
        let pack = minimal_pack(tmp.path());

        let project_report = install_harness(
            HarnessId::Cursor,
            &pack,
            &InstallOptions {
                scope: SetupScope::Project,
                project_dir: project.clone(),
                skills: true,
                mcp: true,
                force: true,
            },
        )
        .unwrap();
        assert_eq!(project_report.skills_root, project.join(".cursor/skills"));
        assert!(project.join(".cursor/mcp.json").is_file());

        let global_report = install_harness(
            HarnessId::Cursor,
            &pack,
            &InstallOptions {
                scope: SetupScope::Global,
                project_dir: project.clone(),
                skills: true,
                mcp: true,
                force: true,
            },
        )
        .unwrap();
        assert_eq!(global_report.skills_root, home.join(".cursor/skills"));
        assert!(home.join(".cursor/mcp.json").is_file());
    }

    #[test]
    #[serial]
    fn claude_project_writes_root_mcp_json_global_skips_mcp() {
        let tmp = TempDir::new().unwrap();
        let home = tmp.path().join("home");
        let project = tmp.path().join("project");
        fs::create_dir_all(&home).unwrap();
        fs::create_dir_all(&project).unwrap();
        let _g = HomeGuard::set(&home);
        let pack = minimal_pack(tmp.path());

        let project_report = install_harness(
            HarnessId::Claude,
            &pack,
            &InstallOptions {
                scope: SetupScope::Project,
                project_dir: project.clone(),
                skills: true,
                mcp: true,
                force: true,
            },
        )
        .unwrap();
        assert_eq!(project_report.skills_root, project.join(".claude/skills"));
        assert!(project.join(".mcp.json").is_file());
        assert!(project_report.mcp.is_some());

        let global_report = install_harness(
            HarnessId::Claude,
            &pack,
            &InstallOptions {
                scope: SetupScope::Global,
                project_dir: project.clone(),
                skills: true,
                mcp: true,
                force: true,
            },
        )
        .unwrap();
        assert_eq!(global_report.skills_root, home.join(".claude/skills"));
        assert!(global_report.mcp.is_none());
        assert!(global_report.mcp_note.is_some());
        assert!(global_report.plugin_hint.is_some());
    }

    #[test]
    #[serial]
    fn codex_global_merges_toml_project_skills_only() {
        let tmp = TempDir::new().unwrap();
        let home = tmp.path().join("home");
        let project = tmp.path().join("project");
        fs::create_dir_all(home.join(".codex")).unwrap();
        fs::write(home.join(".codex/config.toml"), "model = \"gpt-5\"\n").unwrap();
        fs::create_dir_all(&project).unwrap();
        let _g = HomeGuard::set(&home);
        let pack = minimal_pack(tmp.path());

        let global_report = install_harness(
            HarnessId::Codex,
            &pack,
            &InstallOptions {
                scope: SetupScope::Global,
                project_dir: project.clone(),
                skills: true,
                mcp: true,
                force: true,
            },
        )
        .unwrap();
        assert_eq!(global_report.skills_root, home.join(".codex/skills"));
        let cfg = fs::read_to_string(home.join(".codex/config.toml")).unwrap();
        assert!(cfg.contains("[mcp_servers.rusl]"));
        assert!(cfg.contains("model = \"gpt-5\""));

        let project_report = install_harness(
            HarnessId::Codex,
            &pack,
            &InstallOptions {
                scope: SetupScope::Project,
                project_dir: project.clone(),
                skills: true,
                mcp: true,
                force: true,
            },
        )
        .unwrap();
        assert_eq!(project_report.skills_root, project.join(".agents/skills"));
        assert!(project_report.mcp.is_none());
        assert!(project_report.mcp_note.is_some());
    }

    #[test]
    #[serial]
    fn opencode_and_grok_native_mcp_formats() {
        let tmp = TempDir::new().unwrap();
        let home = tmp.path().join("home");
        let project = tmp.path().join("project");
        fs::create_dir_all(home.join(".config/opencode")).unwrap();
        fs::create_dir_all(home.join(".grok")).unwrap();
        fs::create_dir_all(&project).unwrap();
        let _g = HomeGuard::set(&home);
        let pack = minimal_pack(tmp.path());

        install_harness(
            HarnessId::OpenCode,
            &pack,
            &InstallOptions {
                scope: SetupScope::Global,
                project_dir: project.clone(),
                skills: true,
                mcp: true,
                force: true,
            },
        )
        .unwrap();
        let oc: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(home.join(".config/opencode/opencode.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(oc["mcp"]["rusl"]["type"], "local");

        install_harness(
            HarnessId::Grok,
            &pack,
            &InstallOptions {
                scope: SetupScope::Project,
                project_dir: project.clone(),
                skills: true,
                mcp: true,
                force: true,
            },
        )
        .unwrap();
        assert!(project.join(".grok/skills/rusl-init/SKILL.md").is_file());
        let grok_cfg = fs::read_to_string(project.join(".grok/config.toml")).unwrap();
        assert!(grok_cfg.contains("[mcp_servers.rusl]"));
    }
}
