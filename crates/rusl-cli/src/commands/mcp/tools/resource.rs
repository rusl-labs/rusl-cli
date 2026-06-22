use crate::commands::mcp::{MCP_AGENT, errors::to_mcp_error};
use rusl_app::resource_service::{
    self, GetAnnotationRecordRequest, GetAnnotationTypeRecordRequest, GetBundleRecordRequest,
    GetSchemaRecordRequest,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use turbomcp::prelude::{McpError, McpResult, Tool, ToolInputSchema, ToolResult};

#[derive(Debug, Clone, Copy)]
struct ResourceToolSpec {
    name: &'static str,
    description: &'static str,
    kind: ResourceToolKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::commands::mcp) enum ResourceToolKind {
    Schema,
    Bundle,
    AnnotationType,
    Annotation,
}

const RESOURCE_TOOLS: &[ResourceToolSpec] = &[
    ResourceToolSpec {
        name: "get_schema",
        description: "Use after search when the full schema record is needed. Fetches the API schema show record by canonical identifier; set version to fetch a specific schema version record with JSON Schema content.",
        kind: ResourceToolKind::Schema,
    },
    ResourceToolSpec {
        name: "get_bundle",
        description: "Use after search when the full bundle record is needed. Fetches the API bundle show record by canonical identifier; set version to fetch a specific bundle version record with manifest content.",
        kind: ResourceToolKind::Bundle,
    },
    ResourceToolSpec {
        name: "get_annotation_type",
        description: "Use after search when the full registered annotation type record is needed, including validation schema linkage and cardinality metadata.",
        kind: ResourceToolKind::AnnotationType,
    },
    ResourceToolSpec {
        name: "get_annotation",
        description: "Use after search when the full annotation record is needed, including annotation content. Accepts either the raw annotation ID or an annotations.<id> GUID.",
        kind: ResourceToolKind::Annotation,
    },
];

pub(in crate::commands::mcp) fn definitions() -> Vec<Tool> {
    RESOURCE_TOOLS
        .iter()
        .map(|spec| Tool::new(spec.name, spec.description).with_schema(input_schema(spec.kind)))
        .collect()
}

pub(in crate::commands::mcp) fn kind_for_tool(name: &str) -> Option<ResourceToolKind> {
    RESOURCE_TOOLS
        .iter()
        .find(|spec| spec.name == name)
        .map(|spec| spec.kind)
}

pub(in crate::commands::mcp) async fn call(
    kind: ResourceToolKind,
    args: Value,
) -> McpResult<ToolResult> {
    match kind {
        ResourceToolKind::Schema => {
            let request: GetSchemaToolRequest = deserialize_request(args, "get_schema")?;
            let output = resource_service::get_schema_record_with_user_agent_context(
                request.into_service_request(),
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "get schema")
        }
        ResourceToolKind::Bundle => {
            let request: GetBundleToolRequest = deserialize_request(args, "get_bundle")?;
            let output = resource_service::get_bundle_record_with_user_agent_context(
                request.into_service_request(),
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "get bundle")
        }
        ResourceToolKind::AnnotationType => {
            let request: GetAnnotationTypeToolRequest =
                deserialize_request(args, "get_annotation_type")?;
            let output = resource_service::get_annotation_type_record_with_user_agent_context(
                request.into_service_request(),
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "get annotation type")
        }
        ResourceToolKind::Annotation => {
            let request: GetAnnotationToolRequest = deserialize_request(args, "get_annotation")?;
            let output = resource_service::get_annotation_record_with_user_agent_context(
                request.into_service_request()?,
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "get annotation")
        }
    }
}

fn input_schema(kind: ResourceToolKind) -> ToolInputSchema {
    let value = match kind {
        ResourceToolKind::Schema => schema_for::<GetSchemaToolRequest>(),
        ResourceToolKind::Bundle => schema_for::<GetBundleToolRequest>(),
        ResourceToolKind::AnnotationType => schema_for::<GetAnnotationTypeToolRequest>(),
        ResourceToolKind::Annotation => schema_for::<GetAnnotationToolRequest>(),
    };

    ToolInputSchema::from_value(value)
}

fn schema_for<T: JsonSchema>() -> Value {
    serde_json::to_value(schemars::schema_for!(T)).expect("resource tool schema serializes")
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

fn json_result<T: serde::Serialize>(output: &T, label: &str) -> McpResult<ToolResult> {
    ToolResult::json(output).map_err(|error| {
        McpError::internal(format!("Failed to serialize {label} response: {error}"))
    })
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct GetSchemaToolRequest {
    #[schemars(description = "Canonical schema identifier in account/schemas/slug form.")]
    identifier: String,
    #[schemars(
        description = "Optional semantic version. When set, fetches /api/{account}/schemas/{slug}/versions/{version}; v-prefixes are accepted."
    )]
    version: Option<String>,
}

impl GetSchemaToolRequest {
    fn into_service_request(self) -> GetSchemaRecordRequest {
        GetSchemaRecordRequest {
            identifier: self.identifier,
            version: self.version,
        }
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct GetBundleToolRequest {
    #[schemars(description = "Canonical bundle identifier in account/bundles/slug form.")]
    identifier: String,
    #[schemars(
        description = "Optional semantic version. When set, fetches /api/{account}/bundles/{slug}/versions/{version}; v-prefixes are accepted."
    )]
    version: Option<String>,
}

impl GetBundleToolRequest {
    fn into_service_request(self) -> GetBundleRecordRequest {
        GetBundleRecordRequest {
            identifier: self.identifier,
            version: self.version,
        }
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct GetAnnotationTypeToolRequest {
    #[schemars(
        description = "Canonical annotation type identifier in account/annotation-types/slug form."
    )]
    identifier: String,
}

impl GetAnnotationTypeToolRequest {
    fn into_service_request(self) -> GetAnnotationTypeRecordRequest {
        GetAnnotationTypeRecordRequest {
            identifier: self.identifier,
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
struct GetAnnotationToolRequest {
    #[schemars(description = "Raw annotation ID. Use either id or guid.")]
    id: Option<String>,
    #[schemars(description = "Annotation GUID in annotations.<id> form. Use either id or guid.")]
    guid: Option<String>,
}

impl GetAnnotationToolRequest {
    fn into_service_request(self) -> McpResult<GetAnnotationRecordRequest> {
        match (self.id, self.guid) {
            (Some(id), None) => Ok(GetAnnotationRecordRequest { id }),
            (None, Some(guid)) => Ok(GetAnnotationRecordRequest { id: guid }),
            (None, None) => Err(McpError::invalid_params("id or guid is required")),
            (Some(_), Some(_)) => Err(McpError::invalid_params("provide only one of id or guid")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ResourceToolKind, deserialize_request, input_schema, kind_for_tool};
    use serde_json::json;

    #[test]
    fn exposes_resource_tool_kinds() {
        assert_eq!(kind_for_tool("get_schema"), Some(ResourceToolKind::Schema));
        assert_eq!(kind_for_tool("get_bundle"), Some(ResourceToolKind::Bundle));
        assert_eq!(
            kind_for_tool("get_annotation_type"),
            Some(ResourceToolKind::AnnotationType)
        );
        assert_eq!(
            kind_for_tool("get_annotation"),
            Some(ResourceToolKind::Annotation)
        );
        assert_eq!(kind_for_tool("search"), None);
    }

    #[test]
    fn resource_tool_schemas_expose_expected_inputs() {
        let schema = input_schema(ResourceToolKind::Schema);
        let properties = schema.properties_as_object().expect("schema properties");
        assert!(properties.contains_key("identifier"));
        assert!(properties.contains_key("version"));

        let annotation = input_schema(ResourceToolKind::Annotation);
        let properties = annotation
            .properties_as_object()
            .expect("annotation schema properties");
        assert!(properties.contains_key("id"));
        assert!(properties.contains_key("guid"));
    }

    #[test]
    fn annotation_tool_accepts_id_or_guid_but_not_both() {
        deserialize_request::<super::GetAnnotationToolRequest>(
            json!({ "id": "123" }),
            "get_annotation",
        )
        .expect("id request")
        .into_service_request()
        .expect("id conversion");

        deserialize_request::<super::GetAnnotationToolRequest>(
            json!({ "guid": "annotations.123" }),
            "get_annotation",
        )
        .expect("guid request")
        .into_service_request()
        .expect("guid conversion");

        let error = deserialize_request::<super::GetAnnotationToolRequest>(
            json!({ "id": "123", "guid": "annotations.123" }),
            "get_annotation",
        )
        .expect("deserialize both")
        .into_service_request()
        .expect_err("both should fail");
        assert!(error.to_string().contains("provide only one"));
    }
}
