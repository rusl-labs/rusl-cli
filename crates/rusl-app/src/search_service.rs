use crate::{config, registry::client::RegistryClient};
use anyhow::{Context, Result, bail};
use rusl_api_client::{models, rusl_user_agent_with_context};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchDocumentType {
    Schema,
    Bundle,
    AnnotationType,
    Annotation,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchView {
    #[default]
    Compact,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryProfileStatus {
    Pending,
    Ready,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceStatus {
    Active,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionStatus {
    Draft,
    Active,
    Deprecated,
    Yanked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JsonRootInstanceType {
    Array,
    Boolean,
    Integer,
    Null,
    Number,
    Object,
    String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaFormat {
    JsonSchema,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleSort {
    Relevance,
    Dependencies,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationTypeCardinality {
    OnePerSubjectPerAccount,
    ManyPerSubjectPerAccount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationStatus {
    Active,
    Deprecated,
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationSort {
    Relevance,
    Endorsements,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationSubjectType {
    Annotations,
    BundleVersions,
    Bundles,
    SchemaProposals,
    SchemaVersions,
    Schemas,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchRequest {
    pub query: Option<String>,
    pub types: Vec<SearchDocumentType>,
    pub identifiers: Vec<String>,
    pub account_slugs: Vec<String>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub view: SearchView,
    pub include_metrics: bool,
    pub discovery_profile_status: Option<DiscoveryProfileStatus>,
    pub metric_names: Vec<String>,
    pub identifier_prefix: Option<String>,
    pub status: Option<ResourceStatus>,
    pub current_version_status: Option<VersionStatus>,
    pub current_version_root_instance_types: Vec<JsonRootInstanceType>,
    pub schema_format: Option<SchemaFormat>,
    pub bundle_sort: Option<BundleSort>,
    pub annotation_type_cardinality: Option<AnnotationTypeCardinality>,
    pub annotation_status: Option<AnnotationStatus>,
    pub annotation_sort: Option<AnnotationSort>,
    pub annotation_type_guids: Vec<String>,
    pub set_by_user_guids: Vec<String>,
    pub subject_account_slugs: Vec<String>,
    pub subject_guids: Vec<String>,
    pub subject_identifier_prefix: Option<String>,
    pub subject_types: Vec<AnnotationSubjectType>,
    pub type_identifiers: Vec<String>,
}

impl Default for SearchRequest {
    fn default() -> Self {
        Self {
            query: None,
            types: Vec::new(),
            identifiers: Vec::new(),
            account_slugs: Vec::new(),
            page: None,
            per_page: None,
            view: SearchView::Compact,
            include_metrics: false,
            discovery_profile_status: None,
            metric_names: Vec::new(),
            identifier_prefix: None,
            status: None,
            current_version_status: None,
            current_version_root_instance_types: Vec::new(),
            schema_format: None,
            bundle_sort: None,
            annotation_type_cardinality: None,
            annotation_status: None,
            annotation_sort: None,
            annotation_type_guids: Vec::new(),
            set_by_user_guids: Vec::new(),
            subject_account_slugs: Vec::new(),
            subject_guids: Vec::new(),
            subject_identifier_prefix: None,
            subject_types: Vec::new(),
            type_identifiers: Vec::new(),
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
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    search_with_client(&client, request)
        .await
        .with_context(|| format!("Failed to search registry at {search_url}"))
}

async fn search_with_client(
    client: &RegistryClient,
    request: SearchRequest,
) -> Result<SearchOutput> {
    let endpoint = search_endpoint(&request)?;
    let response = match endpoint {
        SearchEndpoint::Global => client.search(to_global_api_request(request)).await?,
        SearchEndpoint::Schemas => {
            client
                .search_schemas(to_schema_api_request(request))
                .await?
        }
        SearchEndpoint::Bundles => {
            client
                .search_bundles(to_bundle_api_request(request))
                .await?
        }
        SearchEndpoint::AnnotationTypes => {
            client
                .search_annotation_types(to_annotation_type_api_request(request))
                .await?
        }
        SearchEndpoint::Annotations => {
            client
                .search_annotations(to_annotation_api_request(request))
                .await?
        }
    };
    Ok(map_search_response(response))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SearchEndpoint {
    Global,
    Schemas,
    Bundles,
    AnnotationTypes,
    Annotations,
}

fn search_endpoint(request: &SearchRequest) -> Result<SearchEndpoint> {
    if !has_type_specific_filters(request) {
        return Ok(SearchEndpoint::Global);
    }

    if has_global_only_filters(request) {
        bail!(
            "discovery_profile_status and metric_names cannot be combined with type-specific search filters"
        );
    }

    if request.types.len() != 1 {
        bail!("type-specific search filters require exactly one search type");
    }

    match request.types[0] {
        SearchDocumentType::Schema => {
            validate_schema_filters(request).map(|()| SearchEndpoint::Schemas)
        }
        SearchDocumentType::Bundle => {
            validate_bundle_filters(request).map(|()| SearchEndpoint::Bundles)
        }
        SearchDocumentType::AnnotationType => {
            validate_annotation_type_filters(request).map(|()| SearchEndpoint::AnnotationTypes)
        }
        SearchDocumentType::Annotation => {
            validate_annotation_filters(request).map(|()| SearchEndpoint::Annotations)
        }
    }
}

fn has_type_specific_filters(request: &SearchRequest) -> bool {
    request
        .identifier_prefix
        .as_ref()
        .is_some_and(|value| is_non_blank(value))
        || request.status.is_some()
        || request.current_version_status.is_some()
        || !request.current_version_root_instance_types.is_empty()
        || request.schema_format.is_some()
        || request.bundle_sort.is_some()
        || request.annotation_type_cardinality.is_some()
        || request.annotation_status.is_some()
        || request.annotation_sort.is_some()
        || !request.annotation_type_guids.is_empty()
        || !request.set_by_user_guids.is_empty()
        || !request.subject_account_slugs.is_empty()
        || !request.subject_guids.is_empty()
        || request
            .subject_identifier_prefix
            .as_ref()
            .is_some_and(|value| is_non_blank(value))
        || !request.subject_types.is_empty()
        || !request.type_identifiers.is_empty()
}

fn has_global_only_filters(request: &SearchRequest) -> bool {
    request.discovery_profile_status.is_some()
        || request.metric_names.iter().any(|value| is_non_blank(value))
}

fn validate_schema_filters(request: &SearchRequest) -> Result<()> {
    if request.bundle_sort.is_some() {
        bail!("bundle_sort is only valid for bundle search");
    }
    if request.annotation_type_cardinality.is_some() {
        bail!("annotation_type_cardinality is only valid for annotation_type search");
    }
    if has_annotation_only_filters(request) {
        bail!("annotation filters are only valid for annotation search");
    }
    Ok(())
}

fn validate_bundle_filters(request: &SearchRequest) -> Result<()> {
    if !request.current_version_root_instance_types.is_empty() || request.schema_format.is_some() {
        bail!("schema filters are only valid for schema search");
    }
    if request.annotation_type_cardinality.is_some() {
        bail!("annotation_type_cardinality is only valid for annotation_type search");
    }
    if has_annotation_only_filters(request) {
        bail!("annotation filters are only valid for annotation search");
    }
    Ok(())
}

fn validate_annotation_type_filters(request: &SearchRequest) -> Result<()> {
    if request.current_version_status.is_some()
        || !request.current_version_root_instance_types.is_empty()
        || request.schema_format.is_some()
    {
        bail!("schema and version filters are not valid for annotation_type search");
    }
    if request.bundle_sort.is_some() {
        bail!("bundle_sort is only valid for bundle search");
    }
    if has_annotation_only_filters(request) {
        bail!("annotation filters are only valid for annotation search");
    }
    Ok(())
}

fn validate_annotation_filters(request: &SearchRequest) -> Result<()> {
    if request
        .identifier_prefix
        .as_ref()
        .is_some_and(|value| is_non_blank(value))
        || request.status.is_some()
        || request.current_version_status.is_some()
        || !request.current_version_root_instance_types.is_empty()
        || request.schema_format.is_some()
    {
        bail!("resource, schema, and version filters are not valid for annotation search");
    }
    if request.bundle_sort.is_some() {
        bail!("bundle_sort is only valid for bundle search");
    }
    if request.annotation_type_cardinality.is_some() {
        bail!("annotation_type_cardinality is only valid for annotation_type search");
    }
    Ok(())
}

fn has_annotation_only_filters(request: &SearchRequest) -> bool {
    request.annotation_status.is_some()
        || request.annotation_sort.is_some()
        || !request.annotation_type_guids.is_empty()
        || !request.set_by_user_guids.is_empty()
        || !request.subject_account_slugs.is_empty()
        || !request.subject_guids.is_empty()
        || request
            .subject_identifier_prefix
            .as_ref()
            .is_some_and(|value| is_non_blank(value))
        || !request.subject_types.is_empty()
        || !request.type_identifiers.is_empty()
}

fn to_global_api_request(request: SearchRequest) -> models::GlobalSearchRequest {
    models::GlobalSearchRequest {
        q: request.query.and_then(non_blank),
        types: non_empty(
            request
                .types
                .into_iter()
                .map(map_global_search_type)
                .collect(),
        ),
        identifiers: non_empty(
            request
                .identifiers
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        account_slugs: non_empty(
            request
                .account_slugs
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        page: request.page,
        per_page: request.per_page,
        discovery_profile_status: request
            .discovery_profile_status
            .map(map_global_discovery_profile_status),
        include: global_include(request.include_metrics),
        metric_names: non_empty(
            request
                .metric_names
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        view: Some(map_global_search_view(request.view)),
    }
}

fn to_schema_api_request(request: SearchRequest) -> models::SchemaSearchRequest {
    models::SchemaSearchRequest {
        q: request.query.and_then(non_blank),
        identifiers: non_empty(
            request
                .identifiers
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        account_slugs: non_empty(
            request
                .account_slugs
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        page: request.page,
        per_page: request.per_page,
        identifier_prefix: request.identifier_prefix.and_then(non_blank),
        status: request.status.map(map_schema_status),
        current_version_status: request
            .current_version_status
            .map(map_schema_version_status),
        current_version_root_instance_types: non_empty(
            request
                .current_version_root_instance_types
                .into_iter()
                .map(map_schema_root_instance_type)
                .collect(),
        ),
        schema_format: request.schema_format.map(map_schema_format),
        include: schema_include(request.include_metrics),
        view: Some(map_schema_search_view(request.view)),
    }
}

fn to_bundle_api_request(request: SearchRequest) -> models::BundleSearchRequest {
    models::BundleSearchRequest {
        q: request.query.and_then(non_blank),
        identifiers: non_empty(
            request
                .identifiers
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        account_slugs: non_empty(
            request
                .account_slugs
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        page: request.page,
        per_page: request.per_page,
        identifier_prefix: request.identifier_prefix.and_then(non_blank),
        status: request.status.map(map_bundle_status),
        current_version_status: request
            .current_version_status
            .map(map_bundle_version_status),
        sort: request.bundle_sort.map(map_bundle_sort),
        include: bundle_include(request.include_metrics),
        view: Some(map_bundle_search_view(request.view)),
    }
}

fn to_annotation_type_api_request(request: SearchRequest) -> models::AnnotationTypeSearchRequest {
    models::AnnotationTypeSearchRequest {
        q: request.query.and_then(non_blank),
        identifiers: non_empty(
            request
                .identifiers
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        account_slugs: non_empty(
            request
                .account_slugs
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        page: request.page,
        per_page: request.per_page,
        identifier_prefix: request.identifier_prefix.and_then(non_blank),
        status: request.status.map(map_annotation_type_status),
        cardinality: request
            .annotation_type_cardinality
            .map(map_annotation_type_cardinality),
        include: annotation_type_include(request.include_metrics),
        view: Some(map_annotation_type_search_view(request.view)),
    }
}

fn to_annotation_api_request(request: SearchRequest) -> models::AnnotationSearchRequest {
    models::AnnotationSearchRequest {
        q: request.query.and_then(non_blank),
        account_slugs: non_empty(
            request
                .account_slugs
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        page: request.page,
        per_page: request.per_page,
        annotation_type_guids: non_empty(
            request
                .annotation_type_guids
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        set_by_user_guids: non_empty(
            request
                .set_by_user_guids
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        status: request.annotation_status.map(map_annotation_status),
        subject_account_slugs: non_empty(
            request
                .subject_account_slugs
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        subject_guids: non_empty(
            request
                .subject_guids
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        subject_identifier_prefix: request.subject_identifier_prefix.and_then(non_blank),
        subject_types: non_empty(
            request
                .subject_types
                .into_iter()
                .map(map_annotation_subject_type)
                .collect(),
        ),
        type_identifiers: non_empty(
            request
                .type_identifiers
                .into_iter()
                .filter_map(non_blank)
                .collect(),
        ),
        sort: request.annotation_sort.map(map_annotation_sort),
        include: annotation_include(request.include_metrics),
        view: Some(map_annotation_search_view(request.view)),
    }
}

fn map_global_search_type(
    document_type: SearchDocumentType,
) -> models::global_search_request::Types {
    match document_type {
        SearchDocumentType::Schema => models::global_search_request::Types::Schema,
        SearchDocumentType::Bundle => models::global_search_request::Types::Bundle,
        SearchDocumentType::AnnotationType => models::global_search_request::Types::AnnotationType,
        SearchDocumentType::Annotation => models::global_search_request::Types::Annotation,
    }
}

fn map_global_search_view(view: SearchView) -> models::global_search_request::View {
    match view {
        SearchView::Compact => models::global_search_request::View::Compact,
        SearchView::Full => models::global_search_request::View::Full,
    }
}

fn map_schema_search_view(view: SearchView) -> models::schema_search_request::View {
    match view {
        SearchView::Compact => models::schema_search_request::View::Compact,
        SearchView::Full => models::schema_search_request::View::Full,
    }
}

fn map_bundle_search_view(view: SearchView) -> models::bundle_search_request::View {
    match view {
        SearchView::Compact => models::bundle_search_request::View::Compact,
        SearchView::Full => models::bundle_search_request::View::Full,
    }
}

fn map_annotation_type_search_view(
    view: SearchView,
) -> models::annotation_type_search_request::View {
    match view {
        SearchView::Compact => models::annotation_type_search_request::View::Compact,
        SearchView::Full => models::annotation_type_search_request::View::Full,
    }
}

fn map_annotation_search_view(view: SearchView) -> models::annotation_search_request::View {
    match view {
        SearchView::Compact => models::annotation_search_request::View::Compact,
        SearchView::Full => models::annotation_search_request::View::Full,
    }
}

fn map_global_discovery_profile_status(
    status: DiscoveryProfileStatus,
) -> models::global_search_request::DiscoveryProfileStatus {
    match status {
        DiscoveryProfileStatus::Pending => {
            models::global_search_request::DiscoveryProfileStatus::Pending
        }
        DiscoveryProfileStatus::Ready => {
            models::global_search_request::DiscoveryProfileStatus::Ready
        }
        DiscoveryProfileStatus::Failed => {
            models::global_search_request::DiscoveryProfileStatus::Failed
        }
    }
}

fn map_schema_status(status: ResourceStatus) -> models::schema_search_request::Status {
    match status {
        ResourceStatus::Active => models::schema_search_request::Status::Active,
        ResourceStatus::Archived => models::schema_search_request::Status::Archived,
    }
}

fn map_bundle_status(status: ResourceStatus) -> models::bundle_search_request::Status {
    match status {
        ResourceStatus::Active => models::bundle_search_request::Status::Active,
        ResourceStatus::Archived => models::bundle_search_request::Status::Archived,
    }
}

fn map_annotation_type_status(
    status: ResourceStatus,
) -> models::annotation_type_search_request::Status {
    match status {
        ResourceStatus::Active => models::annotation_type_search_request::Status::Active,
        ResourceStatus::Archived => models::annotation_type_search_request::Status::Archived,
    }
}

fn map_schema_version_status(
    status: VersionStatus,
) -> models::schema_search_request::CurrentVersionStatus {
    match status {
        VersionStatus::Draft => models::schema_search_request::CurrentVersionStatus::Draft,
        VersionStatus::Active => models::schema_search_request::CurrentVersionStatus::Active,
        VersionStatus::Deprecated => {
            models::schema_search_request::CurrentVersionStatus::Deprecated
        }
        VersionStatus::Yanked => models::schema_search_request::CurrentVersionStatus::Yanked,
    }
}

fn map_bundle_version_status(
    status: VersionStatus,
) -> models::bundle_search_request::CurrentVersionStatus {
    match status {
        VersionStatus::Draft => models::bundle_search_request::CurrentVersionStatus::Draft,
        VersionStatus::Active => models::bundle_search_request::CurrentVersionStatus::Active,
        VersionStatus::Deprecated => {
            models::bundle_search_request::CurrentVersionStatus::Deprecated
        }
        VersionStatus::Yanked => models::bundle_search_request::CurrentVersionStatus::Yanked,
    }
}

fn map_schema_root_instance_type(
    instance_type: JsonRootInstanceType,
) -> models::schema_search_request::CurrentVersionRootInstanceTypes {
    match instance_type {
        JsonRootInstanceType::Array => {
            models::schema_search_request::CurrentVersionRootInstanceTypes::Array
        }
        JsonRootInstanceType::Boolean => {
            models::schema_search_request::CurrentVersionRootInstanceTypes::Boolean
        }
        JsonRootInstanceType::Integer => {
            models::schema_search_request::CurrentVersionRootInstanceTypes::Integer
        }
        JsonRootInstanceType::Null => {
            models::schema_search_request::CurrentVersionRootInstanceTypes::Null
        }
        JsonRootInstanceType::Number => {
            models::schema_search_request::CurrentVersionRootInstanceTypes::Number
        }
        JsonRootInstanceType::Object => {
            models::schema_search_request::CurrentVersionRootInstanceTypes::Object
        }
        JsonRootInstanceType::String => {
            models::schema_search_request::CurrentVersionRootInstanceTypes::String
        }
    }
}

fn map_schema_format(format: SchemaFormat) -> models::schema_search_request::SchemaFormat {
    match format {
        SchemaFormat::JsonSchema => models::schema_search_request::SchemaFormat::JsonSchema,
    }
}

fn map_bundle_sort(sort: BundleSort) -> models::bundle_search_request::Sort {
    match sort {
        BundleSort::Relevance => models::bundle_search_request::Sort::Relevance,
        BundleSort::Dependencies => models::bundle_search_request::Sort::Dependencies,
    }
}

fn map_annotation_type_cardinality(
    cardinality: AnnotationTypeCardinality,
) -> models::annotation_type_search_request::Cardinality {
    match cardinality {
        AnnotationTypeCardinality::OnePerSubjectPerAccount => {
            models::annotation_type_search_request::Cardinality::OnePerSubjectPerAccount
        }
        AnnotationTypeCardinality::ManyPerSubjectPerAccount => {
            models::annotation_type_search_request::Cardinality::ManyPerSubjectPerAccount
        }
    }
}

fn map_annotation_status(status: AnnotationStatus) -> models::annotation_search_request::Status {
    match status {
        AnnotationStatus::Active => models::annotation_search_request::Status::Active,
        AnnotationStatus::Deprecated => models::annotation_search_request::Status::Deprecated,
        AnnotationStatus::Revoked => models::annotation_search_request::Status::Revoked,
    }
}

fn map_annotation_sort(sort: AnnotationSort) -> models::annotation_search_request::Sort {
    match sort {
        AnnotationSort::Relevance => models::annotation_search_request::Sort::Relevance,
        AnnotationSort::Endorsements => models::annotation_search_request::Sort::Endorsements,
    }
}

fn map_annotation_subject_type(
    subject_type: AnnotationSubjectType,
) -> models::annotation_search_request::SubjectTypes {
    match subject_type {
        AnnotationSubjectType::Annotations => {
            models::annotation_search_request::SubjectTypes::Annotations
        }
        AnnotationSubjectType::BundleVersions => {
            models::annotation_search_request::SubjectTypes::BundleVersions
        }
        AnnotationSubjectType::Bundles => models::annotation_search_request::SubjectTypes::Bundles,
        AnnotationSubjectType::SchemaProposals => {
            models::annotation_search_request::SubjectTypes::SchemaProposals
        }
        AnnotationSubjectType::SchemaVersions => {
            models::annotation_search_request::SubjectTypes::SchemaVersions
        }
        AnnotationSubjectType::Schemas => models::annotation_search_request::SubjectTypes::Schemas,
    }
}

fn global_include(include_metrics: bool) -> Option<Vec<models::global_search_request::Include>> {
    include_metrics.then_some(vec![models::global_search_request::Include::Metrics])
}

fn schema_include(include_metrics: bool) -> Option<Vec<models::schema_search_request::Include>> {
    include_metrics.then_some(vec![models::schema_search_request::Include::Metrics])
}

fn bundle_include(include_metrics: bool) -> Option<Vec<models::bundle_search_request::Include>> {
    include_metrics.then_some(vec![models::bundle_search_request::Include::Metrics])
}

fn annotation_type_include(
    include_metrics: bool,
) -> Option<Vec<models::annotation_type_search_request::Include>> {
    include_metrics.then_some(vec![
        models::annotation_type_search_request::Include::Metrics,
    ])
}

fn annotation_include(
    include_metrics: bool,
) -> Option<Vec<models::annotation_search_request::Include>> {
    include_metrics.then_some(vec![models::annotation_search_request::Include::Metrics])
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
            models::search_result::DocumentType::Bundle => "bundle",
            models::search_result::DocumentType::AnnotationType => "annotation_type",
            models::search_result::DocumentType::Annotation => "annotation",
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

fn is_non_blank(value: &str) -> bool {
    !value.trim().is_empty()
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
        AnnotationSort, AnnotationStatus, AnnotationSubjectType, AnnotationTypeCardinality,
        BundleSort, DiscoveryProfileStatus, JsonRootInstanceType, ResourceStatus, SchemaFormat,
        SearchDocumentType, SearchEndpoint, SearchRequest, SearchView, VersionStatus,
        map_search_response, registry_search_url, search_endpoint, to_annotation_api_request,
        to_annotation_type_api_request, to_bundle_api_request, to_global_api_request,
        to_schema_api_request,
    };
    use rusl_api_client::models;
    use serde_json::json;
    use std::collections::HashMap;

    #[test]
    fn builds_global_search_request_with_supported_filters() {
        let request = to_global_api_request(SearchRequest {
            query: Some("  bearing ".to_string()),
            types: vec![
                SearchDocumentType::Schema,
                SearchDocumentType::Bundle,
                SearchDocumentType::AnnotationType,
            ],
            identifiers: vec![" hassox/schemas/common ".to_string(), " ".to_string()],
            account_slugs: vec![" hassox ".to_string(), " ".to_string()],
            page: Some(2),
            per_page: Some(25),
            view: SearchView::Compact,
            include_metrics: false,
            discovery_profile_status: Some(DiscoveryProfileStatus::Ready),
            metric_names: vec![" watchers ".to_string(), " ".to_string()],
            ..SearchRequest::default()
        });

        assert_eq!(request.q.as_deref(), Some("bearing"));
        assert_eq!(request.account_slugs, Some(vec!["hassox".to_string()]));
        assert_eq!(
            request.types,
            Some(vec![
                models::global_search_request::Types::Schema,
                models::global_search_request::Types::Bundle,
                models::global_search_request::Types::AnnotationType,
            ])
        );
        assert_eq!(
            request.identifiers,
            Some(vec!["hassox/schemas/common".to_string()])
        );
        assert_eq!(request.page, Some(2));
        assert_eq!(request.per_page, Some(25));
        assert_eq!(
            request.view,
            Some(models::global_search_request::View::Compact)
        );
        assert_eq!(request.include, None);
        assert_eq!(
            request.discovery_profile_status,
            Some(models::global_search_request::DiscoveryProfileStatus::Ready)
        );
        assert_eq!(request.metric_names, Some(vec!["watchers".to_string()]));
    }

    #[test]
    fn builds_full_search_request_when_requested() {
        let request = to_global_api_request(SearchRequest {
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
        let request = to_global_api_request(SearchRequest {
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
    fn builds_schema_search_request_with_specific_filters() {
        let request = to_schema_api_request(SearchRequest {
            query: Some("part".to_string()),
            types: vec![SearchDocumentType::Schema],
            identifiers: vec![" hassox/schemas/part ".to_string()],
            identifier_prefix: Some(" hassox/schemas/ ".to_string()),
            status: Some(ResourceStatus::Active),
            current_version_status: Some(VersionStatus::Deprecated),
            current_version_root_instance_types: vec![JsonRootInstanceType::Object],
            schema_format: Some(SchemaFormat::JsonSchema),
            include_metrics: true,
            ..SearchRequest::default()
        });

        assert_eq!(
            request.identifier_prefix.as_deref(),
            Some("hassox/schemas/")
        );
        assert_eq!(
            request.identifiers,
            Some(vec!["hassox/schemas/part".to_string()])
        );
        assert_eq!(
            request.status,
            Some(models::schema_search_request::Status::Active)
        );
        assert_eq!(
            request.current_version_status,
            Some(models::schema_search_request::CurrentVersionStatus::Deprecated)
        );
        assert_eq!(
            request.current_version_root_instance_types,
            Some(vec![
                models::schema_search_request::CurrentVersionRootInstanceTypes::Object
            ])
        );
        assert_eq!(
            request.schema_format,
            Some(models::schema_search_request::SchemaFormat::JsonSchema)
        );
        assert_eq!(
            request.include,
            Some(vec![models::schema_search_request::Include::Metrics])
        );
    }

    #[test]
    fn builds_bundle_search_request_with_specific_filters() {
        let request = to_bundle_api_request(SearchRequest {
            types: vec![SearchDocumentType::Bundle],
            identifiers: vec!["hassox/bundles/common".to_string()],
            identifier_prefix: Some("hassox/bundles/".to_string()),
            status: Some(ResourceStatus::Archived),
            current_version_status: Some(VersionStatus::Active),
            bundle_sort: Some(BundleSort::Dependencies),
            ..SearchRequest::default()
        });

        assert_eq!(
            request.identifiers,
            Some(vec!["hassox/bundles/common".to_string()])
        );
        assert_eq!(
            request.status,
            Some(models::bundle_search_request::Status::Archived)
        );
        assert_eq!(
            request.current_version_status,
            Some(models::bundle_search_request::CurrentVersionStatus::Active)
        );
        assert_eq!(
            request.sort,
            Some(models::bundle_search_request::Sort::Dependencies)
        );
    }

    #[test]
    fn builds_annotation_type_search_request_with_identifier_filters() {
        let request = to_annotation_type_api_request(SearchRequest {
            types: vec![SearchDocumentType::AnnotationType],
            identifiers: vec![
                " hassox/annotation-types/review ".to_string(),
                " ".to_string(),
            ],
            identifier_prefix: Some("hassox/annotation-types/".to_string()),
            status: Some(ResourceStatus::Active),
            annotation_type_cardinality: Some(AnnotationTypeCardinality::OnePerSubjectPerAccount),
            include_metrics: true,
            ..SearchRequest::default()
        });

        assert_eq!(
            request.identifiers,
            Some(vec!["hassox/annotation-types/review".to_string()])
        );
        assert_eq!(
            request.identifier_prefix.as_deref(),
            Some("hassox/annotation-types/")
        );
        assert_eq!(
            request.status,
            Some(models::annotation_type_search_request::Status::Active)
        );
        assert_eq!(
            request.cardinality,
            Some(models::annotation_type_search_request::Cardinality::OnePerSubjectPerAccount)
        );
        assert_eq!(
            request.include,
            Some(vec![
                models::annotation_type_search_request::Include::Metrics
            ])
        );
    }

    #[test]
    fn builds_annotation_search_request_with_specific_filters() {
        let request = to_annotation_api_request(SearchRequest {
            types: vec![SearchDocumentType::Annotation],
            account_slugs: vec!["hassox".to_string()],
            annotation_status: Some(AnnotationStatus::Active),
            annotation_sort: Some(AnnotationSort::Endorsements),
            annotation_type_guids: vec!["type-guid".to_string()],
            set_by_user_guids: vec!["user-guid".to_string()],
            subject_account_slugs: vec!["subject-account".to_string()],
            subject_guids: vec!["subject-guid".to_string()],
            subject_identifier_prefix: Some("hassox/schemas/common".to_string()),
            subject_types: vec![AnnotationSubjectType::Schemas],
            type_identifiers: vec!["hassox/annotation-types/review".to_string()],
            ..SearchRequest::default()
        });

        assert_eq!(
            request.status,
            Some(models::annotation_search_request::Status::Active)
        );
        assert_eq!(
            request.sort,
            Some(models::annotation_search_request::Sort::Endorsements)
        );
        assert_eq!(
            request.subject_types,
            Some(vec![
                models::annotation_search_request::SubjectTypes::Schemas
            ])
        );
        assert_eq!(
            request.type_identifiers,
            Some(vec!["hassox/annotation-types/review".to_string()])
        );
    }

    #[test]
    fn type_specific_filters_select_matching_endpoint() {
        let endpoint = search_endpoint(&SearchRequest {
            types: vec![SearchDocumentType::Bundle],
            bundle_sort: Some(BundleSort::Dependencies),
            ..SearchRequest::default()
        })
        .expect("select endpoint");

        assert_eq!(endpoint, SearchEndpoint::Bundles);
    }

    #[test]
    fn type_specific_filters_require_one_type() {
        let error = search_endpoint(&SearchRequest {
            bundle_sort: Some(BundleSort::Dependencies),
            ..SearchRequest::default()
        })
        .expect_err("invalid endpoint");

        assert_eq!(
            error.to_string(),
            "type-specific search filters require exactly one search type"
        );
    }

    #[test]
    fn rejects_incompatible_type_specific_filters() {
        let error = search_endpoint(&SearchRequest {
            types: vec![SearchDocumentType::Annotation],
            status: Some(ResourceStatus::Active),
            ..SearchRequest::default()
        })
        .expect_err("invalid endpoint");

        assert_eq!(
            error.to_string(),
            "resource, schema, and version filters are not valid for annotation search"
        );
    }

    #[test]
    fn rejects_global_only_filters_with_type_specific_filters() {
        let error = search_endpoint(&SearchRequest {
            types: vec![SearchDocumentType::Schema],
            identifier_prefix: Some("hassox/schemas/".to_string()),
            metric_names: vec!["watchers".to_string()],
            ..SearchRequest::default()
        })
        .expect_err("invalid endpoint");

        assert_eq!(
            error.to_string(),
            "discovery_profile_status and metric_names cannot be combined with type-specific search filters"
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
                identifier: "hassox/schemas/common".to_string(),
            }],
            vec![models::SearchFacet::new(
                vec![models::SearchFacetCountsInner::new(1, "schema".to_string())],
                "document_type".to_string(),
            )],
            models::SearchPageInfo::new(1, false, false, 10, 1, 1),
        );

        let output = map_search_response(response);

        assert_eq!(output.data[0].identifier, "hassox/schemas/common");
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
