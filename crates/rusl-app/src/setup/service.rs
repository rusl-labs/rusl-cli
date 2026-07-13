use super::harness::{
    HarnessId, HarnessInstallReport, InstallOptions, SetupScope, doctor_mcp, doctor_skills_root,
    install_harness, parse_targets_in,
};
use super::lockfile::AgentSkillsLock;
use super::resolve::{ResolvePackOptions, ResolvedPack, resolve_pack};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct SetupRequest {
    pub targets: Vec<String>,
    pub all: bool,
    pub pack_dir: Option<PathBuf>,
    pub tag: Option<String>,
    /// Install scope. Defaults to project (cwd) when neither flag is set.
    pub scope: SetupScope,
    pub skills_only: bool,
    pub mcp_only: bool,
    pub force: bool,
    /// When false, do not git-clone (tests / offline explicit pack only).
    pub allow_clone: bool,
}

impl SetupRequest {
    pub fn production() -> Self {
        Self {
            allow_clone: true,
            scope: SetupScope::Project,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct SetupResult {
    pub pack: ResolvedPack,
    pub harnesses: Vec<HarnessInstallReport>,
    pub lock_path: PathBuf,
}

/// Install agent skills + MCP for the selected harnesses.
pub fn setup(request: SetupRequest) -> Result<SetupResult> {
    let pack = resolve_pack(&ResolvePackOptions {
        pack_dir: request.pack_dir.clone(),
        tag: request.tag.clone(),
        clone_url: None,
        allow_clone: request.allow_clone,
    })?;

    let project_dir = std::env::current_dir()?;
    let harnesses = parse_targets_in(&request.targets, request.all, Some(project_dir.as_path()))?;
    let scope = request.scope;

    let install_opts = InstallOptions {
        scope,
        project_dir: project_dir.clone(),
        skills: !request.mcp_only,
        mcp: !request.skills_only,
        force: request.force,
    };

    let mut reports = Vec::new();
    for harness in &harnesses {
        reports.push(install_harness(*harness, &pack, &install_opts)?);
    }

    let lock = AgentSkillsLock::from_install(
        &pack.manifest.version,
        pack.source,
        &pack.pack_root,
        pack.kit_root.as_deref(),
        pack.symlink_target.as_deref(),
        &harnesses,
        match scope {
            SetupScope::Global => "global",
            SetupScope::Project => "project",
        },
    );
    let lock_path = lock.save()?;

    Ok(SetupResult {
        pack,
        harnesses: reports,
        lock_path,
    })
}

#[derive(Debug, Clone)]
pub struct DoctorReport {
    pub pack: Option<ResolvedPack>,
    pub pack_error: Option<String>,
    pub lock: Option<AgentSkillsLock>,
    pub harness_checks: Vec<HarnessDoctorCheck>,
}

#[derive(Debug, Clone)]
pub struct HarnessDoctorCheck {
    pub harness: HarnessId,
    pub scope: SetupScope,
    pub skills_root: PathBuf,
    pub skills_present: Vec<String>,
    pub skills_missing: Vec<String>,
    pub mcp_ok: Option<bool>,
    pub mcp_path: Option<PathBuf>,
    pub mcp_note: Option<String>,
}

/// Non-mutating health check for installed skills/MCP.
///
/// Checks project-scope paths under cwd and global-scope paths under `$HOME`.
pub fn doctor(pack_dir: Option<PathBuf>) -> Result<DoctorReport> {
    let lock = AgentSkillsLock::load()?;

    let (pack, pack_error) = match resolve_pack(&ResolvePackOptions {
        pack_dir,
        tag: None,
        clone_url: None,
        allow_clone: false,
    }) {
        Ok(p) => (Some(p), None),
        Err(e) => (None, Some(e.to_string())),
    };

    let expected_skills: Vec<String> = pack
        .as_ref()
        .map(|p| {
            p.manifest
                .skills
                .iter()
                .map(|s| s.install_name.clone())
                .collect()
        })
        .unwrap_or_else(|| {
            vec![
                "rusl-init".into(),
                "rusl-resolve".into(),
                "rusl-reuse".into(),
                "rusl-feedback".into(),
                "rusl-proposals".into(),
            ]
        });

    let server_key = pack
        .as_ref()
        .map(|p| p.manifest.mcp.server_key.as_str())
        .unwrap_or("rusl");

    let harness_ids = lock
        .as_ref()
        .map(|l| {
            l.harnesses
                .iter()
                .filter_map(|h| HarnessId::parse(h))
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| HarnessId::default_install_set().to_vec());

    let project_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let preferred_scope = lock.as_ref().map(|l| l.scope.as_str()).unwrap_or("project");

    let mut harness_checks = Vec::new();
    for harness in harness_ids {
        let scopes = match preferred_scope {
            "global" => [SetupScope::Global, SetupScope::Project],
            _ => [SetupScope::Project, SetupScope::Global],
        };
        for scope in scopes {
            let check = check_harness(harness, scope, &project_dir, &expected_skills, server_key)?;
            let preferred = match preferred_scope {
                "global" => SetupScope::Global,
                _ => SetupScope::Project,
            };
            let has_any = !check.skills_present.is_empty()
                || check.mcp_ok == Some(true)
                || scope == preferred;
            if has_any {
                harness_checks.push(check);
            }
        }
    }

    Ok(DoctorReport {
        pack,
        pack_error,
        lock,
        harness_checks,
    })
}

fn check_harness(
    harness: HarnessId,
    scope: SetupScope,
    project_dir: &std::path::Path,
    expected: &[String],
    server_key: &str,
) -> Result<HarnessDoctorCheck> {
    let skills_root = doctor_skills_root(harness, scope, project_dir)?;
    let mut present = Vec::new();
    let mut missing = Vec::new();
    for name in expected {
        if skills_root.join(name).join("SKILL.md").is_file() {
            present.push(name.clone());
        } else {
            missing.push(name.clone());
        }
    }

    let adapter = harness.adapter();
    let (mcp_path, mcp_ok, mcp_note) = match doctor_mcp(harness, scope, project_dir, server_key)? {
        Some((path, _format, present)) => (Some(path), Some(present), None),
        None => (None, None, adapter.mcp_skip_note(scope).map(str::to_string)),
    };

    Ok(HarnessDoctorCheck {
        harness,
        scope,
        skills_root,
        skills_present: present,
        skills_missing: missing,
        mcp_ok,
        mcp_path,
        mcp_note,
    })
}

pub fn status() -> Result<Option<AgentSkillsLock>> {
    AgentSkillsLock::load()
}
