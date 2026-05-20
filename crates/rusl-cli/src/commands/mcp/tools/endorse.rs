use crate::commands::mcp::{MCP_AGENT, errors::to_mcp_error};
use rusl_app::annotation_service::{self, EndorseRequest, EndorseTargetType};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use turbomcp::prelude::{McpError, McpResult, Tool, ToolInputSchema, ToolResult};

const ENDORSE_TOOL_DESCRIPTION: &str = "Endorse a Rusl subject as a positive trust signal. The current Rusl API supports annotation endorsements; schema and bundle targets are accepted in the input shape for forward compatibility and return a clear unsupported-target error until backend endpoints exist.";

pub(in crate::commands::mcp) fn definition() -> Tool {
    Tool::new("endorse", ENDORSE_TOOL_DESCRIPTION).with_schema(input_schema())
}

fn input_schema() -> ToolInputSchema {
    let schema = schemars::schema_for!(EndorseToolRequest);
    let value = serde_json::to_value(schema).expect("EndorseToolRequest schema serializes");
    ToolInputSchema::from_value(value)
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
pub(in crate::commands::mcp) struct EndorseToolRequest {
    #[schemars(
        description = "GUID of the subject to endorse. For current annotation endorsements, pass annotations.<id> or the raw annotation id."
    )]
    subject_guid: String,
    #[schemars(
        description = "Optional target type. If omitted, it is inferred from the subject_guid prefix when possible and otherwise defaults to annotation."
    )]
    target_type: Option<EndorseToolTargetType>,
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum EndorseToolTargetType {
    Annotation,
    Schema,
    Bundle,
}

impl EndorseToolTargetType {
    fn into_service_target_type(self) -> EndorseTargetType {
        match self {
            Self::Annotation => EndorseTargetType::Annotation,
            Self::Schema => EndorseTargetType::Schema,
            Self::Bundle => EndorseTargetType::Bundle,
        }
    }
}

pub(in crate::commands::mcp) async fn call(args: Value) -> McpResult<ToolResult> {
    let request = deserialize_request(args)?;
    let output = annotation_service::endorse_with_user_agent_context(
        EndorseRequest {
            target_type: request.target_type(),
            subject_guid: request.subject_guid,
        },
        MCP_AGENT,
    )
    .await
    .map_err(to_mcp_error)?;

    ToolResult::json(&output).map_err(|error| {
        McpError::internal(format!("Failed to serialize endorse response: {error}"))
    })
}

fn deserialize_request(args: Value) -> McpResult<EndorseToolRequest> {
    serde_json::from_value(args).map_err(|error| {
        McpError::invalid_params(format!("Invalid endorse tool arguments: {error}"))
    })
}

impl EndorseToolRequest {
    fn target_type(&self) -> EndorseTargetType {
        self.target_type
            .map(EndorseToolTargetType::into_service_target_type)
            .unwrap_or_else(|| infer_target_type(&self.subject_guid))
    }
}

fn infer_target_type(subject_guid: &str) -> EndorseTargetType {
    if subject_guid.starts_with("schemas.") {
        EndorseTargetType::Schema
    } else if subject_guid.starts_with("bundles.") {
        EndorseTargetType::Bundle
    } else {
        EndorseTargetType::Annotation
    }
}

#[cfg(test)]
mod tests {
    use super::{EndorseTargetType, infer_target_type};

    #[test]
    fn infers_target_type_from_guid_prefix() {
        assert_eq!(infer_target_type("schemas.123"), EndorseTargetType::Schema);
        assert_eq!(infer_target_type("bundles.123"), EndorseTargetType::Bundle);
        assert_eq!(
            infer_target_type("annotations.123"),
            EndorseTargetType::Annotation
        );
        assert_eq!(infer_target_type("raw-id"), EndorseTargetType::Annotation);
    }
}
