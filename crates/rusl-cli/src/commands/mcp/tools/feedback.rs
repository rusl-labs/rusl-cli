use crate::commands::mcp::{MCP_AGENT, errors::to_mcp_error};
use rusl_app::annotation_service::{self, CreateFeedbackAnnotationRequest, FeedbackAnnotationKind};
use serde_json::{Map, Value, json};
use turbomcp::prelude::{McpError, McpResult, Tool, ToolInputSchema, ToolResult};

const ENVELOPE_ACCOUNT_SLUG: &str = "account_slug";
const ENVELOPE_SUBJECT_GUID: &str = "subject_guid";
const ENVELOPE_LABEL: &str = "label";

#[derive(Debug, Clone, Copy)]
struct FeedbackToolSpec {
    name: &'static str,
    description: &'static str,
    kind: FeedbackAnnotationKind,
}

const FEEDBACK_TOOLS: &[FeedbackToolSpec] = &[
    FeedbackToolSpec {
        name: "create_context_loading_hint",
        description: "Create a context-loading-hint feedback annotation with guidance for what context an agent should load before using a contract.",
        kind: FeedbackAnnotationKind::ContextLoadingHint,
    },
    FeedbackToolSpec {
        name: "create_usage_report",
        description: "Create a usage-report feedback annotation recording that a contract was used in a project, tool call, generator, validator, or workflow.",
        kind: FeedbackAnnotationKind::UsageReport,
    },
    FeedbackToolSpec {
        name: "create_domain_interpretation",
        description: "Create a domain-interpretation feedback annotation explaining what a field or contract means in a specific domain.",
        kind: FeedbackAnnotationKind::DomainInterpretation,
    },
    FeedbackToolSpec {
        name: "create_semantic_link",
        description: "Create a semantic-link feedback annotation relating contracts, fields, annotations, docs, or external sources.",
        kind: FeedbackAnnotationKind::SemanticLink,
    },
    FeedbackToolSpec {
        name: "create_trust_signal",
        description: "Create a trust-signal feedback annotation with evidence that helps consumers decide whether a contract or annotation is dependable.",
        kind: FeedbackAnnotationKind::TrustSignal,
    },
    FeedbackToolSpec {
        name: "create_context_request",
        description: "Create a context-request feedback annotation asking for missing meaning when an agent or human cannot safely proceed.",
        kind: FeedbackAnnotationKind::ContextRequest,
    },
    FeedbackToolSpec {
        name: "create_source_attestation",
        description: "Create a source-attestation feedback annotation connecting a contract or annotation back to its source of authority.",
        kind: FeedbackAnnotationKind::SourceAttestation,
    },
    FeedbackToolSpec {
        name: "create_migration_guide",
        description: "Create a migration-guide feedback annotation with version-to-version compatibility, breaking change, and upgrade guidance.",
        kind: FeedbackAnnotationKind::MigrationGuide,
    },
];

pub(in crate::commands::mcp) fn definitions() -> Vec<Tool> {
    FEEDBACK_TOOLS
        .iter()
        .map(|spec| Tool::new(spec.name, spec.description).with_schema(input_schema(spec.kind)))
        .collect()
}

pub(in crate::commands::mcp) fn kind_for_tool(name: &str) -> Option<FeedbackAnnotationKind> {
    FEEDBACK_TOOLS
        .iter()
        .find(|spec| spec.name == name)
        .map(|spec| spec.kind)
}

pub(in crate::commands::mcp) async fn call(
    kind: FeedbackAnnotationKind,
    args: Value,
) -> McpResult<ToolResult> {
    let (envelope, content) = split_feedback_request(args)?;
    let output = annotation_service::create_feedback_annotation_with_user_agent_context(
        CreateFeedbackAnnotationRequest {
            account_slug: envelope.account_slug,
            subject_guid: envelope.subject_guid,
            kind,
            label: envelope.label,
            content,
        },
        MCP_AGENT,
    )
    .await
    .map_err(to_mcp_error)?;

    ToolResult::json(&output).map_err(|error| {
        McpError::internal(format!(
            "Failed to serialize create annotation response: {error}"
        ))
    })
}

fn input_schema(kind: FeedbackAnnotationKind) -> ToolInputSchema {
    let schema = compose_input_schema(kind).unwrap_or_else(|error| {
        missing_schema_input_schema(kind, &format!("Failed to load feedback schema: {error}"))
    });
    ToolInputSchema::from_value(schema)
}

fn compose_input_schema(kind: FeedbackAnnotationKind) -> anyhow::Result<Value> {
    let content_schema = annotation_service::feedback_content_schema(kind)?;
    let content_properties = content_schema
        .get("properties")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let content_required = content_schema
        .get("required")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let mut properties = envelope_properties();
    for (key, value) in content_properties {
        if properties.contains_key(&key) {
            anyhow::bail!("Feedback schema property {key} conflicts with the MCP envelope");
        }
        properties.insert(key, value);
    }

    let mut required = vec![json!(ENVELOPE_ACCOUNT_SLUG), json!(ENVELOPE_SUBJECT_GUID)];
    required.extend(content_required);

    Ok(json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": format!("Create {}", kind.type_identifier()),
        "description": "Envelope fields identify the annotation owner and subject. The remaining fields are the feedback annotation content validated against the vendored Rusl schema.",
        "type": "object",
        "additionalProperties": false,
        "properties": properties,
        "required": required
    }))
}

fn missing_schema_input_schema(kind: FeedbackAnnotationKind, reason: &str) -> Value {
    let mut properties = envelope_properties();
    properties.insert(
        "content".to_string(),
        json!({
            "type": "object",
            "description": "Raw annotation content. Typed fields are unavailable because the local feedback schema could not be loaded."
        }),
    );

    json!({
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": format!("Create {}", kind.type_identifier()),
        "description": reason,
        "type": "object",
        "additionalProperties": false,
        "properties": properties,
        "required": [ENVELOPE_ACCOUNT_SLUG, ENVELOPE_SUBJECT_GUID, "content"]
    })
}

fn envelope_properties() -> Map<String, Value> {
    Map::from_iter([
        (
            ENVELOPE_ACCOUNT_SLUG.to_string(),
            json!({
                "type": "string",
                "minLength": 1,
                "description": "Account slug that will own the created annotation."
            }),
        ),
        (
            ENVELOPE_SUBJECT_GUID.to_string(),
            json!({
                "type": "string",
                "minLength": 1,
                "description": "GUID of the schema, bundle, annotation, version, proposal, or other visible subject being annotated."
            }),
        ),
        (
            ENVELOPE_LABEL.to_string(),
            json!({
                "type": "string",
                "description": "Optional short human-readable annotation label."
            }),
        ),
    ])
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FeedbackEnvelope {
    account_slug: String,
    subject_guid: String,
    label: Option<String>,
}

fn split_feedback_request(args: Value) -> McpResult<(FeedbackEnvelope, Value)> {
    let mut object = match args {
        Value::Object(object) => object,
        Value::Null => {
            return Err(McpError::invalid_params(
                "Feedback tool arguments must be an object",
            ));
        }
        _ => {
            return Err(McpError::invalid_params(
                "Feedback tool arguments must be an object",
            ));
        }
    };

    let account_slug = take_required_string(&mut object, ENVELOPE_ACCOUNT_SLUG)?;
    let subject_guid = take_required_string(&mut object, ENVELOPE_SUBJECT_GUID)?;
    let label = take_optional_string(&mut object, ENVELOPE_LABEL)?;

    Ok((
        FeedbackEnvelope {
            account_slug,
            subject_guid,
            label,
        },
        Value::Object(object),
    ))
}

fn take_required_string(object: &mut Map<String, Value>, key: &str) -> McpResult<String> {
    match object.remove(key) {
        Some(Value::String(value)) if !value.trim().is_empty() => Ok(value),
        Some(_) => Err(McpError::invalid_params(format!(
            "{key} must be a non-empty string"
        ))),
        None => Err(McpError::invalid_params(format!("{key} is required"))),
    }
}

fn take_optional_string(object: &mut Map<String, Value>, key: &str) -> McpResult<Option<String>> {
    match object.remove(key) {
        Some(Value::String(value)) if !value.trim().is_empty() => Ok(Some(value)),
        Some(Value::String(_)) | Some(Value::Null) | None => Ok(None),
        Some(_) => Err(McpError::invalid_params(format!("{key} must be a string"))),
    }
}

#[cfg(test)]
mod tests {
    use super::{FeedbackAnnotationKind, compose_input_schema, split_feedback_request};
    use serde_json::json;
    use serial_test::serial;
    use std::{ffi::OsString, path::PathBuf};
    use tempfile::TempDir;

    const CONTEXT_REQUEST_SCHEMA: &str =
        include_str!("../../../../../../schemas/vendor/rusl/context-request.json");

    struct SchemaWorkspaceGuard {
        previous_home: Option<OsString>,
        previous_xdg_config_home: Option<OsString>,
        previous_xdg_data_home: Option<OsString>,
        previous_dir: PathBuf,
        _temp_dir: TempDir,
    }

    impl SchemaWorkspaceGuard {
        fn new() -> Self {
            let temp_dir = TempDir::new().expect("create temp dir");
            let home_dir = temp_dir.path().join("home");
            let workspace_dir = temp_dir.path().join("workspace");
            let schema_dir = workspace_dir.join("schemas").join("rusl");
            std::fs::create_dir_all(&home_dir).expect("create home dir");
            std::fs::create_dir_all(&schema_dir).expect("create schema dir");
            std::fs::write(
                schema_dir.join("context-request.json"),
                CONTEXT_REQUEST_SCHEMA,
            )
            .expect("write context request schema");

            let previous_home = std::env::var_os(home_var_name());
            let previous_xdg_config_home = std::env::var_os("XDG_CONFIG_HOME");
            let previous_xdg_data_home = std::env::var_os("XDG_DATA_HOME");
            let previous_dir = std::env::current_dir().expect("current dir");

            set_env_var(home_var_name(), home_dir.as_os_str());
            set_env_var("XDG_CONFIG_HOME", home_dir.join(".config"));
            set_env_var("XDG_DATA_HOME", home_dir.join(".local").join("share"));
            std::env::set_current_dir(&workspace_dir).expect("set workspace dir");

            Self {
                previous_home,
                previous_xdg_config_home,
                previous_xdg_data_home,
                previous_dir,
                _temp_dir: temp_dir,
            }
        }
    }

    impl Drop for SchemaWorkspaceGuard {
        fn drop(&mut self) {
            restore_env_var(home_var_name(), self.previous_home.as_ref());
            restore_env_var("XDG_CONFIG_HOME", self.previous_xdg_config_home.as_ref());
            restore_env_var("XDG_DATA_HOME", self.previous_xdg_data_home.as_ref());
            std::env::set_current_dir(&self.previous_dir).expect("restore current dir");
        }
    }

    #[test]
    #[serial]
    fn composes_envelope_and_feedback_content_schema() {
        let _guard = SchemaWorkspaceGuard::new();

        let schema = compose_input_schema(FeedbackAnnotationKind::ContextRequest)
            .expect("compose input schema");

        let properties = schema["properties"].as_object().expect("properties object");
        assert!(properties.contains_key("account_slug"));
        assert!(properties.contains_key("subject_guid"));
        assert!(properties.contains_key("failing_task"));
        assert!(properties.contains_key("suspected_ambiguity"));
        assert_eq!(schema["additionalProperties"], json!(false));
    }

    #[test]
    fn splits_envelope_from_flat_content_fields() {
        let (envelope, content) = split_feedback_request(json!({
            "account_slug": "hassox",
            "subject_guid": "schemas.123",
            "label": "Needs context",
            "failing_task": "Generate bindings",
            "suspected_ambiguity": "Unit field is unclear"
        }))
        .expect("split request");

        assert_eq!(envelope.account_slug, "hassox");
        assert_eq!(envelope.subject_guid, "schemas.123");
        assert_eq!(envelope.label, Some("Needs context".to_string()));
        assert_eq!(
            content,
            json!({
                "failing_task": "Generate bindings",
                "suspected_ambiguity": "Unit field is unclear"
            })
        );
    }

    #[cfg(windows)]
    fn home_var_name() -> &'static str {
        "USERPROFILE"
    }

    #[cfg(not(windows))]
    fn home_var_name() -> &'static str {
        "HOME"
    }

    fn set_env_var<K, V>(key: K, value: V)
    where
        K: AsRef<std::ffi::OsStr>,
        V: AsRef<std::ffi::OsStr>,
    {
        unsafe { std::env::set_var(key, value) }
    }

    fn restore_env_var(key: &str, value: Option<&OsString>) {
        match value {
            Some(value) => unsafe { std::env::set_var(key, value) },
            None => unsafe { std::env::remove_var(key) },
        }
    }
}
