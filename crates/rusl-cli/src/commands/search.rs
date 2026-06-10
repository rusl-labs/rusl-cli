use crate::cli::{
    SearchAnnotationSort, SearchAnnotationStatus, SearchAnnotationSubjectType,
    SearchAnnotationTypeCardinality, SearchArgs, SearchBundleSort, SearchDiscoveryProfileStatus,
    SearchJsonRootInstanceType, SearchResourceStatus, SearchSchemaFormat, SearchType,
    SearchVersionStatus, SearchViewArg,
};
use anyhow::Result;
use rusl_app::search_service::{self, SearchRequest};

pub async fn run(args: SearchArgs) -> Result<()> {
    let output = search_service::search_registry(to_search_request(args)).await?;
    println!("{}", search_data_json(&output)?);
    Ok(())
}

fn search_data_json(output: &search_service::SearchOutput) -> Result<String> {
    Ok(serde_json::to_string_pretty(&output.data)?)
}

fn to_search_request(args: SearchArgs) -> SearchRequest {
    SearchRequest {
        query: args.query,
        types: args.types.into_iter().map(map_search_type).collect(),
        identifiers: args.identifiers,
        account_slugs: args.account_slugs,
        page: args.page,
        per_page: args.per_page,
        view: args
            .view
            .map(map_search_view)
            .unwrap_or(search_service::SearchView::Compact),
        include_metrics: args.include_metrics,
        discovery_profile_status: args
            .discovery_profile_status
            .map(map_discovery_profile_status),
        metric_names: args.metric_names,
        identifier_prefix: args.identifier_prefix,
        status: args.status.map(map_resource_status),
        current_version_status: args.current_version_status.map(map_version_status),
        current_version_root_instance_types: args
            .current_version_root_instance_types
            .into_iter()
            .map(map_json_root_instance_type)
            .collect(),
        schema_format: args.schema_format.map(map_schema_format),
        bundle_sort: args.bundle_sort.map(map_bundle_sort),
        annotation_type_cardinality: args
            .annotation_type_cardinality
            .map(map_annotation_type_cardinality),
        annotation_status: args.annotation_status.map(map_annotation_status),
        annotation_sort: args.annotation_sort.map(map_annotation_sort),
        annotation_type_guids: args.annotation_type_guids,
        set_by_user_guids: args.set_by_user_guids,
        subject_account_slugs: args.subject_account_slugs,
        subject_guids: args.subject_guids,
        subject_identifier_prefix: args.subject_identifier_prefix,
        subject_types: args
            .subject_types
            .into_iter()
            .map(map_annotation_subject_type)
            .collect(),
        type_identifiers: args.type_identifiers,
    }
}

fn map_search_type(value: SearchType) -> search_service::SearchDocumentType {
    match value {
        SearchType::Schema => search_service::SearchDocumentType::Schema,
        SearchType::Bundle => search_service::SearchDocumentType::Bundle,
        SearchType::AnnotationType => search_service::SearchDocumentType::AnnotationType,
        SearchType::Annotation => search_service::SearchDocumentType::Annotation,
    }
}

fn map_search_view(value: SearchViewArg) -> search_service::SearchView {
    match value {
        SearchViewArg::Compact => search_service::SearchView::Compact,
        SearchViewArg::Full => search_service::SearchView::Full,
    }
}

fn map_discovery_profile_status(
    value: SearchDiscoveryProfileStatus,
) -> search_service::DiscoveryProfileStatus {
    match value {
        SearchDiscoveryProfileStatus::Pending => search_service::DiscoveryProfileStatus::Pending,
        SearchDiscoveryProfileStatus::Ready => search_service::DiscoveryProfileStatus::Ready,
        SearchDiscoveryProfileStatus::Failed => search_service::DiscoveryProfileStatus::Failed,
    }
}

fn map_resource_status(value: SearchResourceStatus) -> search_service::ResourceStatus {
    match value {
        SearchResourceStatus::Active => search_service::ResourceStatus::Active,
        SearchResourceStatus::Archived => search_service::ResourceStatus::Archived,
    }
}

fn map_version_status(value: SearchVersionStatus) -> search_service::VersionStatus {
    match value {
        SearchVersionStatus::Draft => search_service::VersionStatus::Draft,
        SearchVersionStatus::Active => search_service::VersionStatus::Active,
        SearchVersionStatus::Deprecated => search_service::VersionStatus::Deprecated,
        SearchVersionStatus::Yanked => search_service::VersionStatus::Yanked,
    }
}

fn map_json_root_instance_type(
    value: SearchJsonRootInstanceType,
) -> search_service::JsonRootInstanceType {
    match value {
        SearchJsonRootInstanceType::Array => search_service::JsonRootInstanceType::Array,
        SearchJsonRootInstanceType::Boolean => search_service::JsonRootInstanceType::Boolean,
        SearchJsonRootInstanceType::Integer => search_service::JsonRootInstanceType::Integer,
        SearchJsonRootInstanceType::Null => search_service::JsonRootInstanceType::Null,
        SearchJsonRootInstanceType::Number => search_service::JsonRootInstanceType::Number,
        SearchJsonRootInstanceType::Object => search_service::JsonRootInstanceType::Object,
        SearchJsonRootInstanceType::String => search_service::JsonRootInstanceType::String,
    }
}

fn map_schema_format(value: SearchSchemaFormat) -> search_service::SchemaFormat {
    match value {
        SearchSchemaFormat::JsonSchema => search_service::SchemaFormat::JsonSchema,
    }
}

fn map_bundle_sort(value: SearchBundleSort) -> search_service::BundleSort {
    match value {
        SearchBundleSort::Relevance => search_service::BundleSort::Relevance,
        SearchBundleSort::Dependencies => search_service::BundleSort::Dependencies,
    }
}

fn map_annotation_type_cardinality(
    value: SearchAnnotationTypeCardinality,
) -> search_service::AnnotationTypeCardinality {
    match value {
        SearchAnnotationTypeCardinality::OnePerSubjectPerAccount => {
            search_service::AnnotationTypeCardinality::OnePerSubjectPerAccount
        }
        SearchAnnotationTypeCardinality::ManyPerSubjectPerAccount => {
            search_service::AnnotationTypeCardinality::ManyPerSubjectPerAccount
        }
    }
}

fn map_annotation_status(value: SearchAnnotationStatus) -> search_service::AnnotationStatus {
    match value {
        SearchAnnotationStatus::Active => search_service::AnnotationStatus::Active,
        SearchAnnotationStatus::Deprecated => search_service::AnnotationStatus::Deprecated,
        SearchAnnotationStatus::Revoked => search_service::AnnotationStatus::Revoked,
    }
}

fn map_annotation_sort(value: SearchAnnotationSort) -> search_service::AnnotationSort {
    match value {
        SearchAnnotationSort::Relevance => search_service::AnnotationSort::Relevance,
        SearchAnnotationSort::Endorsements => search_service::AnnotationSort::Endorsements,
    }
}

fn map_annotation_subject_type(
    value: SearchAnnotationSubjectType,
) -> search_service::AnnotationSubjectType {
    match value {
        SearchAnnotationSubjectType::Annotations => {
            search_service::AnnotationSubjectType::Annotations
        }
        SearchAnnotationSubjectType::BundleVersions => {
            search_service::AnnotationSubjectType::BundleVersions
        }
        SearchAnnotationSubjectType::Bundles => search_service::AnnotationSubjectType::Bundles,
        SearchAnnotationSubjectType::SchemaProposals => {
            search_service::AnnotationSubjectType::SchemaProposals
        }
        SearchAnnotationSubjectType::SchemaVersions => {
            search_service::AnnotationSubjectType::SchemaVersions
        }
        SearchAnnotationSubjectType::Schemas => search_service::AnnotationSubjectType::Schemas,
    }
}

#[cfg(test)]
mod tests {
    use super::{search_data_json, to_search_request};
    use crate::cli::{Cli, Commands};
    use clap::Parser;
    use rusl_app::search_service::{
        AnnotationSort, AnnotationStatus, AnnotationSubjectType, BundleSort,
        DiscoveryProfileStatus, JsonRootInstanceType, ResourceStatus, SearchDocumentType,
        SearchFacet, SearchFacetCount, SearchOutput, SearchPageInfo, SearchResult, SearchView,
        VersionStatus,
    };
    use std::collections::HashMap;

    #[test]
    fn parses_search_command_and_maps_args() {
        let cli = Cli::parse_from([
            "rusl",
            "search",
            "brake",
            "--type",
            "schema,annotation_type",
            "--identifier",
            "hassox/schemas/common",
            "--account",
            "hassox",
            "--page",
            "2",
            "--per-page",
            "25",
            "--view",
            "full",
            "--include-metrics",
            "--discovery-profile-status",
            "ready",
            "--metric-name",
            "watchers",
        ]);

        let Commands::Search(args) = cli.command else {
            panic!("expected search command");
        };
        let request = to_search_request(*args);

        assert_eq!(request.query.as_deref(), Some("brake"));
        assert_eq!(
            request.types,
            vec![
                SearchDocumentType::Schema,
                SearchDocumentType::AnnotationType
            ]
        );
        assert_eq!(
            request.identifiers,
            vec!["hassox/schemas/common".to_string()]
        );
        assert_eq!(request.account_slugs, vec!["hassox".to_string()]);
        assert_eq!(request.page, Some(2));
        assert_eq!(request.per_page, Some(25));
        assert_eq!(request.view, SearchView::Full);
        assert!(request.include_metrics);
        assert_eq!(
            request.discovery_profile_status,
            Some(DiscoveryProfileStatus::Ready)
        );
        assert_eq!(request.metric_names, vec!["watchers".to_string()]);
    }

    #[test]
    fn parses_type_specific_search_filters() {
        let cli = Cli::parse_from([
            "rusl",
            "search",
            "--type",
            "annotation",
            "--annotation-status",
            "active",
            "--annotation-sort",
            "endorsements",
            "--annotation-type-guid",
            "type-guid",
            "--set-by-user-guid",
            "user-guid",
            "--subject-account",
            "hassox",
            "--subject-guid",
            "subject-guid",
            "--subject-identifier-prefix",
            "hassox/schemas/common",
            "--subject-type",
            "schemas",
            "--type-identifier",
            "hassox/annotation-types/review",
        ]);

        let Commands::Search(args) = cli.command else {
            panic!("expected search command");
        };
        let request = to_search_request(*args);

        assert_eq!(request.types, vec![SearchDocumentType::Annotation]);
        assert_eq!(request.annotation_status, Some(AnnotationStatus::Active));
        assert_eq!(request.annotation_sort, Some(AnnotationSort::Endorsements));
        assert_eq!(request.annotation_type_guids, vec!["type-guid".to_string()]);
        assert_eq!(request.set_by_user_guids, vec!["user-guid".to_string()]);
        assert_eq!(request.subject_account_slugs, vec!["hassox".to_string()]);
        assert_eq!(request.subject_guids, vec!["subject-guid".to_string()]);
        assert_eq!(
            request.subject_identifier_prefix.as_deref(),
            Some("hassox/schemas/common")
        );
        assert_eq!(request.subject_types, vec![AnnotationSubjectType::Schemas]);
        assert_eq!(
            request.type_identifiers,
            vec!["hassox/annotation-types/review".to_string()]
        );
    }

    #[test]
    fn parses_schema_and_bundle_filter_enums() {
        let cli = Cli::parse_from([
            "rusl",
            "search",
            "--type",
            "bundle",
            "--identifier-prefix",
            "hassox/bundles/",
            "--status",
            "archived",
            "--current-version-status",
            "deprecated",
            "--current-version-root-instance-type",
            "object,array",
            "--bundle-sort",
            "dependencies",
        ]);

        let Commands::Search(args) = cli.command else {
            panic!("expected search command");
        };
        let request = to_search_request(*args);

        assert_eq!(
            request.identifier_prefix.as_deref(),
            Some("hassox/bundles/")
        );
        assert_eq!(request.status, Some(ResourceStatus::Archived));
        assert_eq!(
            request.current_version_status,
            Some(VersionStatus::Deprecated)
        );
        assert_eq!(
            request.current_version_root_instance_types,
            vec![JsonRootInstanceType::Object, JsonRootInstanceType::Array]
        );
        assert_eq!(request.bundle_sort, Some(BundleSort::Dependencies));
    }

    #[test]
    fn search_data_json_prints_only_result_array() {
        let output = SearchOutput {
            data: vec![SearchResult {
                guid: "schema-guid".to_string(),
                identifier: "hassox/schemas/common".to_string(),
                document_type: "schema".to_string(),
                document_type_label: "Schema".to_string(),
                description: Some("Shared schema".to_string()),
                discovery_profile: Some(HashMap::new()),
                highlights: Vec::new(),
            }],
            facets: vec![SearchFacet {
                field: "document_type".to_string(),
                counts: vec![SearchFacetCount {
                    value: "schema".to_string(),
                    count: 1,
                }],
            }],
            page_info: SearchPageInfo {
                current_page: 1,
                has_next_page: false,
                has_previous_page: false,
                page_size: 20,
                total_count: 1,
                total_pages: 1,
            },
        };

        let json = search_data_json(&output).expect("serialize search data");

        assert!(json.trim_start().starts_with('['));
        assert!(json.contains("\"identifier\": \"hassox/schemas/common\""));
        assert!(!json.contains("page_info"));
        assert!(!json.contains("facets"));
    }
}
