//! Merge Rusl paths/network into Cursor's `~/.cursor/sandbox.json`.
//!
//! Cursor's agent sandbox is deny-by-default for paths outside the workspace.
//! Credentials and the schema store live under the OS app-data dir (on macOS:
//! `~/Library/Application Support/rusl`), and agent-kit cache/lock live under
//! `~/.rusl`. Both must be allowlisted for read/write. Network access to the
//! Rusl API must also be allowed.
//!
//! Existing keys and list entries are preserved; we only add what is missing.

use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

use super::mcp::McpMergeAction;
use super::paths::rusl_home;

/// Hosts Cursor needs for Rusl CLI + registry access from the agent sandbox.
const NETWORK_ALLOWS: &[&str] = &["https://resources.rusl.com", "resources.rusl.com"];

#[derive(Debug, Clone)]
pub struct CursorSandboxMergeResult {
    pub path: PathBuf,
    pub action: McpMergeAction,
    pub paths_added: Vec<String>,
    pub network_added: Vec<String>,
}

/// Path to Cursor's user sandbox config (`~/.cursor/sandbox.json`).
pub fn cursor_sandbox_path() -> Result<PathBuf> {
    let home =
        std::env::var("HOME").context("HOME is required to locate ~/.cursor/sandbox.json")?;
    Ok(PathBuf::from(home).join(".cursor/sandbox.json"))
}

/// Absolute path of Rusl's app-data directory (credentials + schema store).
///
/// Uses the same `directories::ProjectDirs` root as credentials/CAS store.
pub fn rusl_app_data_dir() -> Result<PathBuf> {
    let proj = directories::ProjectDirs::from("", "", "rusl")
        .context("Could not resolve Rusl application data directory")?;
    // On macOS config_dir and data_dir are both Application Support/rusl.
    // Prefer data_dir (store lives there); credentials share the parent.
    Ok(proj.data_dir().to_path_buf())
}

/// Paths that must be readable/writable from Cursor's agent sandbox.
pub fn sandbox_readwrite_paths() -> Result<Vec<String>> {
    Ok(vec![
        rusl_app_data_dir()?.to_string_lossy().into_owned(),
        rusl_home()?.to_string_lossy().into_owned(),
    ])
}

/// Merge Rusl sandbox requirements into `~/.cursor/sandbox.json` without clobbering.
pub fn merge_cursor_sandbox() -> Result<CursorSandboxMergeResult> {
    let path = cursor_sandbox_path()?;
    let readwrite = sandbox_readwrite_paths()?;
    merge_cursor_sandbox_at(&path, &readwrite, NETWORK_ALLOWS)
}

pub fn merge_cursor_sandbox_at(
    path: &Path,
    readwrite_paths: &[String],
    network_allows: &[&str],
) -> Result<CursorSandboxMergeResult> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }

    let (mut root, existed) = if path.is_file() {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let value: Value = serde_json::from_str(&text)
            .with_context(|| format!("Invalid JSON in {}", path.display()))?;
        (value, true)
    } else {
        (json!({}), false)
    };

    let obj = root
        .as_object_mut()
        .context("Cursor sandbox.json root must be a JSON object")?;

    // Only set type when missing — do not overwrite a custom sandbox type.
    if !obj.contains_key("type") {
        obj.insert("type".to_string(), json!("workspace_readwrite"));
    }

    let mut paths_added = Vec::new();
    {
        let paths = obj
            .entry("additionalReadwritePaths")
            .or_insert_with(|| json!([]));
        let arr = paths
            .as_array_mut()
            .context("additionalReadwritePaths must be a JSON array")?;
        for p in readwrite_paths {
            if !arr.iter().any(|v| v.as_str() == Some(p.as_str())) {
                arr.push(json!(p));
                paths_added.push(p.clone());
            }
        }
    }

    let mut network_added = Vec::new();
    {
        let policy = obj
            .entry("networkPolicy")
            .or_insert_with(|| json!({ "default": "deny", "allow": [] }));
        let policy_obj = policy
            .as_object_mut()
            .context("networkPolicy must be a JSON object")?;
        if !policy_obj.contains_key("default") {
            policy_obj.insert("default".to_string(), json!("deny"));
        }
        let allow = policy_obj.entry("allow").or_insert_with(|| json!([]));
        let allow_arr = allow
            .as_array_mut()
            .context("networkPolicy.allow must be a JSON array")?;
        for host in network_allows {
            if !allow_arr.iter().any(|v| v.as_str() == Some(host)) {
                allow_arr.push(json!(host));
                network_added.push((*host).to_string());
            }
        }
    }

    let changed = !existed || !paths_added.is_empty() || !network_added.is_empty();
    let action = if !existed {
        McpMergeAction::Created
    } else if changed {
        McpMergeAction::Updated
    } else {
        McpMergeAction::Unchanged
    };

    if action != McpMergeAction::Unchanged {
        let pretty = serde_json::to_string_pretty(&root)?;
        let tmp = path.with_extension("json.tmp");
        std::fs::write(&tmp, format!("{pretty}\n"))
            .with_context(|| format!("Failed to write {}", tmp.display()))?;
        std::fs::rename(&tmp, path)
            .with_context(|| format!("Failed to finalize {}", path.display()))?;
    }

    Ok(CursorSandboxMergeResult {
        path: path.to_path_buf(),
        action,
        paths_added,
        network_added,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn sample_paths() -> Vec<String> {
        vec![
            "/Users/me/Library/Application Support/rusl".into(),
            "/Users/me/.rusl".into(),
        ]
    }

    #[test]
    fn creates_sandbox_when_missing() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("sandbox.json");
        let paths = sample_paths();
        let result = merge_cursor_sandbox_at(&path, &paths, NETWORK_ALLOWS).unwrap();
        assert_eq!(result.action, McpMergeAction::Created);
        assert_eq!(result.paths_added.len(), 2);

        let data: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(data["type"], "workspace_readwrite");
        let rw = data["additionalReadwritePaths"].as_array().unwrap();
        assert_eq!(rw[0], "/Users/me/Library/Application Support/rusl");
        assert_eq!(rw[1], "/Users/me/.rusl");
        assert_eq!(data["networkPolicy"]["default"], "deny");
        assert!(
            data["networkPolicy"]["allow"]
                .as_array()
                .unwrap()
                .iter()
                .any(|v| v == "resources.rusl.com")
        );
    }

    #[test]
    fn merges_without_clobbering_existing_entries() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("sandbox.json");
        fs::write(
            &path,
            r#"{
  "type": "workspace_readwrite",
  "additionalReadwritePaths": [
    "/already/there"
  ],
  "networkPolicy": {
    "default": "deny",
    "allow": [
      "https://example.com"
    ]
  },
  "customKey": true
}
"#,
        )
        .unwrap();

        let paths = sample_paths();
        let result = merge_cursor_sandbox_at(&path, &paths, NETWORK_ALLOWS).unwrap();
        assert_eq!(result.action, McpMergeAction::Updated);
        assert_eq!(result.paths_added, paths);
        assert!(result.network_added.contains(&"resources.rusl.com".into()));

        let data: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        // Preserved
        assert_eq!(data["customKey"], true);
        assert_eq!(data["additionalReadwritePaths"][0], "/already/there");
        assert_eq!(data["networkPolicy"]["allow"][0], "https://example.com");
        // Added
        assert_eq!(
            data["additionalReadwritePaths"][1],
            "/Users/me/Library/Application Support/rusl"
        );
        assert_eq!(data["additionalReadwritePaths"][2], "/Users/me/.rusl");
        let allows = data["networkPolicy"]["allow"].as_array().unwrap();
        assert!(allows.iter().any(|v| v == "https://resources.rusl.com"));
        assert!(allows.iter().any(|v| v == "resources.rusl.com"));
    }

    #[test]
    fn is_idempotent_when_already_configured() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("sandbox.json");
        let paths = sample_paths();
        fs::write(
            &path,
            r#"{
  "type": "workspace_readwrite",
  "additionalReadwritePaths": [
    "/Users/me/Library/Application Support/rusl",
    "/Users/me/.rusl"
  ],
  "networkPolicy": {
    "default": "deny",
    "allow": ["https://resources.rusl.com", "resources.rusl.com"]
  }
}
"#,
        )
        .unwrap();

        let result = merge_cursor_sandbox_at(&path, &paths, NETWORK_ALLOWS).unwrap();
        assert_eq!(result.action, McpMergeAction::Unchanged);
        assert!(result.paths_added.is_empty());
        assert!(result.network_added.is_empty());
    }

    #[test]
    fn adds_only_missing_dot_rusl_path() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("sandbox.json");
        fs::write(
            &path,
            r#"{
  "type": "workspace_readwrite",
  "additionalReadwritePaths": [
    "/Users/me/Library/Application Support/rusl"
  ],
  "networkPolicy": {
    "default": "deny",
    "allow": ["https://resources.rusl.com", "resources.rusl.com"]
  }
}
"#,
        )
        .unwrap();

        let result = merge_cursor_sandbox_at(&path, &sample_paths(), NETWORK_ALLOWS).unwrap();
        assert_eq!(result.action, McpMergeAction::Updated);
        assert_eq!(result.paths_added, vec!["/Users/me/.rusl".to_string()]);
        assert!(result.network_added.is_empty());
    }

    #[test]
    fn does_not_overwrite_existing_type() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("sandbox.json");
        fs::write(
            &path,
            r#"{"type": "readonly", "additionalReadwritePaths": []}"#,
        )
        .unwrap();

        merge_cursor_sandbox_at(
            &path,
            &["/tmp/rusl".into(), "/tmp/.rusl".into()],
            NETWORK_ALLOWS,
        )
        .unwrap();

        let data: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(data["type"], "readonly");
    }
}
