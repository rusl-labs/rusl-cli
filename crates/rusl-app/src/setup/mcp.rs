use anyhow::{Context, Result, bail};
use serde_json::{Value, json};
use std::path::Path;
use toml_edit::{DocumentMut, Item, Table, value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McpMergeAction {
    Created,
    Updated,
    Unchanged,
}

/// How a harness stores MCP server config.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum McpFormat {
    /// Cursor / Claude-style JSON: `{ "mcpServers": { "rusl": { "command", "args" } } }`
    McpServersJson,
    /// OpenCode JSON/JSONC: `{ "mcp": { "rusl": { "type": "local", "command": ["rusl","mcp"] } } }`
    OpenCodeJson,
    /// Codex / Grok TOML: `[mcp_servers.rusl]` with `command` + `args`
    McpServersToml,
}

#[derive(Debug, Clone)]
pub struct McpMergeResult {
    pub path: std::path::PathBuf,
    pub action: McpMergeAction,
    pub server_key: String,
    pub format: McpFormat,
}

/// Merge Rusl into a harness MCP config using the harness-native format.
pub fn merge_mcp(
    mcp_path: &Path,
    format: McpFormat,
    server_key: &str,
    command: &str,
    args: &[String],
) -> Result<McpMergeResult> {
    if let Some(parent) = mcp_path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create {}", parent.display()))?;
    }

    match format {
        McpFormat::McpServersJson => merge_mcp_servers_json(mcp_path, server_key, command, args),
        McpFormat::OpenCodeJson => merge_opencode_json(mcp_path, server_key, command, args),
        McpFormat::McpServersToml => merge_mcp_servers_toml(mcp_path, server_key, command, args),
    }
}

/// Cursor / Claude JSON fragment merge under `mcpServers.<key>`.
fn merge_mcp_servers_json(
    mcp_path: &Path,
    server_key: &str,
    command: &str,
    args: &[String],
) -> Result<McpMergeResult> {
    let server = json!({
        "command": command,
        "args": args,
    });

    let (mut root, existed) = if mcp_path.is_file() {
        let text = std::fs::read_to_string(mcp_path)
            .with_context(|| format!("Failed to read {}", mcp_path.display()))?;
        let value: Value = serde_json::from_str(&text)
            .with_context(|| format!("Invalid JSON in {}", mcp_path.display()))?;
        (value, true)
    } else {
        (json!({ "mcpServers": {} }), false)
    };

    let servers = root
        .as_object_mut()
        .context("MCP config root must be a JSON object")?
        .entry("mcpServers")
        .or_insert_with(|| json!({}));

    let servers_obj = servers
        .as_object_mut()
        .context("mcpServers must be a JSON object")?;

    let previous = servers_obj.get(server_key).cloned();
    servers_obj.insert(server_key.to_string(), server.clone());

    let action = classify_action(existed, previous.as_ref() == Some(&server));
    if action != McpMergeAction::Unchanged {
        write_pretty_json(mcp_path, &root)?;
    }

    Ok(McpMergeResult {
        path: mcp_path.to_path_buf(),
        action,
        server_key: server_key.to_string(),
        format: McpFormat::McpServersJson,
    })
}

/// OpenCode: `mcp.<key> = { type: "local", command: [bin, ...args] }`.
fn merge_opencode_json(
    mcp_path: &Path,
    server_key: &str,
    command: &str,
    args: &[String],
) -> Result<McpMergeResult> {
    let mut cmd = vec![command.to_string()];
    cmd.extend(args.iter().cloned());
    let server = json!({
        "type": "local",
        "command": cmd,
    });

    let (mut root, existed) = if mcp_path.is_file() {
        let text = std::fs::read_to_string(mcp_path)
            .with_context(|| format!("Failed to read {}", mcp_path.display()))?;
        let value: Value = parse_jsonc(&text)
            .with_context(|| format!("Invalid JSON/JSONC in {}", mcp_path.display()))?;
        (value, true)
    } else {
        (json!({}), false)
    };

    let mcp = root
        .as_object_mut()
        .context("OpenCode config root must be a JSON object")?
        .entry("mcp")
        .or_insert_with(|| json!({}));

    let mcp_obj = mcp
        .as_object_mut()
        .context("OpenCode mcp must be a JSON object")?;

    let previous = mcp_obj.get(server_key).cloned();
    mcp_obj.insert(server_key.to_string(), server.clone());

    let action = classify_action(existed, previous.as_ref() == Some(&server));
    if action != McpMergeAction::Unchanged {
        write_pretty_json(mcp_path, &root)?;
    }

    Ok(McpMergeResult {
        path: mcp_path.to_path_buf(),
        action,
        server_key: server_key.to_string(),
        format: McpFormat::OpenCodeJson,
    })
}

/// Codex / Grok: `[mcp_servers.<key>]` with command + args.
fn merge_mcp_servers_toml(
    mcp_path: &Path,
    server_key: &str,
    command: &str,
    args: &[String],
) -> Result<McpMergeResult> {
    let (mut doc, existed) = if mcp_path.is_file() {
        let text = std::fs::read_to_string(mcp_path)
            .with_context(|| format!("Failed to read {}", mcp_path.display()))?;
        let doc: DocumentMut = text
            .parse()
            .with_context(|| format!("Invalid TOML in {}", mcp_path.display()))?;
        (doc, true)
    } else {
        (DocumentMut::new(), false)
    };

    if !doc.as_table().contains_key("mcp_servers") {
        doc["mcp_servers"] = Item::Table(Table::new());
    }
    let servers = doc["mcp_servers"]
        .as_table_mut()
        .context("mcp_servers must be a TOML table")?;

    let previous_cmd = servers
        .get(server_key)
        .and_then(|t| t.get("command"))
        .and_then(|c| c.as_str())
        .map(str::to_string);
    let previous_args: Option<Vec<String>> = servers.get(server_key).and_then(|t| {
        t.get("args")?.as_array().map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
    });

    let unchanged =
        previous_cmd.as_deref() == Some(command) && previous_args.as_deref() == Some(args);

    if !servers.contains_key(server_key) {
        servers.insert(server_key, Item::Table(Table::new()));
    }
    let entry = servers
        .get_mut(server_key)
        .and_then(|i| i.as_table_mut())
        .context("mcp server entry must be a table")?;
    entry["command"] = value(command);
    let mut arr = toml_edit::Array::new();
    for a in args {
        arr.push(a.as_str());
    }
    entry["args"] = Item::Value(toml_edit::Value::Array(arr));

    let action = classify_action(existed, unchanged && previous_cmd.is_some());
    if action != McpMergeAction::Unchanged {
        atomic_write(mcp_path, &doc.to_string())?;
    }

    Ok(McpMergeResult {
        path: mcp_path.to_path_buf(),
        action,
        server_key: server_key.to_string(),
        format: McpFormat::McpServersToml,
    })
}

fn classify_action(existed: bool, unchanged: bool) -> McpMergeAction {
    if !existed {
        McpMergeAction::Created
    } else if unchanged {
        McpMergeAction::Unchanged
    } else {
        McpMergeAction::Updated
    }
}

fn write_pretty_json(path: &Path, root: &Value) -> Result<()> {
    let pretty = serde_json::to_string_pretty(root)?;
    atomic_write(path, &format!("{pretty}\n"))
}

fn atomic_write(path: &Path, contents: &str) -> Result<()> {
    let tmp = path.with_extension(format!(
        "{}.tmp",
        path.extension().and_then(|e| e.to_str()).unwrap_or("cfg")
    ));
    std::fs::write(&tmp, contents).with_context(|| format!("Failed to write {}", tmp.display()))?;
    std::fs::rename(&tmp, path)
        .with_context(|| format!("Failed to finalize config at {}", path.display()))?;
    Ok(())
}

/// Best-effort JSONC: strip `//` line comments and trailing commas.
fn parse_jsonc(text: &str) -> Result<Value> {
    if let Ok(v) = serde_json::from_str::<Value>(text) {
        return Ok(v);
    }
    let stripped = strip_jsonc(text);
    serde_json::from_str(&stripped).context("Failed to parse as JSON or JSONC")
}

fn strip_jsonc(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("//") {
            continue;
        }
        // Drop end-of-line // comments outside strings (best effort).
        let mut cleaned = line.to_string();
        if let Some(idx) = find_line_comment(&cleaned) {
            cleaned.truncate(idx);
        }
        out.push_str(cleaned.trim_end());
        out.push('\n');
    }
    // Remove trailing commas before } or ]
    let mut result = String::with_capacity(out.len());
    let chars: Vec<char> = out.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == ',' {
            let mut j = i + 1;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            if j < chars.len() && (chars[j] == '}' || chars[j] == ']') {
                i += 1;
                continue;
            }
        }
        result.push(chars[i]);
        i += 1;
    }
    result
}

fn find_line_comment(line: &str) -> Option<usize> {
    let mut in_string = false;
    let mut escape = false;
    let bytes = line.as_bytes();
    let mut i = 0;
    while i + 1 < bytes.len() {
        let c = bytes[i] as char;
        if escape {
            escape = false;
            i += 1;
            continue;
        }
        if in_string {
            if c == '\\' {
                escape = true;
            } else if c == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }
        if c == '"' {
            in_string = true;
            i += 1;
            continue;
        }
        if c == '/' && bytes[i + 1] as char == '/' {
            return Some(i);
        }
        i += 1;
    }
    None
}

pub fn load_mcp_fragment(path: &Path) -> Result<Value> {
    let text = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read MCP fragment {}", path.display()))?;
    serde_json::from_str(&text)
        .with_context(|| format!("Invalid MCP fragment JSON {}", path.display()))
}

/// Resolve the rusl binary path for MCP `command` (prefer absolute).
pub fn resolve_rusl_command() -> String {
    std::env::current_exe()
        .ok()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "rusl".to_string())
}

/// Extract command + args from pack fragment, with optional absolute command override.
pub fn command_and_args(
    fragment: &Value,
    command_override: Option<&str>,
) -> Result<(String, Vec<String>)> {
    let command = command_override
        .map(str::to_string)
        .or_else(|| {
            fragment
                .get("command")
                .and_then(|v| v.as_str())
                .map(str::to_string)
        })
        .unwrap_or_else(|| "rusl".to_string());

    let args = fragment
        .get("args")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_else(|| vec!["mcp".to_string()]);

    if command.is_empty() {
        bail!("MCP fragment has empty command");
    }
    Ok((command, args))
}

/// Check whether a config file already has a rusl MCP entry for doctor.
pub fn mcp_entry_present(path: &Path, format: McpFormat, server_key: &str) -> bool {
    if !path.is_file() {
        return false;
    }
    let Ok(text) = std::fs::read_to_string(path) else {
        return false;
    };
    match format {
        McpFormat::McpServersJson => serde_json::from_str::<Value>(&text)
            .ok()
            .and_then(|v| v.get("mcpServers")?.get(server_key).cloned())
            .is_some(),
        McpFormat::OpenCodeJson => parse_jsonc(&text)
            .ok()
            .and_then(|v| v.get("mcp")?.get(server_key).cloned())
            .is_some(),
        McpFormat::McpServersToml => text
            .parse::<DocumentMut>()
            .ok()
            .and_then(|doc| doc.get("mcp_servers")?.as_table()?.get(server_key).cloned())
            .is_some(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn merge_preserves_other_servers() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("mcp.json");
        fs::write(
            &path,
            r#"{
  "mcpServers": {
    "other": { "command": "other" }
  }
}
"#,
        )
        .unwrap();

        let result = merge_mcp(
            &path,
            McpFormat::McpServersJson,
            "rusl",
            "/opt/rusl",
            &["mcp".into()],
        )
        .unwrap();
        assert_eq!(result.action, McpMergeAction::Updated);

        let data: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(data["mcpServers"]["other"]["command"], "other");
        assert_eq!(data["mcpServers"]["rusl"]["command"], "/opt/rusl");
        assert_eq!(data["mcpServers"]["rusl"]["args"][0], "mcp");
    }

    #[test]
    fn merge_opencode_local_command_array() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("opencode.json");
        fs::write(
            &path,
            r#"{
  "mcp": {
    "other": { "type": "local", "command": ["echo"] }
  }
}
"#,
        )
        .unwrap();

        merge_mcp(
            &path,
            McpFormat::OpenCodeJson,
            "rusl",
            "/opt/rusl",
            &["mcp".into()],
        )
        .unwrap();

        let data: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
        assert_eq!(data["mcp"]["other"]["command"][0], "echo");
        assert_eq!(data["mcp"]["rusl"]["type"], "local");
        assert_eq!(data["mcp"]["rusl"]["command"][0], "/opt/rusl");
        assert_eq!(data["mcp"]["rusl"]["command"][1], "mcp");
    }

    #[test]
    fn merge_codex_toml_preserves_other_servers() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("config.toml");
        fs::write(
            &path,
            r#"
model = "gpt-5"

[mcp_servers.linear]
url = "https://mcp.linear.app/mcp"
"#,
        )
        .unwrap();

        merge_mcp(
            &path,
            McpFormat::McpServersToml,
            "rusl",
            "/opt/rusl",
            &["mcp".into()],
        )
        .unwrap();

        let text = fs::read_to_string(&path).unwrap();
        assert!(text.contains("model = \"gpt-5\""));
        assert!(text.contains("[mcp_servers.linear]"));
        assert!(text.contains("[mcp_servers.rusl]"));
        assert!(text.contains("command = \"/opt/rusl\""));
        assert!(text.contains("\"mcp\""));
    }

    #[test]
    fn merge_creates_file_when_missing() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("nested/mcp.json");
        let result = merge_mcp(
            &path,
            McpFormat::McpServersJson,
            "rusl",
            "rusl",
            &["mcp".into()],
        )
        .unwrap();
        assert_eq!(result.action, McpMergeAction::Created);
        assert!(path.is_file());
    }

    #[test]
    fn strip_jsonc_trailing_commas() {
        let text = r#"{
  "mcp": {
    "Rusl": {
      "type": "local",
      "command": ["rusl", "mcp"],
    },
  },
}"#;
        let v = parse_jsonc(text).unwrap();
        assert_eq!(v["mcp"]["Rusl"]["type"], "local");
    }
}
