use crate::commands::mcp::{MCP_USER_AGENT_CONTEXT, errors::to_mcp_error};
use rmcp::{ErrorData, model::CallToolResult, schemars};
use rusl_app::search_service::{self, SearchDocumentType, SearchView};
use serde::Deserialize;

const MAX_PER_PAGE: i32 = 100;

#[derive(Debug, Clone, Default, Deserialize, schemars::JsonSchema)]
pub(in crate::commands::mcp) struct SearchToolRequest {
    #[schemars(description = "Search text. Omit to return all visible results.")]
    query: Option<String>,
    #[schemars(description = "Restrict results to schema and/or annotation_type documents.")]
    types: Option<Vec<SearchToolType>>,
    #[schemars(description = "Restrict results to these account slugs.")]
    account_slugs: Option<Vec<String>>,
    #[schemars(description = "One-based result page.")]
    page: Option<i32>,
    #[schemars(description = "Results per page. Must be between 1 and 100.")]
    per_page: Option<i32>,
    #[schemars(
        description = "Response shape. Defaults to compact to conserve context; use full only when explicitly needed."
    )]
    view: Option<SearchToolView>,
    #[schemars(
        description = "Include popularity and discoverability metrics with compact results when ranking signals are needed."
    )]
    include_metrics: Option<bool>,
}

pub(in crate::commands::mcp) async fn call(
    request: SearchToolRequest,
) -> Result<CallToolResult, ErrorData> {
    let output = search_service::search_registry_with_user_agent_context(
        request.into_search_request()?,
        MCP_USER_AGENT_CONTEXT,
    )
    .await
    .map_err(to_mcp_error)?;
    let value = serde_json::to_value(output).map_err(|error| {
        ErrorData::internal_error(
            format!("Failed to serialize search response: {error}"),
            None,
        )
    })?;

    Ok(CallToolResult::structured(value))
}

impl SearchToolRequest {
    fn into_search_request(self) -> Result<search_service::SearchRequest, ErrorData> {
        Ok(search_service::SearchRequest {
            query: self.query,
            types: self
                .types
                .unwrap_or_default()
                .into_iter()
                .map(SearchToolType::into_search_document_type)
                .collect(),
            account_slugs: self.account_slugs.unwrap_or_default(),
            page: positive_page_value("page", self.page)?,
            per_page: per_page_value(self.per_page)?,
            view: self.view.unwrap_or_default().into_search_view(),
            include_metrics: self.include_metrics.unwrap_or(false),
        })
    }
}

#[derive(Debug, Clone, Copy, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolType {
    Schema,
    AnnotationType,
}

impl SearchToolType {
    fn into_search_document_type(self) -> SearchDocumentType {
        match self {
            SearchToolType::Schema => SearchDocumentType::Schema,
            SearchToolType::AnnotationType => SearchDocumentType::AnnotationType,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolView {
    #[default]
    Compact,
    Full,
}

impl SearchToolView {
    fn into_search_view(self) -> SearchView {
        match self {
            SearchToolView::Compact => SearchView::Compact,
            SearchToolView::Full => SearchView::Full,
        }
    }
}

fn positive_page_value(name: &str, value: Option<i32>) -> Result<Option<i32>, ErrorData> {
    if let Some(value) = value
        && value < 1
    {
        return Err(ErrorData::invalid_params(
            format!("{name} must be greater than zero"),
            None,
        ));
    }

    Ok(value)
}

fn per_page_value(value: Option<i32>) -> Result<Option<i32>, ErrorData> {
    let value = positive_page_value("per_page", value)?;
    if let Some(value) = value
        && value > MAX_PER_PAGE
    {
        return Err(ErrorData::invalid_params(
            format!("per_page must be less than or equal to {MAX_PER_PAGE}"),
            None,
        ));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{SearchToolRequest, SearchToolView};
    use rusl_app::search_service::SearchView;

    #[test]
    fn search_tool_defaults_to_compact_without_metrics() {
        let request = SearchToolRequest::default()
            .into_search_request()
            .expect("convert default search request");

        assert_eq!(request.view, SearchView::Compact);
        assert!(!request.include_metrics);
    }

    #[test]
    fn search_tool_allows_full_view_when_requested() {
        let request = SearchToolRequest {
            view: Some(SearchToolView::Full),
            ..SearchToolRequest::default()
        }
        .into_search_request()
        .expect("convert full search request");

        assert_eq!(request.view, SearchView::Full);
    }

    #[test]
    fn search_tool_can_include_metrics() {
        let request = SearchToolRequest {
            include_metrics: Some(true),
            ..SearchToolRequest::default()
        }
        .into_search_request()
        .expect("convert metrics search request");

        assert!(request.include_metrics);
        assert_eq!(request.view, SearchView::Compact);
    }

    #[test]
    fn rejects_non_positive_page_values() {
        let error = SearchToolRequest {
            page: Some(0),
            ..SearchToolRequest::default()
        }
        .into_search_request()
        .expect_err("invalid page");

        assert_eq!(error.message, "page must be greater than zero");
    }

    #[test]
    fn rejects_per_page_over_api_max() {
        let error = SearchToolRequest {
            per_page: Some(101),
            ..SearchToolRequest::default()
        }
        .into_search_request()
        .expect_err("invalid per_page");

        assert_eq!(error.message, "per_page must be less than or equal to 100");
    }
}
