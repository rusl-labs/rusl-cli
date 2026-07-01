use crate::commands::mcp::{MCP_AGENT, errors::to_mcp_error};
use rusl_app::annotation_type_service::{
    self, AnnotationTypeCardinality, AnnotationTypeSchemaMode, AnnotationTypeVisibility,
    CreateAnnotationTypeRequest,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use turbomcp::prelude::{McpError, McpResult, Tool, ToolInputSchema, ToolResult};

const CREATE_ANNOTATION_TYPE_TOOL_DESCRIPTION: &str = "Use when the user wants to register a new annotation type before creating annotations of that type. Requires user authorization, a validation schema identifier, and explicit schema_mode.";

pub(in crate::commands::mcp) fn definition() -> Tool {
    Tool::new(
        "create_annotation_type",
        CREATE_ANNOTATION_TYPE_TOOL_DESCRIPTION,
    )
    .with_schema(input_schema())
}

pub(in crate::commands::mcp) fn is_tool(name: &str) -> bool {
    name == "create_annotation_type"
}

pub(in crate::commands::mcp) async fn call(args: Value) -> McpResult<ToolResult> {
    let request: CreateAnnotationTypeToolRequest =
        deserialize_request(args, "create_annotation_type")?;
    let output = annotation_type_service::create_annotation_type_with_user_agent_context(
        request.into_service_request(),
        MCP_AGENT,
    )
    .await
    .map_err(to_mcp_error)?;

    ToolResult::json(&output).map_err(|error| {
        McpError::internal(format!(
            "Failed to serialize create annotation type response: {error}"
        ))
    })
}

fn input_schema() -> ToolInputSchema {
    ToolInputSchema::from_value(
        serde_json::to_value(schemars::schema_for!(CreateAnnotationTypeToolRequest))
            .expect("create annotation type tool schema serializes"),
    )
}

fn deserialize_request<T>(args: Value, tool_name: &str) -> McpResult<T>
where
    T: for<'de> Deserialize<'de>,
{
    let args = match args {
        Value::Null => Value::Object(Default::default()),
        args => args,
    };

    serde_json::from_value(args).map_err(|error| {
        McpError::invalid_params(format!("Invalid {tool_name} tool arguments: {error}"))
    })
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct CreateAnnotationTypeToolRequest {
    #[schemars(description = "Account slug that will own the annotation type.")]
    account_slug: String,
    #[schemars(description = "Annotation type slug to create within the account.")]
    annotation_type_slug: String,
    #[schemars(
        description = "Validation schema identifier, such as rusl/schemas/kv-storage-policy."
    )]
    schema_identifier: String,
    #[schemars(description = "How annotation content is validated against the schema.")]
    schema_mode: CreateAnnotationTypeToolSchemaMode,
    #[schemars(description = "Optional human-readable annotation type description.")]
    description: Option<String>,
    #[schemars(
        description = "Visibility for the new annotation type. Required to avoid accidental public types."
    )]
    visibility: CreateAnnotationTypeToolVisibility,
    #[schemars(description = "Optional annotation cardinality policy.")]
    cardinality: Option<CreateAnnotationTypeToolCardinality>,
    #[schemars(description = "Required when schema_mode is pinned.")]
    pinned_schema_version_id: Option<String>,
    #[schemars(description = "Whether annotation content becomes immutable after creation.")]
    content_immutable: Option<bool>,
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum CreateAnnotationTypeToolVisibility {
    Public,
    Private,
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum CreateAnnotationTypeToolSchemaMode {
    Current,
    Pinned,
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum CreateAnnotationTypeToolCardinality {
    OnePerSubjectPerAccount,
    ManyPerSubjectPerAccount,
}

impl CreateAnnotationTypeToolRequest {
    fn into_service_request(self) -> CreateAnnotationTypeRequest {
        CreateAnnotationTypeRequest {
            account_slug: self.account_slug,
            annotation_type_slug: self.annotation_type_slug,
            schema_identifier: self.schema_identifier,
            schema_mode: self.schema_mode.into_service_mode(),
            description: self.description,
            visibility: self.visibility.into_service_visibility(),
            cardinality: self.cardinality.map(Into::into),
            pinned_schema_version_id: self.pinned_schema_version_id,
            content_immutable: self.content_immutable,
        }
    }
}

impl CreateAnnotationTypeToolVisibility {
    fn into_service_visibility(self) -> AnnotationTypeVisibility {
        match self {
            Self::Public => AnnotationTypeVisibility::Public,
            Self::Private => AnnotationTypeVisibility::Private,
        }
    }
}

impl CreateAnnotationTypeToolSchemaMode {
    fn into_service_mode(self) -> AnnotationTypeSchemaMode {
        match self {
            Self::Current => AnnotationTypeSchemaMode::Current,
            Self::Pinned => AnnotationTypeSchemaMode::Pinned,
        }
    }
}

impl From<CreateAnnotationTypeToolCardinality> for AnnotationTypeCardinality {
    fn from(value: CreateAnnotationTypeToolCardinality) -> Self {
        match value {
            CreateAnnotationTypeToolCardinality::OnePerSubjectPerAccount => {
                Self::OnePerSubjectPerAccount
            }
            CreateAnnotationTypeToolCardinality::ManyPerSubjectPerAccount => {
                Self::ManyPerSubjectPerAccount
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::definition;

    #[test]
    fn exposes_create_annotation_type_tool() {
        let tool = definition();
        assert_eq!(tool.name, "create_annotation_type");
    }
}
