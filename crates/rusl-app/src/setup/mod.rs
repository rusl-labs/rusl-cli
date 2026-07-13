//! Agent harness setup: fetch pack, install skills, merge MCP.

mod cursor_sandbox;
mod harness;
mod install;
mod lockfile;
mod manifest;
mod mcp;
mod paths;
mod resolve;
mod service;

pub use cursor_sandbox::{CursorSandboxMergeResult, merge_cursor_sandbox, rusl_app_data_dir};
pub use harness::{
    HarnessAdapter, HarnessId, HarnessInstallReport, InstallOptions, SetupScope, detect_harnesses,
    install_harness, parse_targets,
};
pub use install::{InstallAction, SkillInstallResult, SkillsInstallReport};
pub use lockfile::AgentSkillsLock;
pub use manifest::PackManifest;
pub use mcp::{McpFormat, McpMergeAction, McpMergeResult};
pub use paths::{agent_kit_cache_dir, agent_skills_lock_path, rusl_home};
pub use resolve::{
    DEFAULT_CLONE_URL, DEFAULT_GITHUB_REPO, PACK_DIR_ENV, PackSourceKind, ResolvePackOptions,
    ResolvedPack, resolve_pack,
};
pub use service::{
    DoctorReport, HarnessDoctorCheck, SetupRequest, SetupResult, doctor, setup, status,
};
