use serde::{Deserialize, Serialize};

/// The JSON payload piped to a generator plugin's stdin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationRequest {
    /// Protocol version. Always "1" for now.
    pub version: String,
    /// Contents of `[generators.<name>.args]` — opaque to rusl.
    pub options: serde_json::Value,
    /// Ordered array of schemas (reverse topological order — leaves first).
    pub schemas: Vec<SchemaEntry>,
}

/// A single schema entry within the generation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaEntry {
    /// Fully qualified schema name (`account/slug`).
    pub name: String,
    /// Exact resolved version.
    pub version: String,
    /// Source type: "schema", "bundle", or "external".
    pub kind: String,
    /// `true` if this schema matched the filter, `false` if included as a transitive dep.
    pub target: bool,
    /// The full JSON Schema content inline.
    pub content: Option<serde_json::Value>,
    /// Relative file path to the schema on disk.
    pub content_ref: Option<String>,
    /// Direct dependency names (for graph awareness).
    pub dependencies: Vec<String>,
}

/// The JSON payload a generator plugin writes to stdout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResponse {
    /// Protocol version.
    pub version: String,
    /// Array of files to write.
    pub files: Vec<GeneratedFile>,
}

/// A single generated file returned by the plugin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedFile {
    /// Relative path within `output_dir`.
    pub path: String,
    /// Full file content.
    pub content: String,
}
