use crate::{config, registry::client::RegistryClient};
use anyhow::{Context, Result, bail};
use rusl_api_client::{models, rusl_user_agent_with_context};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackAnnotationKind {
    ContextLoadingHint,
    UsageReport,
    DomainInterpretation,
    SemanticLink,
    TrustSignal,
    ContextRequest,
    SourceAttestation,
    MigrationGuide,
}

impl FeedbackAnnotationKind {
    pub fn slug(self) -> &'static str {
        match self {
            Self::ContextLoadingHint => "context-loading-hint",
            Self::UsageReport => "usage-report",
            Self::DomainInterpretation => "domain-interpretation",
            Self::SemanticLink => "semantic-link",
            Self::TrustSignal => "trust-signal",
            Self::ContextRequest => "context-request",
            Self::SourceAttestation => "source-attestation",
            Self::MigrationGuide => "migration-guide",
        }
    }

    pub fn type_identifier(self) -> String {
        format!("rusl/annotation-types/{}", self.slug())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateAnnotationRequest {
    pub account_slug: String,
    pub subject_guid: String,
    pub annotation_type: String,
    pub label: Option<String>,
    pub content: Value,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateFeedbackAnnotationRequest {
    pub account_slug: String,
    pub subject_guid: String,
    pub kind: FeedbackAnnotationKind,
    pub label: Option<String>,
    pub content: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnnotationOutput {
    pub id: String,
    pub guid: String,
    pub account_slug: String,
    pub subject_guid: String,
    pub annotation_type: String,
    pub status: String,
    pub label: Option<String>,
    pub validation_schema_identifier: Option<String>,
    pub validated_at_version: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndorseTargetType {
    Annotation,
    Schema,
    Bundle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndorseRequest {
    pub target_type: EndorseTargetType,
    pub subject_guid: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EndorseOutput {
    pub id: String,
    pub subject_guid: String,
    pub subject_type: Option<String>,
    pub interaction_type: String,
}

pub async fn create_feedback_annotation_with_user_agent_context(
    request: CreateFeedbackAnnotationRequest,
    context: &str,
) -> Result<AnnotationOutput> {
    validate_feedback_content(request.kind, &request.content)?;
    create_annotation_with_user_agent_context(
        CreateAnnotationRequest {
            account_slug: request.account_slug,
            subject_guid: request.subject_guid,
            annotation_type: request.kind.type_identifier(),
            label: request.label,
            content: request.content,
        },
        context,
    )
    .await
}

pub async fn create_annotation_with_user_agent_context(
    request: CreateAnnotationRequest,
    context: &str,
) -> Result<AnnotationOutput> {
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let account_slug = request.account_slug.clone();
    let annotation = client
        .create_annotation(&account_slug, to_api_create_request(request))
        .await?;

    Ok(map_annotation(annotation))
}

pub async fn endorse_with_user_agent_context(
    request: EndorseRequest,
    context: &str,
) -> Result<EndorseOutput> {
    match request.target_type {
        EndorseTargetType::Annotation => endorse_annotation(request.subject_guid, context).await,
        EndorseTargetType::Schema | EndorseTargetType::Bundle => {
            bail!(
                "Endorsement for {:?} targets is not supported by the current Rusl API yet",
                request.target_type
            )
        }
    }
}

pub fn feedback_content_schema(kind: FeedbackAnnotationKind) -> Result<Value> {
    let path = feedback_schema_path(kind)?;
    let contents = std::fs::read_to_string(&path)
        .with_context(|| missing_feedback_schema_message(kind, &path))?;
    serde_json::from_str(&contents)
        .with_context(|| format!("Failed to parse feedback schema at {}", path.display()))
}

pub fn validate_feedback_content(kind: FeedbackAnnotationKind, content: &Value) -> Result<()> {
    let schema = feedback_content_schema(kind)?;
    let validator = jsonschema::draft202012::options()
        .should_validate_formats(true)
        .build(&schema)
        .with_context(|| format!("Failed to compile feedback schema {}", kind.slug()))?;

    let errors = validator
        .iter_errors(content)
        .take(5)
        .map(|error| {
            let path = error.instance_path().to_string();
            if path.is_empty() {
                error.to_string()
            } else {
                format!("{path}: {error}")
            }
        })
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        bail!(
            "Feedback annotation content failed validation for {}: {}",
            kind.slug(),
            errors.join("; ")
        );
    }

    Ok(())
}

async fn endorse_annotation(subject_guid: String, context: &str) -> Result<EndorseOutput> {
    let annotation_id = annotation_id_from_guid(&subject_guid);
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let response = client.endorse_annotation(&annotation_id).await?;
    let Some(interaction) = response.data else {
        bail!("Rusl API did not return endorsement interaction data");
    };

    Ok(EndorseOutput {
        id: interaction.id.to_string(),
        subject_guid: interaction.subject_guid,
        subject_type: interaction.subject_type.flatten(),
        interaction_type: interaction_type_label(interaction.interaction_type).to_string(),
    })
}

fn to_api_create_request(
    request: CreateAnnotationRequest,
) -> models::RuslWebApiAnnotationControllerCreateRequest {
    let mut api_request = models::RuslWebApiAnnotationControllerCreateRequest::new(
        request.content,
        request.subject_guid,
        request.annotation_type,
    );
    api_request.label = request.label.map(Some);
    api_request
}

fn map_annotation(annotation: models::Annotation) -> AnnotationOutput {
    AnnotationOutput {
        id: annotation.id,
        guid: annotation.guid,
        account_slug: annotation.account_slug,
        subject_guid: annotation.subject_guid,
        annotation_type: annotation.r#type,
        status: annotation_status_label(annotation.status).to_string(),
        label: annotation.label.flatten(),
        validation_schema_identifier: annotation.validation_schema_identifier.flatten(),
        validated_at_version: annotation.validated_at_version.flatten(),
    }
}

fn feedback_schema_path(kind: FeedbackAnnotationKind) -> Result<PathBuf> {
    let config = config::load().context("Failed to load schema configuration")?;
    let cwd = std::env::current_dir().context("Failed to get current working directory")?;
    let schema_file = PathBuf::from("rusl").join(format!("{}.json", kind.slug()));
    let configured_schema_path = PathBuf::from(config.schema_dir()).join(&schema_file);
    let vendored_schema_path = PathBuf::from("schemas/vendor").join(&schema_file);

    for root in schema_search_roots(&cwd) {
        for relative_path in [&configured_schema_path, &vendored_schema_path] {
            let candidate = root.join(relative_path);
            if candidate.exists() {
                return Ok(candidate);
            }
        }
    }

    Ok(cwd.join(configured_schema_path))
}

fn schema_search_roots(cwd: &std::path::Path) -> Vec<PathBuf> {
    let mut roots = cwd.ancestors().map(PathBuf::from).collect::<Vec<_>>();
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    if let Some(workspace_root) = manifest_dir.parent().and_then(std::path::Path::parent) {
        let workspace_root = workspace_root.to_path_buf();
        if !roots.iter().any(|root| root == &workspace_root) {
            roots.push(workspace_root);
        }
    }
    roots
}

fn missing_feedback_schema_message(kind: FeedbackAnnotationKind, path: &std::path::Path) -> String {
    format!(
        "Feedback schema {} is not installed at {}. Add rusl/bundles/feedback-schemas to rusl.bundle.toml and run `rusl install` to vendor the schema snapshot.",
        kind.slug(),
        path.display()
    )
}

fn annotation_id_from_guid(subject_guid: &str) -> String {
    subject_guid
        .trim()
        .strip_prefix("annotations.")
        .unwrap_or_else(|| subject_guid.trim())
        .to_string()
}

fn annotation_status_label(status: models::annotation::Status) -> &'static str {
    match status {
        models::annotation::Status::Active => "ACTIVE",
        models::annotation::Status::Deprecated => "DEPRECATED",
        models::annotation::Status::Revoked => "REVOKED",
    }
}

fn interaction_type_label(
    interaction_type: models::resource_interaction_1::InteractionType,
) -> &'static str {
    match interaction_type {
        models::resource_interaction_1::InteractionType::Favourite => "favourite",
        models::resource_interaction_1::InteractionType::Watch => "watch",
        models::resource_interaction_1::InteractionType::Endorse => "endorse",
    }
}

#[cfg(test)]
mod tests {
    use super::{FeedbackAnnotationKind, annotation_id_from_guid, validate_feedback_content};
    use serde_json::json;

    #[test]
    fn validates_feedback_content_against_vendored_schema() {
        validate_feedback_content(
            FeedbackAnnotationKind::ContextRequest,
            &json!({
                "failing_task": "Generate a type safely",
                "suspected_ambiguity": "The field unit is not defined"
            }),
        )
        .expect("valid context request content");

        let error = validate_feedback_content(FeedbackAnnotationKind::ContextRequest, &json!({}))
            .expect_err("missing required fields");

        assert!(
            error
                .to_string()
                .contains("Feedback annotation content failed validation for context-request")
        );
    }

    #[test]
    fn strips_annotation_guid_prefix_for_current_endorse_endpoint() {
        assert_eq!(
            annotation_id_from_guid("annotations.abc-123"),
            "abc-123".to_string()
        );
        assert_eq!(annotation_id_from_guid("abc-123"), "abc-123".to_string());
    }
}
