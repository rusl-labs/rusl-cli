use crate::{config, registry::client::RegistryClient, resource_identifier::RegistryResource};
use anyhow::{Context, Result, bail};
use rusl_api_client::rusl_user_agent_with_context;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceRecordKind {
    Schema,
    Bundle,
    AnnotationType,
    Annotation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetSchemaRecordRequest {
    pub identifier: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBundleRecordRequest {
    pub identifier: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetAnnotationTypeRecordRequest {
    pub identifier: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetAnnotationRecordRequest {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceRecordOutput {
    pub kind: ResourceRecordKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub record: Value,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AnnotationTypeResource {
    account: String,
    slug: String,
}

impl AnnotationTypeResource {
    fn identifier(&self) -> String {
        format!("{}/annotation-types/{}", self.account, self.slug)
    }
}

pub async fn get_schema_record_with_user_agent_context(
    request: GetSchemaRecordRequest,
    context: &str,
) -> Result<ResourceRecordOutput> {
    let resource = RegistryResource::schema(&request.identifier).with_context(|| {
        format!(
            "Invalid schema identifier `{}`. Use account/schemas/slug.",
            request.identifier
        )
    })?;
    let version = normalize_version(request.version);
    let client = registry_client(context)?;
    let record = match version.as_deref() {
        Some(version) => {
            client
                .fetch_schema_version_record(&resource.account, &resource.slug, version)
                .await?
        }
        None => {
            client
                .fetch_schema_record(&resource.account, &resource.slug)
                .await?
        }
    };

    Ok(ResourceRecordOutput {
        kind: ResourceRecordKind::Schema,
        identifier: Some(resource.identifier()),
        id: None,
        version,
        record,
    })
}

pub async fn get_bundle_record_with_user_agent_context(
    request: GetBundleRecordRequest,
    context: &str,
) -> Result<ResourceRecordOutput> {
    let resource = RegistryResource::bundle(&request.identifier).with_context(|| {
        format!(
            "Invalid bundle identifier `{}`. Use account/bundles/slug.",
            request.identifier
        )
    })?;
    let version = normalize_version(request.version);
    let client = registry_client(context)?;
    let record = match version.as_deref() {
        Some(version) => {
            client
                .fetch_bundle_version_record(&resource.account, &resource.slug, version)
                .await?
        }
        None => {
            client
                .fetch_bundle_record(&resource.account, &resource.slug)
                .await?
        }
    };

    Ok(ResourceRecordOutput {
        kind: ResourceRecordKind::Bundle,
        identifier: Some(resource.identifier()),
        id: None,
        version,
        record,
    })
}

pub async fn get_annotation_type_record_with_user_agent_context(
    request: GetAnnotationTypeRecordRequest,
    context: &str,
) -> Result<ResourceRecordOutput> {
    let resource = parse_annotation_type_identifier(&request.identifier).with_context(|| {
        format!(
            "Invalid annotation type identifier `{}`. Use account/annotation-types/slug.",
            request.identifier
        )
    })?;
    let client = registry_client(context)?;
    let record = client
        .fetch_annotation_type_record(&resource.account, &resource.slug)
        .await?;

    Ok(ResourceRecordOutput {
        kind: ResourceRecordKind::AnnotationType,
        identifier: Some(resource.identifier()),
        id: None,
        version: None,
        record,
    })
}

pub async fn get_annotation_record_with_user_agent_context(
    request: GetAnnotationRecordRequest,
    context: &str,
) -> Result<ResourceRecordOutput> {
    let annotation_id = normalize_annotation_id(&request.id)?;
    let client = registry_client(context)?;
    let record = client.fetch_annotation_record(&annotation_id).await?;

    Ok(ResourceRecordOutput {
        kind: ResourceRecordKind::Annotation,
        identifier: None,
        id: Some(annotation_id),
        version: None,
        record,
    })
}

fn registry_client(context: &str) -> Result<RegistryClient> {
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    Ok(RegistryClient::with_user_agent_and_rusl_agent(
        config, user_agent, context,
    ))
}

fn normalize_version(version: Option<String>) -> Option<String> {
    version.and_then(|value| {
        let trimmed = value.trim();
        let trimmed = match trimmed.strip_prefix('v') {
            Some(rest) if rest.starts_with(|character: char| character.is_ascii_digit()) => rest,
            _ => trimmed,
        };
        (!trimmed.is_empty()).then(|| trimmed.to_string())
    })
}

fn normalize_annotation_id(id: &str) -> Result<String> {
    let trimmed = id.trim();
    let id = trimmed
        .strip_prefix("annotations.")
        .unwrap_or(trimmed)
        .trim();
    if id.is_empty() {
        bail!("Annotation id is required");
    }
    Ok(id.to_string())
}

fn parse_annotation_type_identifier(identifier: &str) -> Option<AnnotationTypeResource> {
    let trimmed = identifier.trim();
    let parts: Vec<&str> = trimmed.split('/').collect();
    match parts.as_slice() {
        [account, "annotation-types", slug] | [account, "annotation_types", slug]
            if valid_part(account) && valid_part(slug) =>
        {
            Some(AnnotationTypeResource {
                account: (*account).to_string(),
                slug: (*slug).to_string(),
            })
        }
        _ => None,
    }
}

fn valid_part(part: &str) -> bool {
    !part.trim().is_empty() && part == part.trim()
}

#[cfg(test)]
mod tests {
    use super::{
        ResourceRecordKind, normalize_annotation_id, normalize_version,
        parse_annotation_type_identifier,
    };
    use crate::resource_identifier::RegistryResource;

    #[test]
    fn normalizes_schema_and_bundle_identifiers_for_gets() {
        let schema = RegistryResource::schema("hassox/schemas/common").expect("schema identifier");
        let bundle = RegistryResource::bundle("hassox/bundles/common").expect("bundle identifier");

        assert_eq!(schema.identifier(), "hassox/schemas/common");
        assert_eq!(bundle.identifier(), "hassox/bundles/common");
    }

    #[test]
    fn parses_annotation_type_identifiers() {
        let resource =
            parse_annotation_type_identifier("hassox/annotation-types/review").expect("canonical");
        assert_eq!(resource.account, "hassox");
        assert_eq!(resource.slug, "review");
        assert_eq!(resource.identifier(), "hassox/annotation-types/review");

        let resource =
            parse_annotation_type_identifier("hassox/annotation_types/review").expect("api style");
        assert_eq!(resource.identifier(), "hassox/annotation-types/review");

        assert!(parse_annotation_type_identifier("hassox/review").is_none());
    }

    #[test]
    fn normalizes_version_and_annotation_id_inputs() {
        assert_eq!(
            normalize_version(Some(" v1.2.3 ".to_string())),
            Some("1.2.3".to_string())
        );
        assert_eq!(
            normalize_version(Some("version".to_string())),
            Some("version".to_string())
        );
        assert_eq!(normalize_version(Some("".to_string())), None);
        assert_eq!(
            normalize_annotation_id(" annotations.123 ").expect("guid style"),
            "123"
        );
        assert_eq!(normalize_annotation_id("123").expect("raw id"), "123");
        assert!(normalize_annotation_id("annotations.").is_err());
    }

    #[test]
    fn resource_record_kind_serializes_for_tool_output() {
        let kind = serde_json::to_value(ResourceRecordKind::AnnotationType).expect("serialize");

        assert_eq!(kind, serde_json::json!("annotation_type"));
    }
}
