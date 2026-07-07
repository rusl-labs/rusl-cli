use crate::commands::mcp::{MCP_AGENT, errors::to_mcp_error};
use rusl_app::annotation_service::{self, CreateAnnotationRequest};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use turbomcp::prelude::{McpError, McpResult, Tool, ToolInputSchema, ToolResult};

const CREATE_ANNOTATION_TOOL_DESCRIPTION: &str = "Create an annotation of any registered annotation type against a visible subject. Use for arbitrary annotation types (e.g. `rusl/annotation-types/storage-policy`) whose schema is not embedded in the CLI. For the typed feedback annotations (context-loading-hint, usage-report, domain-interpretation, semantic-link, trust-signal, context-request, source-attestation, migration-guide) prefer the dedicated `create_<kind>` tools — they validate content against the vendored feedback schema before hitting the API. `content` is forwarded verbatim; the server validates it against the annotation type's linked schema.";

pub(in crate::commands::mcp) fn definition() -> Tool {
    Tool::new("create_annotation", CREATE_ANNOTATION_TOOL_DESCRIPTION).with_schema(input_schema())
}

pub(in crate::commands::mcp) fn is_tool(name: &str) -> bool {
    name == "create_annotation"
}

pub(in crate::commands::mcp) async fn call(args: Value) -> McpResult<ToolResult> {
    let request: CreateAnnotationToolRequest = deserialize_request(args)?;
    let output = annotation_service::create_annotation_with_user_agent_context(
        request.into_service_request(),
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

fn input_schema() -> ToolInputSchema {
    ToolInputSchema::from_value(
        serde_json::to_value(schemars::schema_for!(CreateAnnotationToolRequest))
            .expect("create annotation tool schema serializes"),
    )
}

fn deserialize_request(args: Value) -> McpResult<CreateAnnotationToolRequest> {
    let args = match args {
        Value::Null => Value::Object(Default::default()),
        args => args,
    };

    serde_json::from_value(args).map_err(|error| {
        McpError::invalid_params(format!("Invalid create_annotation tool arguments: {error}"))
    })
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct CreateAnnotationToolRequest {
    #[schemars(
        description = "Account slug that will own (author) the created annotation. Must be an account the caller can write to. For a policy annotation that rusl-kv or a similar consumer will trust, choose an author account in that consumer's trusted-authors list."
    )]
    account_slug: String,
    #[schemars(
        description = "GUID of the annotated subject (e.g. `schemas.<uuid>`, `bundles.<uuid>`, `annotation_types.<uuid>`). Fetch with search or get_schema before calling."
    )]
    subject_guid: String,
    #[schemars(
        description = "Canonical annotation type identifier in account/annotation-types/slug form (e.g. `rusl/annotation-types/storage-policy`). The annotation type must already exist; the server rejects unknown types."
    )]
    annotation_type: String,
    #[schemars(
        description = "Optional short human-readable label. Purely descriptive — does not affect resolution or validation."
    )]
    label: Option<String>,
    #[schemars(
        description = "Annotation content, forwarded verbatim to the API. Must satisfy the annotation type's linked validation schema; the server rejects invalid content."
    )]
    content: Value,
}

impl CreateAnnotationToolRequest {
    fn into_service_request(self) -> CreateAnnotationRequest {
        CreateAnnotationRequest {
            account_slug: self.account_slug,
            subject_guid: self.subject_guid,
            annotation_type: self.annotation_type,
            label: self.label,
            content: self.content,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CreateAnnotationToolRequest, definition, is_tool};
    use serde_json::json;

    #[test]
    fn exposes_create_annotation_tool() {
        let tool = definition();
        assert_eq!(tool.name, "create_annotation");
        assert!(is_tool("create_annotation"));
        assert!(!is_tool("create_annotation_type"));
    }

    #[test]
    fn deserializes_full_request() {
        let request: CreateAnnotationToolRequest = serde_json::from_value(json!({
            "account_slug": "rusl",
            "subject_guid": "schemas.abc",
            "annotation_type": "rusl/annotation-types/storage-policy",
            "label": "KV storage policy for rusl/common",
            "content": {
                "owner_writes_only": false,
                "immutable": false,
                "deletable": true,
                "read_access": "tenant_only",
                "max_cardinality": "unlimited"
            }
        }))
        .expect("valid request deserializes");

        assert_eq!(request.account_slug, "rusl");
        assert_eq!(request.subject_guid, "schemas.abc");
        assert_eq!(
            request.annotation_type,
            "rusl/annotation-types/storage-policy"
        );
        assert_eq!(
            request.label.as_deref(),
            Some("KV storage policy for rusl/common")
        );
        assert!(request.content.is_object());
    }

    #[test]
    fn label_is_optional() {
        let request: CreateAnnotationToolRequest = serde_json::from_value(json!({
            "account_slug": "rusl",
            "subject_guid": "schemas.abc",
            "annotation_type": "rusl/annotation-types/storage-policy",
            "content": {}
        }))
        .expect("valid request deserializes");

        assert!(request.label.is_none());
    }
}
