mod errors;
mod tools;

use crate::cli::McpArgs;
use anyhow::Result;
use rmcp::{
    ErrorData, ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, Implementation, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router,
};

const MCP_USER_AGENT_CONTEXT: &str = "mcp";

pub async fn run(_args: McpArgs) -> Result<()> {
    let service = RuslMcpServer::new().serve(rmcp::transport::stdio()).await?;
    let _reason = service.waiting().await?;
    Ok(())
}

#[derive(Debug, Clone)]
struct RuslMcpServer {
    tool_router: ToolRouter<Self>,
}

impl RuslMcpServer {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

#[tool_router(router = tool_router)]
impl RuslMcpServer {
    #[tool(
        name = "search",
        description = "Search visible Rusl resources, including schemas, bundles, annotation types, and annotations, using compact responses by default. Set identifiers to exact canonical resource identifiers when resolving known resources. Set view to full only when the user explicitly asks for full search data or large embedded fields are truly needed. Set include_metrics when popularity or discoverability signals are needed."
    )]
    async fn search(
        &self,
        Parameters(request): Parameters<tools::search::SearchToolRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        tools::search::call(request).await
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for RuslMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("rusl", env!("CARGO_PKG_VERSION")))
            .with_instructions("Use the search tool to find visible Rusl resources.")
    }
}

#[cfg(test)]
mod tests {
    use super::RuslMcpServer;

    #[test]
    fn exposes_search_tool() {
        let server = RuslMcpServer::new();
        let tools = server.tool_router.list_all();
        let search = tools
            .iter()
            .find(|tool| tool.name == "search")
            .expect("search tool is registered");

        assert!(
            search
                .description
                .as_deref()
                .is_some_and(|description| description.contains("compact responses by default"))
        );
        assert!(search.input_schema.contains_key("properties"));
    }
}
