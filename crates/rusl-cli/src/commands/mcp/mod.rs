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
            .with_description("Search visible Rusl resources, fetch full resource records, inspect examples, manage proposals and bundles, register annotation types, and create feedback annotations. Search is a discovery surface; use get_schema, get_bundle, get_annotation_type, or get_annotation when full content is needed.")
    }

    fn list_tools(&self) -> Vec<Tool> {
        let mut tools = vec![tools::search::definition(), tools::endorse::definition()];
        tools.extend(tools::resource::definitions());
        tools.extend(tools::feedback::definitions());
        tools.extend(tools::proposal::definitions());
        tools.extend(tools::bundle::definitions());
        tools.push(tools::annotation_type::definition());
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
                        tools::bundle::kind_for_tool(resource_or_proposal_tool)
                    {
                        tools::bundle::call(kind, args).await
                    } else if tools::annotation_type::is_tool(resource_or_proposal_tool) {
                        tools::annotation_type::call(args).await
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
    use serial_test::serial;
    use std::{ffi::OsString, path::PathBuf};
    use tempfile::TempDir;
    use turbomcp::prelude::McpHandler;

    const CONTEXT_REQUEST_SCHEMA: &str = r#"{
  "$id": "https://resources.rusl.com/resources/rusl/schemas/context-request",
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "additionalProperties": false,
  "description": "A request for better explanation when a subject is ambiguous enough to block a task.",
  "properties": {
    "error_received": {
      "description": "Optional exact validation, tool, or runtime error.",
      "type": "string"
    },
    "failing_task": {
      "description": "Task that was blocked or made unsafe by missing context.",
      "minLength": 1,
      "type": "string"
    },
    "suspected_ambiguity": {
      "description": "Question or point of confusion that should be clarified.",
      "minLength": 1,
      "type": "string"
    }
  },
  "required": ["failing_task", "suspected_ambiguity"],
  "title": "Context Request",
  "type": "object",
  "version": "0.1.0"
}"#;

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
                schema_dir.join("context-request.schema.json"),
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
    fn exposes_search_and_feedback_tools() {
        let _guard = SchemaWorkspaceGuard::new();

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
