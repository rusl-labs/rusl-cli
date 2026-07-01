use crate::commands::mcp::{MCP_AGENT, errors::to_mcp_error};
use rusl_app::bundle_service::{
    self, BundleVisibility, CreateBundleRequest, CreateBundleVersionRequest,
    PublishBundleVersionRequest,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use turbomcp::prelude::{McpError, McpResult, Tool, ToolInputSchema, ToolResult};

#[derive(Debug, Clone, Copy)]
struct BundleToolSpec {
    name: &'static str,
    description: &'static str,
    kind: BundleToolKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::commands::mcp) enum BundleToolKind {
    CreateBundle,
    CreateBundleVersion,
    PublishBundleVersion,
}

const BUNDLE_TOOLS: &[BundleToolSpec] = &[
    BundleToolSpec {
        name: "create_bundle",
        description: "Use when the user wants to create a missing bundle namespace before publishing bundle content. Requires user authorization and explicit public/private visibility.",
        kind: BundleToolKind::CreateBundle,
    },
    BundleToolSpec {
        name: "create_bundle_version",
        description: "Use to create a draft bundle version with manifest content. Requires an existing bundle and user authorization.",
        kind: BundleToolKind::CreateBundleVersion,
    },
    BundleToolSpec {
        name: "publish_bundle_version",
        description: "Use to publish a draft bundle version after validating its manifest. Requires user authorization.",
        kind: BundleToolKind::PublishBundleVersion,
    },
];

pub(in crate::commands::mcp) fn definitions() -> Vec<Tool> {
    BUNDLE_TOOLS
        .iter()
        .map(|spec| Tool::new(spec.name, spec.description).with_schema(input_schema(spec.kind)))
        .collect()
}

pub(in crate::commands::mcp) fn kind_for_tool(name: &str) -> Option<BundleToolKind> {
    BUNDLE_TOOLS
        .iter()
        .find(|spec| spec.name == name)
        .map(|spec| spec.kind)
}

pub(in crate::commands::mcp) async fn call(
    kind: BundleToolKind,
    args: Value,
) -> McpResult<ToolResult> {
    match kind {
        BundleToolKind::CreateBundle => {
            let request: CreateBundleToolRequest = deserialize_request(args, "create_bundle")?;
            let output = bundle_service::create_bundle_with_user_agent_context(
                request.into_service_request(),
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "create bundle")
        }
        BundleToolKind::CreateBundleVersion => {
            let request: CreateBundleVersionToolRequest =
                deserialize_request(args, "create_bundle_version")?;
            let output = bundle_service::create_bundle_version_with_user_agent_context(
                request.into_service_request(),
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "create bundle version")
        }
        BundleToolKind::PublishBundleVersion => {
            let request: PublishBundleVersionToolRequest =
                deserialize_request(args, "publish_bundle_version")?;
            let output = bundle_service::publish_bundle_version_with_user_agent_context(
                request.into_service_request(),
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "publish bundle version")
        }
    }
}

fn input_schema(kind: BundleToolKind) -> ToolInputSchema {
    let value = match kind {
        BundleToolKind::CreateBundle => schema_for::<CreateBundleToolRequest>(),
        BundleToolKind::CreateBundleVersion => schema_for::<CreateBundleVersionToolRequest>(),
        BundleToolKind::PublishBundleVersion => schema_for::<PublishBundleVersionToolRequest>(),
    };

    ToolInputSchema::from_value(value)
}

fn schema_for<T: JsonSchema>() -> Value {
    serde_json::to_value(schemars::schema_for!(T)).expect("bundle tool schema serializes")
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
struct CreateBundleToolRequest {
    #[schemars(description = "Account slug that will own the bundle.")]
    account_slug: String,
    #[schemars(description = "Bundle slug to create within the account.")]
    bundle_slug: String,
    #[schemars(description = "Optional human-readable bundle description.")]
    description: Option<String>,
    #[schemars(
        description = "Visibility for the new bundle. Required to avoid accidental public bundles."
    )]
    visibility: CreateBundleToolVisibility,
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum CreateBundleToolVisibility {
    Public,
    Private,
}

impl CreateBundleToolVisibility {
    fn into_service_visibility(self) -> BundleVisibility {
        match self {
            Self::Public => BundleVisibility::Public,
            Self::Private => BundleVisibility::Private,
        }
    }
}

impl CreateBundleToolRequest {
    fn into_service_request(self) -> CreateBundleRequest {
        CreateBundleRequest {
            account_slug: self.account_slug,
            bundle_slug: self.bundle_slug,
            description: self.description,
            visibility: self.visibility.into_service_visibility(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct CreateBundleVersionToolRequest {
    #[schemars(description = "Account slug that owns the bundle.")]
    account_slug: String,
    #[schemars(description = "Bundle slug receiving the draft version.")]
    bundle_slug: String,
    #[schemars(description = "SemVer version string for the draft.")]
    version: String,
    #[schemars(description = "Bundle manifest content as JSON or TOML text.")]
    manifest: String,
    #[schemars(description = "Optional release notes for this version.")]
    description: Option<String>,
}

impl CreateBundleVersionToolRequest {
    fn into_service_request(self) -> CreateBundleVersionRequest {
        CreateBundleVersionRequest {
            account_slug: self.account_slug,
            bundle_slug: self.bundle_slug,
            version: self.version,
            manifest: self.manifest,
            description: self.description,
        }
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct PublishBundleVersionToolRequest {
    #[schemars(description = "Account slug that owns the bundle.")]
    account_slug: String,
    #[schemars(description = "Bundle slug containing the draft version.")]
    bundle_slug: String,
    #[schemars(description = "SemVer version string to publish.")]
    version: String,
}

impl PublishBundleVersionToolRequest {
    fn into_service_request(self) -> PublishBundleVersionRequest {
        PublishBundleVersionRequest {
            account_slug: self.account_slug,
            bundle_slug: self.bundle_slug,
            version: self.version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{definitions, kind_for_tool};

    #[test]
    fn exposes_bundle_management_tools() {
        let tools = definitions();
        let tool_names = tools
            .iter()
            .map(|tool| tool.name.as_str())
            .collect::<Vec<_>>();

        assert!(tool_names.contains(&"create_bundle"));
        assert!(tool_names.contains(&"create_bundle_version"));
        assert!(tool_names.contains(&"publish_bundle_version"));
        assert_eq!(
            kind_for_tool("create_bundle"),
            Some(super::BundleToolKind::CreateBundle)
        );
    }
}
