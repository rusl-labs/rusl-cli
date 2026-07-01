use crate::{config, config::credentials::Credentials, registry::client::RegistryClient};
use anyhow::{Context, Result, bail};
use rusl_api_client::{models, rusl_user_agent_with_context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationTypeVisibility {
    Public,
    Private,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnnotationTypeSchemaMode {
    Current,
    Pinned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AnnotationTypeCardinality {
    OnePerSubjectPerAccount,
    ManyPerSubjectPerAccount,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateAnnotationTypeRequest {
    pub account_slug: String,
    pub annotation_type_slug: String,
    pub schema_identifier: String,
    pub schema_mode: AnnotationTypeSchemaMode,
    pub description: Option<String>,
    pub visibility: AnnotationTypeVisibility,
    pub cardinality: Option<AnnotationTypeCardinality>,
    pub pinned_schema_version_id: Option<String>,
    pub content_immutable: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnotationTypeOutput {
    pub id: String,
    pub guid: String,
    pub account_slug: String,
    pub annotation_type_slug: String,
    pub identifier: String,
    pub description: Option<String>,
    pub schema_identifier: Option<String>,
    pub schema_mode: String,
    pub cardinality: String,
    pub status: String,
    pub visibility: Option<String>,
    pub content_immutable: bool,
    pub inserted_at: String,
    pub updated_at: String,
}

pub async fn create_annotation_type_with_user_agent_context(
    request: CreateAnnotationTypeRequest,
    context: &str,
) -> Result<AnnotationTypeOutput> {
    require_login()?;
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let account_slug = request.account_slug.clone();
    let response = client
        .create_annotation_type(
            &account_slug,
            to_api_create_annotation_type_request(request),
        )
        .await?;
    let Some(annotation_type) = response.data.flatten() else {
        bail!("Rusl API did not return created annotation type data");
    };

    Ok(map_annotation_type(*annotation_type))
}

fn require_login() -> Result<()> {
    let Some(credentials) = Credentials::load() else {
        bail!("This tool requires user authorization. Run `rusl login` first.");
    };

    if credentials.access_token.trim().is_empty() && credentials.refresh_token.trim().is_empty() {
        bail!("This tool requires user authorization. Run `rusl login` first.");
    }

    Ok(())
}

fn to_api_create_annotation_type_request(
    request: CreateAnnotationTypeRequest,
) -> rusl_api_client::CreateAnnotationTypeRequest {
    rusl_api_client::CreateAnnotationTypeRequest {
        slug: request.annotation_type_slug,
        schema_identifier: request.schema_identifier,
        schema_mode: schema_mode_label(request.schema_mode).to_string(),
        description: request.description,
        visibility: Some(visibility_label(request.visibility).to_string()),
        cardinality: request
            .cardinality
            .map(|value| cardinality_label(value).to_string()),
        pinned_schema_version_id: request.pinned_schema_version_id,
        content_immutable: request.content_immutable,
    }
}

fn map_annotation_type(annotation_type: models::AnnotationType1) -> AnnotationTypeOutput {
    AnnotationTypeOutput {
        id: annotation_type.id,
        guid: annotation_type.guid,
        account_slug: annotation_type.account_slug,
        annotation_type_slug: annotation_type.slug,
        identifier: annotation_type.identifier,
        description: annotation_type.description.flatten(),
        schema_identifier: annotation_type.schema_identifier.flatten(),
        schema_mode: schema_mode_api_label(annotation_type.schema_mode).to_string(),
        cardinality: cardinality_api_label(annotation_type.cardinality).to_string(),
        status: annotation_type_status_label(annotation_type.status).to_string(),
        visibility: Some(visibility_api_label(annotation_type.visibility).to_string()),
        content_immutable: annotation_type.content_immutable,
        inserted_at: annotation_type.inserted_at,
        updated_at: annotation_type.updated_at,
    }
}

fn schema_mode_label(mode: AnnotationTypeSchemaMode) -> &'static str {
    match mode {
        AnnotationTypeSchemaMode::Current => "CURRENT",
        AnnotationTypeSchemaMode::Pinned => "PINNED",
    }
}

fn schema_mode_api_label(mode: models::annotation_type_1::SchemaMode) -> &'static str {
    match mode {
        models::annotation_type_1::SchemaMode::Current => "CURRENT",
        models::annotation_type_1::SchemaMode::Pinned => "PINNED",
    }
}

fn cardinality_label(cardinality: AnnotationTypeCardinality) -> &'static str {
    match cardinality {
        AnnotationTypeCardinality::OnePerSubjectPerAccount => "ONE_PER_SUBJECT_PER_ACCOUNT",
        AnnotationTypeCardinality::ManyPerSubjectPerAccount => "MANY_PER_SUBJECT_PER_ACCOUNT",
    }
}

fn cardinality_api_label(cardinality: models::annotation_type_1::Cardinality) -> &'static str {
    match cardinality {
        models::annotation_type_1::Cardinality::OnePerSubjectPerAccount => {
            "ONE_PER_SUBJECT_PER_ACCOUNT"
        }
        models::annotation_type_1::Cardinality::ManyPerSubjectPerAccount => {
            "MANY_PER_SUBJECT_PER_ACCOUNT"
        }
    }
}

fn visibility_label(visibility: AnnotationTypeVisibility) -> &'static str {
    match visibility {
        AnnotationTypeVisibility::Public => "PUBLIC",
        AnnotationTypeVisibility::Private => "PRIVATE",
    }
}

fn visibility_api_label(visibility: models::annotation_type_1::Visibility) -> &'static str {
    match visibility {
        models::annotation_type_1::Visibility::Public => "PUBLIC",
        models::annotation_type_1::Visibility::Private => "PRIVATE",
    }
}

fn annotation_type_status_label(status: models::annotation_type_1::Status) -> &'static str {
    match status {
        models::annotation_type_1::Status::Active => "ACTIVE",
        models::annotation_type_1::Status::Archived => "ARCHIVED",
    }
}
