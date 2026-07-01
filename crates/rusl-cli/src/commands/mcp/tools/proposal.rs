use crate::commands::mcp::{MCP_AGENT, errors::to_mcp_error};
use rusl_app::proposal_service::{
    self, AcceptSchemaProposalRequest, CreateProposalReviewThreadRequest,
    CreateSchemaProposalRequest, CreateSchemaRequest, ExampleDataInput,
    GetProposalReviewThreadRequest, GetSchemaProposalRequest, ProposalReviewThreadsRequest,
    ReplyToProposalReviewThreadRequest, SchemaVisibility, UpdateSchemaProposalRequest,
};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use turbomcp::prelude::{McpError, McpResult, Tool, ToolInputSchema, ToolResult};

#[derive(Debug, Clone, Copy)]
struct ProposalToolSpec {
    name: &'static str,
    description: &'static str,
    kind: ProposalToolKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::commands::mcp) enum ProposalToolKind {
    CreateSchema,
    CreateSchemaProposal,
    GetSchemaProposal,
    UpdateSchemaProposal,
    ListReviewThreads,
    GetReviewThread,
    CreateReviewThread,
    ReplyToReviewThread,
    AcceptSchemaProposal,
}

const PROPOSAL_TOOLS: &[ProposalToolSpec] = &[
    ProposalToolSpec {
        name: "create_schema",
        description: "Use when the user wants to create a missing schema namespace before proposing schema content. Requires user authorization and explicit public/private visibility.",
        kind: ProposalToolKind::CreateSchema,
    },
    ProposalToolSpec {
        name: "create_schema_proposal",
        description: "Use to propose a new version for an existing schema. Submit complete JSON Schema content and valid examples for review.",
        kind: ProposalToolKind::CreateSchemaProposal,
    },
    ProposalToolSpec {
        name: "get_schema_proposal",
        description: "Use before revising a proposal to read its current editable JSON Schema content, valid examples, status, and version metadata.",
        kind: ProposalToolKind::GetSchemaProposal,
    },
    ProposalToolSpec {
        name: "update_schema_proposal",
        description: "Use after review feedback to replace a pending proposal with revised complete JSON Schema content and valid examples. Read the proposal first when preserving existing fields.",
        kind: ProposalToolKind::UpdateSchemaProposal,
    },
    ProposalToolSpec {
        name: "list_proposal_review_threads",
        description: "Use to inspect proposal review state before commenting or revising. Comment bodies are omitted by default; set include_comments only when the conversation text is needed.",
        kind: ProposalToolKind::ListReviewThreads,
    },
    ProposalToolSpec {
        name: "get_proposal_review_thread",
        description: "Use when one review thread needs full context, including comments, before replying or updating the proposal.",
        kind: ProposalToolKind::GetReviewThread,
    },
    ProposalToolSpec {
        name: "create_proposal_review_thread",
        description: "Use to start a new top-level review conversation on a schema proposal. Requires user authorization.",
        kind: ProposalToolKind::CreateReviewThread,
    },
    ProposalToolSpec {
        name: "reply_to_proposal_review_thread",
        description: "Use to respond in an existing proposal review thread after reading the relevant thread context. Omit parent_comment_id to reply to the root comment.",
        kind: ProposalToolKind::ReplyToReviewThread,
    },
    ProposalToolSpec {
        name: "accept_schema_proposal",
        description: "Use after review to accept a pending schema proposal and publish its content as a new schema version. Requires user authorization.",
        kind: ProposalToolKind::AcceptSchemaProposal,
    },
];

pub(in crate::commands::mcp) fn definitions() -> Vec<Tool> {
    PROPOSAL_TOOLS
        .iter()
        .map(|spec| Tool::new(spec.name, spec.description).with_schema(input_schema(spec.kind)))
        .collect()
}

pub(in crate::commands::mcp) fn kind_for_tool(name: &str) -> Option<ProposalToolKind> {
    PROPOSAL_TOOLS
        .iter()
        .find(|spec| spec.name == name)
        .map(|spec| spec.kind)
}

pub(in crate::commands::mcp) async fn call(
    kind: ProposalToolKind,
    args: Value,
) -> McpResult<ToolResult> {
    match kind {
        ProposalToolKind::CreateSchema => {
            let request: CreateSchemaToolRequest = deserialize_request(args, "create_schema")?;
            let output = proposal_service::create_schema_with_user_agent_context(
                request.into_service_request(),
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "create schema")
        }
        ProposalToolKind::CreateSchemaProposal => {
            let request: CreateSchemaProposalToolRequest =
                deserialize_request(args, "create_schema_proposal")?;
            let output = proposal_service::create_schema_proposal_with_user_agent_context(
                request.into_service_request()?,
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "create schema proposal")
        }
        ProposalToolKind::GetSchemaProposal => {
            let request: GetSchemaProposalToolRequest =
                deserialize_request(args, "get_schema_proposal")?;
            let output = proposal_service::get_schema_proposal_with_user_agent_context(
                request.into_service_request()?,
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "get schema proposal")
        }
        ProposalToolKind::UpdateSchemaProposal => {
            let request: UpdateSchemaProposalToolRequest =
                deserialize_request(args, "update_schema_proposal")?;
            let output = proposal_service::update_schema_proposal_with_user_agent_context(
                request.into_service_request()?,
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "update schema proposal")
        }
        ProposalToolKind::ListReviewThreads => {
            let request: ListProposalReviewThreadsToolRequest =
                deserialize_request(args, "list_proposal_review_threads")?;
            let output = proposal_service::list_proposal_review_threads_with_user_agent_context(
                request.into_service_request()?,
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "list proposal review threads")
        }
        ProposalToolKind::GetReviewThread => {
            let request: GetProposalReviewThreadToolRequest =
                deserialize_request(args, "get_proposal_review_thread")?;
            let output = proposal_service::get_proposal_review_thread_with_user_agent_context(
                request.into_service_request()?,
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "get proposal review thread")
        }
        ProposalToolKind::CreateReviewThread => {
            let request: CreateProposalReviewThreadToolRequest =
                deserialize_request(args, "create_proposal_review_thread")?;
            let output = proposal_service::create_proposal_review_thread_with_user_agent_context(
                request.into_service_request()?,
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "create proposal review thread")
        }
        ProposalToolKind::ReplyToReviewThread => {
            let request: ReplyToProposalReviewThreadToolRequest =
                deserialize_request(args, "reply_to_proposal_review_thread")?;
            let output = proposal_service::reply_to_proposal_review_thread_with_user_agent_context(
                request.into_service_request()?,
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "reply to proposal review thread")
        }
        ProposalToolKind::AcceptSchemaProposal => {
            let request: AcceptSchemaProposalToolRequest =
                deserialize_request(args, "accept_schema_proposal")?;
            let output = proposal_service::accept_schema_proposal_with_user_agent_context(
                request.into_service_request()?,
                MCP_AGENT,
            )
            .await
            .map_err(to_mcp_error)?;
            json_result(&output, "accept schema proposal")
        }
    }
}

fn input_schema(kind: ProposalToolKind) -> ToolInputSchema {
    let value = match kind {
        ProposalToolKind::CreateSchema => schema_for::<CreateSchemaToolRequest>(),
        ProposalToolKind::CreateSchemaProposal => schema_for::<CreateSchemaProposalToolRequest>(),
        ProposalToolKind::GetSchemaProposal => schema_for::<GetSchemaProposalToolRequest>(),
        ProposalToolKind::UpdateSchemaProposal => schema_for::<UpdateSchemaProposalToolRequest>(),
        ProposalToolKind::ListReviewThreads => schema_for::<ListProposalReviewThreadsToolRequest>(),
        ProposalToolKind::GetReviewThread => schema_for::<GetProposalReviewThreadToolRequest>(),
        ProposalToolKind::CreateReviewThread => {
            schema_for::<CreateProposalReviewThreadToolRequest>()
        }
        ProposalToolKind::ReplyToReviewThread => {
            schema_for::<ReplyToProposalReviewThreadToolRequest>()
        }
        ProposalToolKind::AcceptSchemaProposal => schema_for::<AcceptSchemaProposalToolRequest>(),
    };

    ToolInputSchema::from_value(value)
}

fn schema_for<T: JsonSchema>() -> Value {
    serde_json::to_value(schemars::schema_for!(T)).expect("proposal tool schema serializes")
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
struct CreateSchemaToolRequest {
    #[schemars(description = "Account slug that will own the schema.")]
    account_slug: String,
    #[schemars(description = "Schema slug to create within the account.")]
    schema_slug: String,
    #[schemars(description = "Optional human-readable schema description.")]
    description: Option<String>,
    #[schemars(
        description = "Visibility for the new schema. Required to avoid accidental public schemas."
    )]
    visibility: CreateSchemaToolVisibility,
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum CreateSchemaToolVisibility {
    Public,
    Private,
}

impl CreateSchemaToolVisibility {
    fn into_service_visibility(self) -> SchemaVisibility {
        match self {
            Self::Public => SchemaVisibility::Public,
            Self::Private => SchemaVisibility::Private,
        }
    }
}

impl CreateSchemaToolRequest {
    fn into_service_request(self) -> CreateSchemaRequest {
        CreateSchemaRequest {
            account_slug: self.account_slug,
            schema_slug: self.schema_slug,
            description: self.description,
            visibility: self.visibility.into_service_visibility(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct CreateSchemaProposalToolRequest {
    #[schemars(description = "Account slug that owns the schema.")]
    account_slug: String,
    #[schemars(description = "Schema slug receiving the proposal.")]
    schema_slug: String,
    #[schemars(description = "JSON Schema proposal content.")]
    content: Value,
    #[schemars(description = "Optional proposal description.")]
    description: Option<String>,
    #[serde(default)]
    #[schemars(
        description = "Valid example data for the proposed schema. Defaults to an empty list."
    )]
    valid_data: Vec<ExampleDataToolInput>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct ExampleDataToolInput {
    #[schemars(
        description = "Example JSON data that should validate against the proposed schema."
    )]
    data: Value,
    #[schemars(description = "Optional title for this example.")]
    title: Option<String>,
}

impl CreateSchemaProposalToolRequest {
    fn into_service_request(self) -> McpResult<CreateSchemaProposalRequest> {
        Ok(CreateSchemaProposalRequest {
            account_slug: self.account_slug,
            schema_slug: self.schema_slug,
            content: self.content,
            description: self.description,
            valid_data: self.valid_data.into_iter().map(Into::into).collect(),
        })
    }
}

impl From<ExampleDataToolInput> for ExampleDataInput {
    fn from(input: ExampleDataToolInput) -> Self {
        Self {
            data: input.data,
            title: input.title,
        }
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct GetSchemaProposalToolRequest {
    #[schemars(description = "Account slug that owns the schema.")]
    account_slug: String,
    #[schemars(description = "Schema slug for the proposal.")]
    schema_slug: String,
    #[schemars(description = "Proposal number within the schema.")]
    proposal_number: i32,
}

impl GetSchemaProposalToolRequest {
    fn into_service_request(self) -> McpResult<GetSchemaProposalRequest> {
        Ok(GetSchemaProposalRequest {
            account_slug: self.account_slug,
            schema_slug: self.schema_slug,
            proposal_number: positive_proposal_number(self.proposal_number)?,
        })
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct UpdateSchemaProposalToolRequest {
    #[schemars(description = "Account slug that owns the schema.")]
    account_slug: String,
    #[schemars(description = "Schema slug for the proposal.")]
    schema_slug: String,
    #[schemars(description = "Proposal number within the schema.")]
    proposal_number: i32,
    #[schemars(
        description = "Complete replacement JSON Schema content for the proposal. Use get_schema_proposal first when preserving existing fields."
    )]
    content: Value,
    #[schemars(description = "Optional revised proposal description.")]
    description: Option<String>,
    #[schemars(
        description = "Required complete replacement set of valid examples for the proposed schema."
    )]
    valid_data: Vec<ExampleDataToolInput>,
}

impl UpdateSchemaProposalToolRequest {
    fn into_service_request(self) -> McpResult<UpdateSchemaProposalRequest> {
        Ok(UpdateSchemaProposalRequest {
            account_slug: self.account_slug,
            schema_slug: self.schema_slug,
            proposal_number: positive_proposal_number(self.proposal_number)?,
            content: self.content,
            description: self.description,
            valid_data: self.valid_data.into_iter().map(Into::into).collect(),
        })
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct ListProposalReviewThreadsToolRequest {
    #[schemars(description = "Account slug that owns the schema.")]
    account_slug: String,
    #[schemars(description = "Schema slug for the proposal.")]
    schema_slug: String,
    #[schemars(description = "Proposal number within the schema.")]
    proposal_number: i32,
    #[serde(default)]
    #[schemars(
        description = "Include full review comments. Defaults to false to conserve context."
    )]
    include_comments: bool,
}

impl ListProposalReviewThreadsToolRequest {
    fn into_service_request(self) -> McpResult<ProposalReviewThreadsRequest> {
        Ok(ProposalReviewThreadsRequest {
            account_slug: self.account_slug,
            schema_slug: self.schema_slug,
            proposal_number: positive_proposal_number(self.proposal_number)?,
            include_comments: self.include_comments,
        })
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct GetProposalReviewThreadToolRequest {
    #[schemars(description = "Account slug that owns the schema.")]
    account_slug: String,
    #[schemars(description = "Schema slug for the proposal.")]
    schema_slug: String,
    #[schemars(description = "Proposal number within the schema.")]
    proposal_number: i32,
    #[schemars(description = "Review thread ID to return.")]
    thread_id: String,
}

impl GetProposalReviewThreadToolRequest {
    fn into_service_request(self) -> McpResult<GetProposalReviewThreadRequest> {
        Ok(GetProposalReviewThreadRequest {
            account_slug: self.account_slug,
            schema_slug: self.schema_slug,
            proposal_number: positive_proposal_number(self.proposal_number)?,
            thread_id: self.thread_id,
        })
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct CreateProposalReviewThreadToolRequest {
    #[schemars(description = "Account slug that owns the schema.")]
    account_slug: String,
    #[schemars(description = "Schema slug for the proposal.")]
    schema_slug: String,
    #[schemars(description = "Proposal number within the schema.")]
    proposal_number: i32,
    #[schemars(description = "Markdown body for the root review comment.")]
    body_text: String,
    #[schemars(description = "Optional structured metadata for the review comment body.")]
    body_meta: Option<Value>,
}

impl CreateProposalReviewThreadToolRequest {
    fn into_service_request(self) -> McpResult<CreateProposalReviewThreadRequest> {
        Ok(CreateProposalReviewThreadRequest {
            account_slug: self.account_slug,
            schema_slug: self.schema_slug,
            proposal_number: positive_proposal_number(self.proposal_number)?,
            body_text: self.body_text,
            body_meta: self.body_meta,
        })
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct ReplyToProposalReviewThreadToolRequest {
    #[schemars(description = "Account slug that owns the schema.")]
    account_slug: String,
    #[schemars(description = "Schema slug for the proposal.")]
    schema_slug: String,
    #[schemars(description = "Proposal number within the schema.")]
    proposal_number: i32,
    #[schemars(description = "Review thread ID to reply to.")]
    thread_id: String,
    #[schemars(
        description = "Parent comment ID. If omitted, Rusl resolves the root comment for the thread."
    )]
    parent_comment_id: Option<String>,
    #[schemars(description = "Markdown body for the reply.")]
    body_text: String,
    #[schemars(description = "Optional structured metadata for the reply body.")]
    body_meta: Option<Value>,
}

impl ReplyToProposalReviewThreadToolRequest {
    fn into_service_request(self) -> McpResult<ReplyToProposalReviewThreadRequest> {
        Ok(ReplyToProposalReviewThreadRequest {
            account_slug: self.account_slug,
            schema_slug: self.schema_slug,
            proposal_number: positive_proposal_number(self.proposal_number)?,
            thread_id: self.thread_id,
            parent_comment_id: self.parent_comment_id,
            body_text: self.body_text,
            body_meta: self.body_meta,
        })
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema)]
struct AcceptSchemaProposalToolRequest {
    #[schemars(description = "Account slug that owns the schema.")]
    account_slug: String,
    #[schemars(description = "Schema slug for the proposal.")]
    schema_slug: String,
    #[schemars(description = "Proposal number within the schema.")]
    proposal_number: i32,
    #[schemars(
        description = "Optional version override. Must be greater than or equal to the proposal's proposed version."
    )]
    version: Option<String>,
    #[schemars(description = "Optional description override for the created schema version.")]
    description: Option<String>,
}

impl AcceptSchemaProposalToolRequest {
    fn into_service_request(self) -> McpResult<AcceptSchemaProposalRequest> {
        Ok(AcceptSchemaProposalRequest {
            account_slug: self.account_slug,
            schema_slug: self.schema_slug,
            proposal_number: positive_proposal_number(self.proposal_number)?,
            version: self.version,
            description: self.description,
        })
    }
}

fn positive_proposal_number(value: i32) -> McpResult<i32> {
    if value < 1 {
        return Err(McpError::invalid_params(
            "proposal_number must be greater than or equal to 1",
        ));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{UpdateSchemaProposalToolRequest, definitions, deserialize_request, kind_for_tool};
    use serde_json::json;

    #[test]
    fn exposes_proposal_management_tools() {
        let tools = definitions();
        let tool_names = tools
            .iter()
            .map(|tool| tool.name.as_str())
            .collect::<Vec<_>>();

        assert!(tool_names.contains(&"create_schema"));
        assert!(tool_names.contains(&"create_schema_proposal"));
        assert!(tool_names.contains(&"get_schema_proposal"));
        assert!(tool_names.contains(&"update_schema_proposal"));
        assert!(tool_names.contains(&"list_proposal_review_threads"));
        assert!(tool_names.contains(&"get_proposal_review_thread"));
        assert!(tool_names.contains(&"create_proposal_review_thread"));
        assert!(tool_names.contains(&"reply_to_proposal_review_thread"));
        assert!(tool_names.contains(&"accept_schema_proposal"));
        assert!(kind_for_tool("list_proposal_review_threads").is_some());
    }

    #[test]
    fn update_proposal_requires_replacement_valid_data() {
        let error = deserialize_request::<UpdateSchemaProposalToolRequest>(
            json!({
                "account_slug": "hassox",
                "schema_slug": "sample",
                "proposal_number": 1,
                "content": {"type": "object"}
            }),
            "update_schema_proposal",
        )
        .expect_err("expected valid_data to be required");

        assert!(error.to_string().contains("valid_data"));
    }
}
