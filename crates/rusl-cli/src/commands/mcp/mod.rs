mod errors;
mod tools;

use crate::cli::McpArgs;
use anyhow::Result;
use serde_json::Value;
use std::future::Future;
use turbomcp::prelude::{
    McpError, McpHandler, McpHandlerExt, McpResult, Prompt, PromptResult, RequestContext, Resource,
    ResourceResult, ServerInfo, Tool, ToolResult,
};

const MCP_AGENT: &str = "mcp";

pub async fn run(_args: McpArgs) -> Result<()> {
    RuslMcpServer.run_stdio().await?;
    Ok(())
}

#[derive(Debug, Clone)]
struct RuslMcpServer;

impl McpHandler for RuslMcpServer {
    fn server_info(&self) -> ServerInfo {
        ServerInfo::new("rusl", env!("CARGO_PKG_VERSION"))
            .with_description("Search visible Rusl resources, fetch full resource records, inspect examples, manage proposals, and create feedback annotations. Search is a discovery surface; use get_schema, get_bundle, get_annotation_type, or get_annotation when full content is needed.")
    }

    fn list_tools(&self) -> Vec<Tool> {
        let mut tools = vec![tools::search::definition(), tools::endorse::definition()];
        tools.extend(tools::resource::definitions());
        tools.extend(tools::feedback::definitions());
        tools.extend(tools::proposal::definitions());
        tools.push(tools::schema_examples::definition());
        tools
    }

    fn list_resources(&self) -> Vec<Resource> {
        vec![]
    }

    fn list_prompts(&self) -> Vec<Prompt> {
        vec![]
    }

    fn call_tool<'a>(
        &'a self,
        name: &'a str,
        args: Value,
        _ctx: &'a RequestContext,
    ) -> impl Future<Output = McpResult<ToolResult>> + Send + 'a {
        let name = name.to_string();

        async move {
            match name.as_str() {
                "search" => tools::search::call(args).await,
                "endorse" => tools::endorse::call(args).await,
                "list_schema_examples" => tools::schema_examples::call(args).await,
                resource_or_proposal_tool => {
                    if let Some(kind) = tools::resource::kind_for_tool(resource_or_proposal_tool) {
                        tools::resource::call(kind, args).await
                    } else if let Some(kind) =
                        tools::proposal::kind_for_tool(resource_or_proposal_tool)
                    {
                        tools::proposal::call(kind, args).await
                    } else if let Some(kind) =
                        tools::feedback::kind_for_tool(resource_or_proposal_tool)
                    {
                        tools::feedback::call(kind, args).await
                    } else {
                        Err(McpError::tool_not_found(&name))
                    }
                }
            }
        }
    }

    fn read_resource<'a>(
        &'a self,
        uri: &'a str,
        _ctx: &'a RequestContext,
    ) -> impl Future<Output = McpResult<ResourceResult>> + Send + 'a {
        let uri = uri.to_string();

        async move { Err(McpError::resource_not_found(uri)) }
    }

    fn get_prompt<'a>(
        &'a self,
        name: &'a str,
        _args: Option<Value>,
        _ctx: &'a RequestContext,
    ) -> impl Future<Output = McpResult<PromptResult>> + Send + 'a {
        let name = name.to_string();

        async move { Err(McpError::prompt_not_found(name)) }
    }
}

#[cfg(test)]
mod tests {
    use super::RuslMcpServer;
    use turbomcp::prelude::McpHandler;

    #[test]
    fn exposes_search_and_feedback_tools() {
        let server = RuslMcpServer;
        let tools = server.list_tools();
        let search = tools
            .iter()
            .find(|tool| tool.name == "search")
            .expect("search tool is registered");

        assert!(
            search
                .description
                .as_deref()
                .is_some_and(|description| description.contains("does not return complete"))
        );
        assert!(
            search
                .input_schema
                .properties_as_object()
                .is_some_and(|properties| properties.contains_key("query"))
        );

        let context_request = tools
            .iter()
            .find(|tool| tool.name == "create_context_request")
            .expect("context request tool is registered");
        assert!(
            context_request
                .input_schema
                .properties_as_object()
                .is_some_and(|properties| properties.contains_key("failing_task"))
        );

        assert!(tools.iter().any(|tool| tool.name == "endorse"));
        assert!(tools.iter().any(|tool| tool.name == "get_schema"));
        assert!(tools.iter().any(|tool| tool.name == "get_bundle"));
        assert!(tools.iter().any(|tool| tool.name == "get_annotation_type"));
        assert!(tools.iter().any(|tool| tool.name == "get_annotation"));
        assert!(
            tools
                .iter()
                .any(|tool| tool.name == "list_proposal_review_threads")
        );
        assert!(tools.iter().any(|tool| tool.name == "list_schema_examples"));
    }
}
