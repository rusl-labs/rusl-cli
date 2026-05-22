use crate::commands::mcp::{MCP_AGENT, errors::to_mcp_error};
use rusl_app::search_service::{self, SearchDocumentType, SearchView};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;
use turbomcp::prelude::{McpError, McpResult, Tool, ToolInputSchema, ToolResult};

const MAX_PER_PAGE: i32 = 100;
const SEARCH_TOOL_DESCRIPTION: &str = "Search visible Rusl resources, including schemas, bundles, annotation types, and annotations, using compact responses by default. Set identifiers to exact canonical resource identifiers when resolving known resources. Set view to full only when the user explicitly asks for full search data or large embedded fields are truly needed. Set include_metrics when popularity or discoverability signals are needed.";

pub(in crate::commands::mcp) fn definition() -> Tool {
    Tool::new("search", SEARCH_TOOL_DESCRIPTION).with_schema(input_schema())
}

fn input_schema() -> ToolInputSchema {
    let schema = schemars::schema_for!(SearchToolRequest);
    let value = serde_json::to_value(schema).expect("SearchToolRequest schema serializes");
    ToolInputSchema::from_value(value)
}

#[derive(Debug, Clone, Default, Deserialize, JsonSchema)]
pub(in crate::commands::mcp) struct SearchToolRequest {
    #[schemars(description = "Search text. Omit to return all visible results.")]
    query: Option<String>,
    #[schemars(
        description = "Restrict results to schema, bundle, annotation_type, and/or annotation documents."
    )]
    types: Option<Vec<SearchToolType>>,
    #[schemars(description = "Restrict results to exact canonical resource identifiers.")]
    identifiers: Option<Vec<String>>,
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
    #[schemars(description = "Restrict global search by discovery profile status.")]
    discovery_profile_status: Option<SearchToolDiscoveryProfileStatus>,
    #[schemars(
        description = "Restrict global search to resources that have any of these metric names."
    )]
    metric_names: Option<Vec<String>>,
    #[schemars(
        description = "Type-specific identifier prefix for schema, bundle, or annotation_type search. Requires exactly one matching type."
    )]
    identifier_prefix: Option<String>,
    #[schemars(
        description = "Type-specific resource lifecycle status for schema, bundle, or annotation_type search."
    )]
    status: Option<SearchToolResourceStatus>,
    #[schemars(description = "Type-specific current version status for schema or bundle search.")]
    current_version_status: Option<SearchToolVersionStatus>,
    #[schemars(
        description = "Type-specific JSON root instance types for schema search, such as object or array."
    )]
    current_version_root_instance_types: Option<Vec<SearchToolJsonRootInstanceType>>,
    #[schemars(description = "Type-specific schema format for schema search.")]
    schema_format: Option<SearchToolSchemaFormat>,
    #[schemars(description = "Type-specific sort for bundle search.")]
    bundle_sort: Option<SearchToolBundleSort>,
    #[schemars(description = "Type-specific cardinality for annotation_type search.")]
    annotation_type_cardinality: Option<SearchToolAnnotationTypeCardinality>,
    #[schemars(description = "Type-specific lifecycle status for annotation search.")]
    annotation_status: Option<SearchToolAnnotationStatus>,
    #[schemars(description = "Type-specific sort for annotation search.")]
    annotation_sort: Option<SearchToolAnnotationSort>,
    #[schemars(description = "Restrict annotation search to registered annotation type GUIDs.")]
    annotation_type_guids: Option<Vec<String>>,
    #[schemars(description = "Restrict annotation search to annotations set by these user GUIDs.")]
    set_by_user_guids: Option<Vec<String>>,
    #[schemars(description = "Restrict annotation search by annotated subject account slugs.")]
    subject_account_slugs: Option<Vec<String>>,
    #[schemars(description = "Restrict annotation search to annotated subject GUIDs.")]
    subject_guids: Option<Vec<String>>,
    #[schemars(description = "Restrict annotation search by annotated subject identifier prefix.")]
    subject_identifier_prefix: Option<String>,
    #[schemars(description = "Restrict annotation search by annotated subject types.")]
    subject_types: Option<Vec<SearchToolAnnotationSubjectType>>,
    #[schemars(
        description = "Restrict annotation search to registered annotation type identifiers."
    )]
    type_identifiers: Option<Vec<String>>,
}

pub(in crate::commands::mcp) async fn call(args: Value) -> McpResult<ToolResult> {
    let request = deserialize_request(args)?;
    let output = search_service::search_registry_with_user_agent_context(
        request.into_search_request()?,
        MCP_AGENT,
    )
    .await
    .map_err(to_mcp_error)?;

    ToolResult::json(&output).map_err(|error| {
        McpError::internal(format!("Failed to serialize search response: {error}"))
    })
}

fn deserialize_request(args: Value) -> McpResult<SearchToolRequest> {
    let args = match args {
        Value::Null => Value::Object(Default::default()),
        args => args,
    };

    serde_json::from_value(args).map_err(|error| {
        McpError::invalid_params(format!("Invalid search tool arguments: {error}"))
    })
}

impl SearchToolRequest {
    fn into_search_request(self) -> McpResult<search_service::SearchRequest> {
        Ok(search_service::SearchRequest {
            query: self.query,
            types: self
                .types
                .unwrap_or_default()
                .into_iter()
                .map(SearchToolType::into_search_document_type)
                .collect(),
            identifiers: self.identifiers.unwrap_or_default(),
            account_slugs: self.account_slugs.unwrap_or_default(),
            page: positive_page_value("page", self.page)?,
            per_page: per_page_value(self.per_page)?,
            view: self.view.unwrap_or_default().into_search_view(),
            include_metrics: self.include_metrics.unwrap_or(false),
            discovery_profile_status: self
                .discovery_profile_status
                .map(SearchToolDiscoveryProfileStatus::into_search_status),
            metric_names: self.metric_names.unwrap_or_default(),
            identifier_prefix: self.identifier_prefix,
            status: self
                .status
                .map(SearchToolResourceStatus::into_search_status),
            current_version_status: self
                .current_version_status
                .map(SearchToolVersionStatus::into_search_status),
            current_version_root_instance_types: self
                .current_version_root_instance_types
                .unwrap_or_default()
                .into_iter()
                .map(SearchToolJsonRootInstanceType::into_search_instance_type)
                .collect(),
            schema_format: self
                .schema_format
                .map(SearchToolSchemaFormat::into_search_schema_format),
            bundle_sort: self.bundle_sort.map(SearchToolBundleSort::into_search_sort),
            annotation_type_cardinality: self
                .annotation_type_cardinality
                .map(SearchToolAnnotationTypeCardinality::into_search_cardinality),
            annotation_status: self
                .annotation_status
                .map(SearchToolAnnotationStatus::into_search_status),
            annotation_sort: self
                .annotation_sort
                .map(SearchToolAnnotationSort::into_search_sort),
            annotation_type_guids: self.annotation_type_guids.unwrap_or_default(),
            set_by_user_guids: self.set_by_user_guids.unwrap_or_default(),
            subject_account_slugs: self.subject_account_slugs.unwrap_or_default(),
            subject_guids: self.subject_guids.unwrap_or_default(),
            subject_identifier_prefix: self.subject_identifier_prefix,
            subject_types: self
                .subject_types
                .unwrap_or_default()
                .into_iter()
                .map(SearchToolAnnotationSubjectType::into_search_subject_type)
                .collect(),
            type_identifiers: self.type_identifiers.unwrap_or_default(),
        })
    }
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolType {
    Schema,
    Bundle,
    AnnotationType,
    Annotation,
}

impl SearchToolType {
    fn into_search_document_type(self) -> SearchDocumentType {
        match self {
            SearchToolType::Schema => SearchDocumentType::Schema,
            SearchToolType::Bundle => SearchDocumentType::Bundle,
            SearchToolType::AnnotationType => SearchDocumentType::AnnotationType,
            SearchToolType::Annotation => SearchDocumentType::Annotation,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Deserialize, JsonSchema)]
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

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolDiscoveryProfileStatus {
    Pending,
    Ready,
    Failed,
}

impl SearchToolDiscoveryProfileStatus {
    fn into_search_status(self) -> search_service::DiscoveryProfileStatus {
        match self {
            SearchToolDiscoveryProfileStatus::Pending => {
                search_service::DiscoveryProfileStatus::Pending
            }
            SearchToolDiscoveryProfileStatus::Ready => {
                search_service::DiscoveryProfileStatus::Ready
            }
            SearchToolDiscoveryProfileStatus::Failed => {
                search_service::DiscoveryProfileStatus::Failed
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolResourceStatus {
    Active,
    Archived,
}

impl SearchToolResourceStatus {
    fn into_search_status(self) -> search_service::ResourceStatus {
        match self {
            SearchToolResourceStatus::Active => search_service::ResourceStatus::Active,
            SearchToolResourceStatus::Archived => search_service::ResourceStatus::Archived,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolVersionStatus {
    Draft,
    Active,
    Deprecated,
    Yanked,
}

impl SearchToolVersionStatus {
    fn into_search_status(self) -> search_service::VersionStatus {
        match self {
            SearchToolVersionStatus::Draft => search_service::VersionStatus::Draft,
            SearchToolVersionStatus::Active => search_service::VersionStatus::Active,
            SearchToolVersionStatus::Deprecated => search_service::VersionStatus::Deprecated,
            SearchToolVersionStatus::Yanked => search_service::VersionStatus::Yanked,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolJsonRootInstanceType {
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
}

impl SearchToolJsonRootInstanceType {
    fn into_search_instance_type(self) -> search_service::JsonRootInstanceType {
        match self {
            SearchToolJsonRootInstanceType::Array => search_service::JsonRootInstanceType::Array,
            SearchToolJsonRootInstanceType::Boolean => {
                search_service::JsonRootInstanceType::Boolean
            }
            SearchToolJsonRootInstanceType::Integer => {
                search_service::JsonRootInstanceType::Integer
            }
            SearchToolJsonRootInstanceType::Null => search_service::JsonRootInstanceType::Null,
            SearchToolJsonRootInstanceType::Number => search_service::JsonRootInstanceType::Number,
            SearchToolJsonRootInstanceType::Object => search_service::JsonRootInstanceType::Object,
            SearchToolJsonRootInstanceType::String => search_service::JsonRootInstanceType::String,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolSchemaFormat {
    JsonSchema,
}

impl SearchToolSchemaFormat {
    fn into_search_schema_format(self) -> search_service::SchemaFormat {
        match self {
            SearchToolSchemaFormat::JsonSchema => search_service::SchemaFormat::JsonSchema,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolBundleSort {
    Relevance,
    Dependencies,
}

impl SearchToolBundleSort {
    fn into_search_sort(self) -> search_service::BundleSort {
        match self {
            SearchToolBundleSort::Relevance => search_service::BundleSort::Relevance,
            SearchToolBundleSort::Dependencies => search_service::BundleSort::Dependencies,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolAnnotationTypeCardinality {
    OnePerSubjectPerAccount,
    ManyPerSubjectPerAccount,
}

impl SearchToolAnnotationTypeCardinality {
    fn into_search_cardinality(self) -> search_service::AnnotationTypeCardinality {
        match self {
            SearchToolAnnotationTypeCardinality::OnePerSubjectPerAccount => {
                search_service::AnnotationTypeCardinality::OnePerSubjectPerAccount
            }
            SearchToolAnnotationTypeCardinality::ManyPerSubjectPerAccount => {
                search_service::AnnotationTypeCardinality::ManyPerSubjectPerAccount
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolAnnotationStatus {
    Active,
    Deprecated,
    Revoked,
}

impl SearchToolAnnotationStatus {
    fn into_search_status(self) -> search_service::AnnotationStatus {
        match self {
            SearchToolAnnotationStatus::Active => search_service::AnnotationStatus::Active,
            SearchToolAnnotationStatus::Deprecated => search_service::AnnotationStatus::Deprecated,
            SearchToolAnnotationStatus::Revoked => search_service::AnnotationStatus::Revoked,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolAnnotationSort {
    Relevance,
    Endorsements,
}

impl SearchToolAnnotationSort {
    fn into_search_sort(self) -> search_service::AnnotationSort {
        match self {
            SearchToolAnnotationSort::Relevance => search_service::AnnotationSort::Relevance,
            SearchToolAnnotationSort::Endorsements => search_service::AnnotationSort::Endorsements,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
enum SearchToolAnnotationSubjectType {
    Annotations,
    BundleVersions,
    Bundles,
    SchemaProposals,
    SchemaVersions,
    Schemas,
}

impl SearchToolAnnotationSubjectType {
    fn into_search_subject_type(self) -> search_service::AnnotationSubjectType {
        match self {
            SearchToolAnnotationSubjectType::Annotations => {
                search_service::AnnotationSubjectType::Annotations
            }
            SearchToolAnnotationSubjectType::BundleVersions => {
                search_service::AnnotationSubjectType::BundleVersions
            }
            SearchToolAnnotationSubjectType::Bundles => {
                search_service::AnnotationSubjectType::Bundles
            }
            SearchToolAnnotationSubjectType::SchemaProposals => {
                search_service::AnnotationSubjectType::SchemaProposals
            }
            SearchToolAnnotationSubjectType::SchemaVersions => {
                search_service::AnnotationSubjectType::SchemaVersions
            }
            SearchToolAnnotationSubjectType::Schemas => {
                search_service::AnnotationSubjectType::Schemas
            }
        }
    }
}

fn positive_page_value(name: &str, value: Option<i32>) -> McpResult<Option<i32>> {
    if let Some(value) = value
        && value < 1
    {
        return Err(McpError::invalid_params(format!(
            "{name} must be greater than zero"
        )));
    }

    Ok(value)
}

fn per_page_value(value: Option<i32>) -> McpResult<Option<i32>> {
    let value = positive_page_value("per_page", value)?;
    if let Some(value) = value
        && value > MAX_PER_PAGE
    {
        return Err(McpError::invalid_params(format!(
            "per_page must be less than or equal to {MAX_PER_PAGE}"
        )));
    }

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{SearchToolRequest, SearchToolView, deserialize_request, input_schema};
    use rusl_app::search_service::{
        AnnotationSort, AnnotationStatus, AnnotationSubjectType, DiscoveryProfileStatus,
        JsonRootInstanceType, ResourceStatus, SearchView, VersionStatus,
    };
    use serde_json::json;

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

    #[test]
    fn search_tool_deserializes_flat_arguments() {
        let request = deserialize_request(json!({
            "query": "brake",
            "types": ["schema"],
            "view": "full",
            "include_metrics": true,
            "discovery_profile_status": "ready",
            "metric_names": ["watchers"],
            "identifier_prefix": "hassox/",
            "status": "active",
            "current_version_status": "deprecated",
            "current_version_root_instance_types": ["object"]
        }))
        .expect("deserialize search tool arguments")
        .into_search_request()
        .expect("convert search request");

        assert_eq!(request.query.as_deref(), Some("brake"));
        assert_eq!(request.types.len(), 1);
        assert_eq!(request.view, SearchView::Full);
        assert!(request.include_metrics);
        assert_eq!(
            request.discovery_profile_status,
            Some(DiscoveryProfileStatus::Ready)
        );
        assert_eq!(request.metric_names, vec!["watchers".to_string()]);
        assert_eq!(request.identifier_prefix.as_deref(), Some("hassox/"));
        assert_eq!(request.status, Some(ResourceStatus::Active));
        assert_eq!(
            request.current_version_status,
            Some(VersionStatus::Deprecated)
        );
        assert_eq!(
            request.current_version_root_instance_types,
            vec![JsonRootInstanceType::Object]
        );
    }

    #[test]
    fn search_tool_deserializes_annotation_filters() {
        let request = deserialize_request(json!({
            "types": ["annotation"],
            "annotation_status": "active",
            "annotation_sort": "endorsements",
            "annotation_type_guids": ["type-guid"],
            "set_by_user_guids": ["user-guid"],
            "subject_account_slugs": ["hassox"],
            "subject_guids": ["subject-guid"],
            "subject_identifier_prefix": "hassox/common",
            "subject_types": ["schemas"],
            "type_identifiers": ["hassox/review"]
        }))
        .expect("deserialize search tool arguments")
        .into_search_request()
        .expect("convert search request");

        assert_eq!(request.annotation_status, Some(AnnotationStatus::Active));
        assert_eq!(request.annotation_sort, Some(AnnotationSort::Endorsements));
        assert_eq!(request.annotation_type_guids, vec!["type-guid".to_string()]);
        assert_eq!(request.set_by_user_guids, vec!["user-guid".to_string()]);
        assert_eq!(request.subject_account_slugs, vec!["hassox".to_string()]);
        assert_eq!(request.subject_guids, vec!["subject-guid".to_string()]);
        assert_eq!(
            request.subject_identifier_prefix.as_deref(),
            Some("hassox/common")
        );
        assert_eq!(request.subject_types, vec![AnnotationSubjectType::Schemas]);
        assert_eq!(request.type_identifiers, vec!["hassox/review".to_string()]);
    }

    #[test]
    fn search_tool_schema_exposes_flat_properties() {
        let schema = input_schema();
        let properties = schema.properties_as_object().expect("schema properties");

        assert!(properties.contains_key("query"));
        assert!(properties.contains_key("types"));
        assert!(properties.contains_key("include_metrics"));
        assert!(properties.contains_key("identifier_prefix"));
        assert!(properties.contains_key("bundle_sort"));
        assert!(properties.contains_key("annotation_sort"));
    }
}
