use crate::commands::mcp::{MCP_AGENT, errors::to_mcp_error};
use rusl_app::schema_example_service::{self, ListSchemaExamplesRequest};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use turbomcp::prelude::{McpError, McpResult, Tool, ToolInputSchema, ToolResult};

const MAX_PER_PAGE: i32 = 100;
const LIST_SCHEMA_EXAMPLES_DESCRIPTION: &str =
    "Use to fetch committed schema examples before using a schema or drafting a proposal.";

pub(in crate::commands::mcp) fn definition() -> Tool {
    Tool::new("list_schema_examples", LIST_SCHEMA_EXAMPLES_DESCRIPTION).with_schema(input_schema())
}

pub(in crate::commands::mcp) async fn call(args: Value) -> McpResult<ToolResult> {
    let request = deserialize_request(args)?;
    let output = schema_example_service::list_schema_examples_with_user_agent_context(
        request.into_service_request()?,
        MCP_AGENT,
    )
    .await
    .map_err(to_mcp_error)?;

    ToolResult::json(&output).map_err(|error| {
        McpError::internal(format!(
            "Failed to serialize list schema examples response: {error}"
        ))
    })
}

fn input_schema() -> ToolInputSchema {
    let schema = schemars::schema_for!(ListSchemaExamplesToolRequest);
    let value = serde_json::to_value(schema).expect("ListSchemaExamplesToolRequest serializes");
    ToolInputSchema::from_value(value)
}

fn deserialize_request(args: Value) -> McpResult<ListSchemaExamplesToolRequest> {
    serde_json::from_value(args).map_err(|error| {
        McpError::invalid_params(format!(
            "Invalid list_schema_examples tool arguments: {error}"
        ))
    })
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct ListSchemaExamplesToolRequest {
    #[schemars(description = "Account slug that owns the schema.")]
    account_slug: String,
    #[schemars(description = "Schema slug whose committed examples should be listed.")]
    schema_slug: String,
    #[schemars(
        description = "Optional committed version filter. Prefixes such as 1 or 1.2 are accepted."
    )]
    version: Option<String>,
    #[schemars(description = "One-based result page.")]
    page: Option<i32>,
    #[schemars(description = "Examples per page. Must be between 1 and 100.")]
    per_page: Option<i32>,
}

impl ListSchemaExamplesToolRequest {
    fn into_service_request(self) -> McpResult<ListSchemaExamplesRequest> {
        Ok(ListSchemaExamplesRequest {
            account_slug: self.account_slug,
            schema_slug: self.schema_slug,
            version: self.version,
            page: positive_page_value("page", self.page)?,
            per_page: per_page_value(self.per_page)?,
        })
    }
}

fn positive_page_value(name: &str, value: Option<i32>) -> McpResult<Option<i32>> {
    if let Some(value) = value
        && value < 1
    {
        return Err(McpError::invalid_params(format!(
            "{name} must be greater than or equal to 1"
        )));
    }

    Ok(value)
}

fn per_page_value(value: Option<i32>) -> McpResult<Option<i32>> {
    let Some(value) = positive_page_value("per_page", value)? else {
        return Ok(None);
    };

    if value > MAX_PER_PAGE {
        return Err(McpError::invalid_params(format!(
            "per_page must be less than or equal to {MAX_PER_PAGE}"
        )));
    }

    Ok(Some(value))
}

#[cfg(test)]
mod tests {
    use super::{definition, per_page_value};

    #[test]
    fn exposes_schema_example_tool_schema() {
        let tool = definition();

        assert_eq!(tool.name, "list_schema_examples");
        assert!(
            tool.description
                .as_deref()
                .is_some_and(|description| description.contains("committed schema examples"))
        );
        assert!(
            tool.input_schema
                .properties_as_object()
                .is_some_and(|properties| properties.contains_key("version"))
        );
    }

    #[test]
    fn rejects_per_page_over_api_max() {
        let error = per_page_value(Some(101)).expect_err("expected max error");

        assert!(error.to_string().contains("per_page"));
    }
}
