use crate::{config, registry::client::RegistryClient};
use anyhow::{Context, Result};
use rusl_api_client::{models, rusl_user_agent_with_context};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchDocumentType {
    Schema,
    AnnotationType,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchView {
    #[default]
    Compact,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchRequest {
    pub query: Option<String>,
    pub types: Vec<SearchDocumentType>,
    pub account_slugs: Vec<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub view: SearchView,
    pub include_metrics: bool,
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            query: None,
            types: Vec::new(),
            account_slugs: Vec::new(),
            page: None,
            per_page: None,
            view: SearchView::Compact,
            include_metrics: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchOutput {
    pub data: Vec<SearchResult>,
    pub facets: Vec<SearchFacet>,
    pub page_info: SearchPageInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    pub guid: String,
    pub identifier: String,
    pub document_type: String,
    pub document_type_label: String,
    pub description: Option<String>,
    pub discovery_profile: Option<HashMap<String, Value>>,
    pub highlights: Vec<HashMap<String, Value>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchFacet {
    pub field: String,
    pub counts: Vec<SearchFacetCount>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchFacetCount {
    pub value: String,
    pub count: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchPageInfo {
    pub current_page: i32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
    pub page_size: i32,
    pub total_count: i32,
    pub total_pages: i32,
}

pub async fn search_registry(request: SearchRequest) -> Result<SearchOutput> {
    let config = config::load().context("Failed to load network configurations")?;
    let search_url = registry_search_url(&config.api_base_url);
    let client = RegistryClient::new(config);
    search_with_client(&client, request)
        .await
        .with_context(|| format!("Failed to search registry at {search_url}"))
}

pub async fn search_registry_with_user_agent_context(
    request: SearchRequest,
    context: &str,
) -> Result<SearchOutput> {
    let config = config::load().context("Failed to load network configurations")?;
    let search_url = registry_search_url(&config.api_base_url);
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent(config, user_agent);
    search_with_client(&client, request)
        .await
        .with_context(|| format!("Failed to search registry at {search_url}"))
}

async fn search_with_client(
    client: &RegistryClient,
    request: SearchRequest,
) -> Result<SearchOutput> {
    let response = client.search(to_api_request(request)).await?;
    Ok(map_search_response(response))
}

fn to_api_request(request: SearchRequest) -> models::GlobalSearchRequest {
    models::GlobalSearchRequest {
        q: request.query.and_then(non_blank),
        types: non_empty(request.types.into_iter().map(map_search_type).collect()),
        account_slugs: non_empty(
            request
                .account_slugs
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        page: request.page,
        per_page: request.per_page,
        discovery_profile_status: None,
        include: if request.include_metrics {
            Some(vec![models::global_search_request::Include::Metrics])
        } else {
            None
        },
        metric_names: None,
        view: Some(map_search_view(request.view)),
    }
}

fn map_search_type(document_type: SearchDocumentType) -> models::global_search_request::Types {
    match document_type {
        SearchDocumentType::Schema => models::global_search_request::Types::Schema,
        SearchDocumentType::AnnotationType => models::global_search_request::Types::AnnotationType,
    }
}

fn map_search_view(view: SearchView) -> models::global_search_request::View {
    match view {
        SearchView::Compact => models::global_search_request::View::Compact,
        SearchView::Full => models::global_search_request::View::Full,
    }
}

fn map_search_response(response: models::SearchResponse) -> SearchOutput {
    SearchOutput {
        data: response.data.into_iter().map(map_search_result).collect(),
        facets: response.facets.into_iter().map(map_search_facet).collect(),
        page_info: map_page_info(*response.page_info),
    }
}

fn map_search_result(result: models::SearchResult) -> SearchResult {
    SearchResult {
        guid: result.guid,
        identifier: result.identifier,
        document_type: match result.document_type {
            models::search_result::DocumentType::Schema => "schema",
            models::search_result::DocumentType::AnnotationType => "annotation_type",
        }
        .to_string(),
        document_type_label: result.document_type_label,
        description: result.description.flatten(),
        discovery_profile: result.discovery_profile.flatten(),
        highlights: result.highlights,
    }
}

fn map_search_facet(facet: models::SearchFacet) -> SearchFacet {
    SearchFacet {
        field: facet.field,
        counts: facet
            .counts
            .into_iter()
            .map(map_search_facet_count)
            .collect(),
    }
}

fn map_search_facet_count(count: models::SearchFacetCountsInner) -> SearchFacetCount {
    SearchFacetCount {
        value: count.value,
        count: count.count,
    }
}

fn map_page_info(page_info: models::SearchPageInfo) -> SearchPageInfo {
    SearchPageInfo {
        current_page: page_info.current_page,
        has_next_page: page_info.has_next_page,
        has_previous_page: page_info.has_previous_page,
        page_size: page_info.page_size,
        total_count: page_info.total_count,
        total_pages: page_info.total_pages,
    }
}

fn non_blank(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn non_empty<T>(values: Vec<T>) -> Option<Vec<T>> {
    if values.is_empty() {
        None
    } else {
        Some(values)
    }
}

fn registry_search_url(api_base_url: &str) -> String {
    format!("{}/api/search", api_base_url.trim_end_matches('/'))
}

#[cfg(test)]
mod tests {
    use super::{
        SearchDocumentType, SearchRequest, SearchView, map_search_response, registry_search_url,
        to_api_request,
    };
    use rusl_api_client::models;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn builds_global_search_request_with_supported_filters() {
        let request = to_api_request(SearchRequest {
            query: Some("  bearing ".to_string()),
            types: vec![
                SearchDocumentType::Schema,
                SearchDocumentType::AnnotationType,
            ],
            account_slugs: vec![" hassox ".to_string(), " ".to_string()],
            page: Some(2),
            per_page: Some(25),
            view: SearchView::Compact,
            include_metrics: false,
        });

        assert_eq!(request.q.as_deref(), Some("bearing"));
        assert_eq!(request.account_slugs, Some(vec!["hassox".to_string()]));
        assert_eq!(
            request.types,
            Some(vec![
                models::global_search_request::Types::Schema,
                models::global_search_request::Types::AnnotationType,
            ])
        );
        assert_eq!(request.page, Some(2));
        assert_eq!(request.per_page, Some(25));
        assert_eq!(
            request.view,
            Some(models::global_search_request::View::Compact)
        );
        assert_eq!(request.include, None);
    }

    #[test]
    fn builds_full_search_request_when_requested() {
        let request = to_api_request(SearchRequest {
            query: Some("payment".to_string()),
            view: SearchView::Full,
            ..SearchRequest::default()
        });

        assert_eq!(request.q.as_deref(), Some("payment"));
        assert_eq!(
            request.view,
            Some(models::global_search_request::View::Full)
        );
        assert_eq!(request.include, None);
    }

    #[test]
    fn includes_metrics_when_requested() {
        let request = to_api_request(SearchRequest {
            include_metrics: true,
            ..SearchRequest::default()
        });

        assert_eq!(
            request.include,
            Some(vec![models::global_search_request::Include::Metrics])
        );
        assert_eq!(
            request.view,
            Some(models::global_search_request::View::Compact)
        );
    }

    #[test]
    fn builds_registry_search_url_without_duplicate_slashes() {
        assert_eq!(
            registry_search_url("https://hassox.rusl-api.ngrok.io/"),
            "https://hassox.rusl-api.ngrok.io/api/search"
        );
    }

    #[test]
    fn maps_search_response_into_stable_output_shape() {
        let mut discovery_profile = HashMap::new();
        discovery_profile.insert("account_slug".to_string(), json!("hassox"));
        let response = models::SearchResponse::new(
            vec![models::SearchResult {
                description: Some(Some("Shared primitives".to_string())),
                discovery_profile: Some(Some(discovery_profile)),
                document_type: models::search_result::DocumentType::Schema,
                document_type_label: "Schema".to_string(),
                guid: "schema_guid".to_string(),
                highlights: Vec::new(),
                identifier: "hassox/common".to_string(),
            }],
            vec![models::SearchFacet::new(
                vec![models::SearchFacetCountsInner::new(1, "schema".to_string())],
                "document_type".to_string(),
            )],
            models::SearchPageInfo::new(1, false, false, 10, 1, 1),
        );

        let output = map_search_response(response);

        assert_eq!(output.data[0].identifier, "hassox/common");
        assert_eq!(
            output.data[0].description.as_deref(),
            Some("Shared primitives")
        );
        assert_eq!(output.data[0].document_type, "schema");
        assert_eq!(
            output.data[0]
                .discovery_profile
                .as_ref()
                .and_then(|profile| profile.get("account_slug")),
            Some(&json!("hassox"))
        );
        assert_eq!(output.facets[0].counts[0].value, "schema");
        assert_eq!(output.page_info.total_count, 1);
    }
}
