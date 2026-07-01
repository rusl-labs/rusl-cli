use crate::models;
use serde::{Deserialize, Serialize};

/// Response envelope for `POST /api/{account_slug}/annotations`.
///
/// The Phoenix API always wraps single-resource responses in `{data:
/// ...}`, and every other create-flavoured endpoint has a generated
/// `RuslWebApi*Controller*200Response` model to match. The annotation
/// controller lacks one in the current OpenAPI snapshot — the
/// generator emits `Result<models::Annotation, _>` directly, which
/// then fails to decode against the `{data: ...}` wrapper the server
/// actually sends.
///
/// This envelope is a stopgap: when the upstream OpenAPI spec is
/// updated to describe a `RuslWebApiAnnotationControllerCreate201Response`
/// (mirroring the annotation-type / bundle / schema ones), regenerate
/// and delete this struct.
#[derive(Debug, Clone, Deserialize)]
pub struct AnnotationEnvelope {
    /// Present on success. The controller returns the created (or
    /// pre-existing, for cardinality-1 types) annotation.
    #[serde(default)]
    pub data: Option<Box<models::Annotation>>,
    /// Present on structured error responses.
    #[serde(default)]
    pub errors: Option<Vec<models::Error2>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AcceptSchemaProposalRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateBundleRequest {
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateAnnotationTypeRequest {
    pub slug: String,
    pub schema_identifier: String,
    pub schema_mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardinality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_schema_version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_immutable: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateBundleVersionRequest {
    pub version: String,
    pub manifest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
