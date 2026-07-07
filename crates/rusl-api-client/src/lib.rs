use reqwest::StatusCode;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::future::Future;

pub mod handwritten;

pub use handwritten::{
    AcceptSchemaProposalRequest, AnnotationEnvelope, CreateAnnotationTypeRequest,
    CreateBundleRequest, CreateBundleVersionRequest,
};
pub use rusl_openapi_client as generated;
pub use rusl_openapi_client::models;

pub const RUSL_AGENT_HEADER: &str = "x-rusl-agent";

pub fn rusl_user_agent(version: &str) -> String {
    format!("rusl-cli/{}", version.trim())
}

pub fn rusl_user_agent_with_context(version: &str, context: &str) -> String {
    let user_agent = rusl_user_agent(version);
    let context = context.trim();
    if context.is_empty() {
        user_agent
    } else {
        format!("{user_agent} ({context})")
    }
}

fn default_user_agent() -> String {
    rusl_user_agent(env!("CARGO_PKG_VERSION"))
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SessionTokens {
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
}

impl SessionTokens {
    pub fn new(access_token: Option<String>, refresh_token: Option<String>) -> Self {
        Self {
            access_token,
            refresh_token,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CliTokenExchange {
    pub access_token: Option<String>,
    pub refresh_token: String,
}

#[derive(Debug, Clone)]
pub struct RuslApiClient {
    base_url: String,
    client: reqwest::Client,
    user_agent: Option<String>,
    rusl_agent: Option<String>,
    acting_account: Option<String>,
}

impl RuslApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: normalize_base_url(base_url.into()),
            client: reqwest::Client::new(),
            user_agent: Some(default_user_agent()),
            rusl_agent: None,
            acting_account: None,
        }
    }

    pub fn with_http_client(base_url: impl Into<String>, client: reqwest::Client) -> Self {
        Self {
            base_url: normalize_base_url(base_url.into()),
            client,
            user_agent: Some(default_user_agent()),
            rusl_agent: None,
            acting_account: None,
        }
    }

    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub fn with_rusl_agent(mut self, agent: impl Into<String>) -> Self {
        self.rusl_agent = non_blank(agent.into());
        self
    }

    pub fn with_acting_account(mut self, acting_account: Option<String>) -> Self {
        self.acting_account = acting_account.and_then(non_blank);
        self
    }

    pub async fn exchange_refresh_token(&self, refresh_token: &str) -> Result<String, ApiError> {
        let response: models::AccessTokenResponse1 = self
            .execute_json(self.request(
                reqwest::Method::POST,
                "/api/v1/tokens/exchange",
                Some(refresh_token.to_string()),
            ))
            .await?;

        Ok(response.access_token)
    }

    pub async fn exchange_cli_token(
        &self,
        code: String,
        code_verifier: String,
    ) -> Result<CliTokenExchange, ApiError> {
        self.post_json(
            None,
            "/api/v1/auth/cli/token",
            &Some(models::CliTokenExchangeRequest1 {
                code,
                code_verifier,
            }),
        )
        .await
    }

    pub async fn search(
        &self,
        session: &mut SessionTokens,
        request: models::GlobalSearchRequest,
    ) -> Result<models::SearchResponse, ApiError> {
        self.with_session(session, |access_token| {
            let request = request.clone();
            async move { self.request_search(access_token, request).await }
        })
        .await
    }

    pub async fn search_schemas(
        &self,
        session: &mut SessionTokens,
        request: models::SchemaSearchRequest,
    ) -> Result<models::SearchResponse, ApiError> {
        self.with_session(session, |access_token| {
            let request = request.clone();
            async move {
                self.post_json(access_token, "/api/v1/schemas/search", &Some(request))
                    .await
            }
        })
        .await
    }

    pub async fn search_bundles(
        &self,
        session: &mut SessionTokens,
        request: models::BundleSearchRequest,
    ) -> Result<models::SearchResponse, ApiError> {
        self.with_session(session, |access_token| {
            let request = request.clone();
            async move {
                self.post_json(access_token, "/api/v1/bundles/search", &Some(request))
                    .await
            }
        })
        .await
    }

    pub async fn search_annotation_types(
        &self,
        session: &mut SessionTokens,
        request: models::AnnotationTypeSearchRequest,
    ) -> Result<models::SearchResponse, ApiError> {
        self.with_session(session, |access_token| {
            let request = request.clone();
            async move {
                self.post_json(
                    access_token,
                    "/api/v1/annotation-types/search",
                    &Some(request),
                )
                .await
            }
        })
        .await
    }

    pub async fn search_annotations(
        &self,
        session: &mut SessionTokens,
        request: models::AnnotationSearchRequest,
    ) -> Result<models::SearchResponse, ApiError> {
        self.with_session(session, |access_token| {
            let request = request.clone();
            async move {
                self.post_json(access_token, "/api/v1/annotations/search", &Some(request))
                    .await
            }
        })
        .await
    }

    pub async fn fetch_schema_record(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        schema_slug: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let path = schema_path(account_slug, schema_slug);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.get_json(access_token, &path).await }
        })
        .await
    }

    pub async fn fetch_schema_version_record(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        schema_slug: &str,
        version: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let path = schema_version_path(account_slug, schema_slug, version);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.get_json(access_token, &path).await }
        })
        .await
    }

    pub async fn fetch_bundle_record(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        bundle_slug: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let path = bundle_path(account_slug, bundle_slug);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.get_json(access_token, &path).await }
        })
        .await
    }

    pub async fn fetch_bundle_version_record(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        bundle_slug: &str,
        version: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let path = bundle_version_path(account_slug, bundle_slug, version);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.get_json(access_token, &path).await }
        })
        .await
    }

    pub async fn fetch_annotation_type_record(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        annotation_type_slug: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let path = annotation_type_path(account_slug, annotation_type_slug);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.get_json(access_token, &path).await }
        })
        .await
    }

    pub async fn fetch_annotation_record(
        &self,
        session: &mut SessionTokens,
        annotation_id: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let path = annotation_path(annotation_id);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.get_json(access_token, &path).await }
        })
        .await
    }

    pub async fn create_annotation(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        request: models::RuslWebApiAnnotationControllerCreateRequest,
    ) -> Result<AnnotationEnvelope, ApiError> {
        let path = create_annotation_path(account_slug);
        self.with_session(session, |access_token| {
            let path = path.clone();
            let request = request.clone();
            async move { self.post_json(access_token, &path, &Some(request)).await }
        })
        .await
    }

    pub async fn endorse_annotation(
        &self,
        session: &mut SessionTokens,
        annotation_id: &str,
    ) -> Result<models::RuslWebApiAnnotationControllerEndorse200Response, ApiError> {
        let path = endorse_annotation_path(annotation_id);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.post_empty(access_token, &path).await }
        })
        .await
    }

    pub async fn create_schema(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        request: models::OpenApiSchema1,
    ) -> Result<models::RuslWebApiSchemaControllerCreate201Response, ApiError> {
        let path = create_schema_path(account_slug);
        self.with_session(session, |access_token| {
            let path = path.clone();
            let request = request.clone();
            async move { self.post_json(access_token, &path, &Some(request)).await }
        })
        .await
    }

    pub async fn create_schema_proposal(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        schema_slug: &str,
        request: models::OpenApiSchema2,
    ) -> Result<models::RuslWebApiProposalControllerCreate201Response, ApiError> {
        let path = schema_proposals_path(account_slug, schema_slug);
        self.with_session(session, |access_token| {
            let path = path.clone();
            let request = request.clone();
            async move { self.post_json(access_token, &path, &Some(request)).await }
        })
        .await
    }

    pub async fn fetch_schema_proposal(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        schema_slug: &str,
        proposal_number: i32,
    ) -> Result<models::RuslWebApiProposalControllerCreate201Response, ApiError> {
        let path = schema_proposal_path(account_slug, schema_slug, proposal_number);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.get_json(access_token, &path).await }
        })
        .await
    }

    pub async fn update_schema_proposal(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        schema_slug: &str,
        proposal_number: i32,
        request: models::OpenApiSchema3,
    ) -> Result<models::RuslWebApiProposalControllerCreate201Response, ApiError> {
        let path = schema_proposal_path(account_slug, schema_slug, proposal_number);
        self.with_session(session, |access_token| {
            let path = path.clone();
            let request = request.clone();
            async move { self.patch_json(access_token, &path, &Some(request)).await }
        })
        .await
    }

    pub async fn accept_schema_proposal(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        schema_slug: &str,
        proposal_number: i32,
        request: AcceptSchemaProposalRequest,
    ) -> Result<models::RuslWebApiProposalControllerCreate201Response, ApiError> {
        let path = schema_proposal_accept_path(account_slug, schema_slug, proposal_number);
        self.with_session(session, |access_token| {
            let path = path.clone();
            let request = request.clone();
            async move { self.post_json(access_token, &path, &request).await }
        })
        .await
    }

    pub async fn create_bundle(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        request: CreateBundleRequest,
    ) -> Result<models::RuslWebApiBundleControllerShow200Response, ApiError> {
        let path = create_bundle_path(account_slug);
        self.with_session(session, |access_token| {
            let path = path.clone();
            let request = request.clone();
            async move { self.post_json(access_token, &path, &request).await }
        })
        .await
    }

    pub async fn create_annotation_type(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        request: CreateAnnotationTypeRequest,
    ) -> Result<models::RuslWebApiAnnotationTypeControllerShow200Response, ApiError> {
        let path = create_annotation_type_path(account_slug);
        self.with_session(session, |access_token| {
            let path = path.clone();
            let request = request.clone();
            async move { self.post_json(access_token, &path, &request).await }
        })
        .await
    }

    pub async fn create_bundle_version(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        bundle_slug: &str,
        request: CreateBundleVersionRequest,
    ) -> Result<models::RuslWebApiBundleVersionControllerShow200Response, ApiError> {
        let path = create_bundle_versions_path(account_slug, bundle_slug);
        self.with_session(session, |access_token| {
            let path = path.clone();
            let request = request.clone();
            async move { self.post_json(access_token, &path, &request).await }
        })
        .await
    }

    pub async fn publish_bundle_version(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        bundle_slug: &str,
        version: &str,
    ) -> Result<models::RuslWebApiBundleVersionControllerShow200Response, ApiError> {
        let path = publish_bundle_version_path(account_slug, bundle_slug, version);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.post_empty(access_token, &path).await }
        })
        .await
    }

    pub async fn list_proposal_review_threads(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        schema_slug: &str,
        proposal_number: i32,
    ) -> Result<models::RuslWebApiProposalReviewControllerIndex200Response, ApiError> {
        let path = proposal_review_threads_path(account_slug, schema_slug, proposal_number);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.get_json(access_token, &path).await }
        })
        .await
    }

    pub async fn create_proposal_review_thread(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        schema_slug: &str,
        proposal_number: i32,
        request: models::CreateReviewThreadRequest1,
    ) -> Result<models::RuslWebApiProposalReviewControllerCreateThread201Response, ApiError> {
        let path = proposal_review_threads_path(account_slug, schema_slug, proposal_number);
        self.with_session(session, |access_token| {
            let path = path.clone();
            let request = request.clone();
            async move { self.post_json(access_token, &path, &Some(request)).await }
        })
        .await
    }

    pub async fn create_proposal_review_comment(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        schema_slug: &str,
        proposal_number: i32,
        thread_id: &str,
        request: models::CreateReviewCommentRequest1,
    ) -> Result<models::RuslWebApiProposalReviewControllerCreateComment201Response, ApiError> {
        let path = proposal_review_thread_comments_path(
            account_slug,
            schema_slug,
            proposal_number,
            thread_id,
        );
        self.with_session(session, |access_token| {
            let path = path.clone();
            let request = request.clone();
            async move { self.post_json(access_token, &path, &Some(request)).await }
        })
        .await
    }

    pub async fn list_schema_examples(
        &self,
        session: &mut SessionTokens,
        account_slug: &str,
        schema_slug: &str,
        version: Option<String>,
        page: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<models::RuslWebApiSchemaVersionControllerExampleDataIndex200Response, ApiError>
    {
        let path = schema_examples_path(account_slug, schema_slug);
        let query = SchemaExamplesQuery {
            version,
            page,
            page_size,
        };
        self.with_session(session, |access_token| {
            let path = path.clone();
            let query = query.clone();
            async move {
                self.execute_json(
                    self.request(reqwest::Method::GET, &path, access_token)
                        .query(&query),
                )
                .await
            }
        })
        .await
    }

    pub async fn fetch_schema_metadata(
        &self,
        session: &mut SessionTokens,
        account: &str,
        slug: &str,
    ) -> Result<models::RuslWebRawSchemaMetadataControllerShow200Response, ApiError> {
        let path = raw_schema_metadata_path(account, slug);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.get_json(access_token, &path).await }
        })
        .await
    }

    pub async fn fetch_schema_document(
        &self,
        session: &mut SessionTokens,
        account: &str,
        schema_slug_and_version: &str,
    ) -> Result<serde_json::Value, ApiError> {
        let path = raw_schema_document_path(account, schema_slug_and_version);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.get_json(access_token, &path).await }
        })
        .await
    }

    pub async fn fetch_bundle_metadata(
        &self,
        session: &mut SessionTokens,
        account: &str,
        slug: &str,
    ) -> Result<models::RuslWebRawBundleMetadataControllerShow200Response, ApiError> {
        let path = raw_bundle_metadata_path(account, slug);
        self.with_session(session, |access_token| {
            let path = path.clone();
            async move { self.get_json(access_token, &path).await }
        })
        .await
    }

    pub async fn fetch_session_me(
        &self,
        session: &mut SessionTokens,
    ) -> Result<models::MeResponse, ApiError> {
        self.with_session(session, |access_token| async move {
            self.request_session_me(access_token).await
        })
        .await
    }

    async fn with_session<T, F, Fut>(
        &self,
        session: &mut SessionTokens,
        mut request: F,
    ) -> Result<T, ApiError>
    where
        F: FnMut(Option<String>) -> Fut,
        Fut: Future<Output = Result<T, ApiError>>,
    {
        self.ensure_access_token(session).await;

        let response = request(session.access_token.clone()).await;
        if !matches!(response, Err(ApiError::Unauthorized)) {
            return response;
        }

        if !has_refresh_token(session) {
            return response;
        }

        if self.try_exchange_refresh_token(session).await.is_err() {
            clear_tokens(session);
            return request(None).await;
        }

        request(session.access_token.clone()).await
    }

    async fn ensure_access_token(&self, session: &mut SessionTokens) {
        if has_access_token(session) || !has_refresh_token(session) {
            return;
        }

        if self.try_exchange_refresh_token(session).await.is_err() {
            clear_tokens(session);
        }
    }

    async fn try_exchange_refresh_token(
        &self,
        session: &mut SessionTokens,
    ) -> Result<(), ApiError> {
        let refresh_token = session
            .refresh_token
            .as_deref()
            .filter(|token| !token.trim().is_empty())
            .ok_or(ApiError::Unauthorized)?;

        let access_token = self.exchange_refresh_token(refresh_token).await?;
        session.access_token = Some(access_token);
        Ok(())
    }

    async fn request_session_me(
        &self,
        access_token: Option<String>,
    ) -> Result<models::MeResponse, ApiError> {
        self.get_json(access_token, "/api/v1/auth/sessions/me")
            .await
    }

    async fn request_search(
        &self,
        access_token: Option<String>,
        request: models::GlobalSearchRequest,
    ) -> Result<models::SearchResponse, ApiError> {
        // Search is public but auth-enhanced, so bearer injection has to remain in this boundary.
        self.post_json(access_token, "/api/v1/search", &Some(request))
            .await
    }

    async fn get_json<T>(&self, access_token: Option<String>, path: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        self.execute_json(self.request(reqwest::Method::GET, path, access_token))
            .await
    }

    async fn post_json<B, T>(
        &self,
        access_token: Option<String>,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.execute_json(
            self.request(reqwest::Method::POST, path, access_token)
                .json(body),
        )
        .await
    }

    async fn post_empty<T>(&self, access_token: Option<String>, path: &str) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        self.execute_json(self.request(reqwest::Method::POST, path, access_token))
            .await
    }

    async fn patch_json<B, T>(
        &self,
        access_token: Option<String>,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError>
    where
        B: Serialize + ?Sized,
        T: DeserializeOwned,
    {
        self.execute_json(
            self.request(reqwest::Method::PATCH, path, access_token)
                .json(body),
        )
        .await
    }

    async fn execute_json<T>(&self, request: reqwest::RequestBuilder) -> Result<T, ApiError>
    where
        T: DeserializeOwned,
    {
        let response = request
            .send()
            .await
            .map_err(|error| ApiError::Transport(error.to_string()))?;
        let status = response.status();
        let body = response
            .text()
            .await
            .map_err(|error| ApiError::Io(error.to_string()))?;

        if status == StatusCode::UNAUTHORIZED {
            return Err(ApiError::Unauthorized);
        }
        if status.is_client_error() || status.is_server_error() {
            return Err(ApiError::Http { status, body });
        }

        serde_json::from_str(&body).map_err(|error| ApiError::Decode(error.to_string()))
    }

    fn request(
        &self,
        method: reqwest::Method,
        path: &str,
        access_token: Option<String>,
    ) -> reqwest::RequestBuilder {
        let uri = format!("{}{}", self.base_url, path);
        let mut request = self.client.request(method, uri);

        if let Some(user_agent) = &self.user_agent {
            request = request.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(agent) = &self.rusl_agent {
            request = request.header(RUSL_AGENT_HEADER, agent.clone());
        }
        if let Some(acting_account) = &self.acting_account {
            request = request.header("X-Acting-Account-Slug", acting_account.clone());
        }
        if let Some(token) = access_token.and_then(non_blank) {
            request = request.bearer_auth(token);
        }

        request
    }
}

#[derive(Clone, Serialize)]
struct SchemaExamplesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Rusl API request was unauthorized")]
    Unauthorized,
    #[error("Rusl API request failed before receiving a response: {0}")]
    Transport(String),
    #[error("Rusl API response could not be decoded: {0}")]
    Decode(String),
    #[error("Rusl API I/O failure: {0}")]
    Io(String),
    #[error("Rusl API returned HTTP {status}: {body}")]
    Http { status: StatusCode, body: String },
}

fn normalize_base_url(base_url: String) -> String {
    base_url.trim_end_matches('/').to_string()
}

fn has_access_token(session: &SessionTokens) -> bool {
    session
        .access_token
        .as_deref()
        .is_some_and(|token| !token.trim().is_empty())
}

fn has_refresh_token(session: &SessionTokens) -> bool {
    session
        .refresh_token
        .as_deref()
        .is_some_and(|token| !token.trim().is_empty())
}

fn clear_tokens(session: &mut SessionTokens) {
    session.access_token = None;
    session.refresh_token = None;
}

fn non_blank(value: String) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_string())
    }
}

fn raw_schema_metadata_path(account: &str, slug: &str) -> String {
    format!(
        "/resources/{}/schemas/{}/metadata",
        generated::apis::urlencode(account),
        generated::apis::urlencode(slug)
    )
}

fn raw_schema_document_path(account: &str, schema_slug_and_version: &str) -> String {
    format!(
        "/resources/{}/schemas/{}",
        generated::apis::urlencode(account),
        generated::apis::urlencode(schema_slug_and_version)
    )
}

fn raw_bundle_metadata_path(account: &str, slug: &str) -> String {
    format!(
        "/resources/{}/bundles/{}/metadata",
        generated::apis::urlencode(account),
        generated::apis::urlencode(slug)
    )
}

fn create_annotation_path(account_slug: &str) -> String {
    format!(
        "/api/{}/annotations",
        generated::apis::urlencode(account_slug)
    )
}

fn endorse_annotation_path(annotation_id: &str) -> String {
    format!(
        "/api/v1/annotations/{}/endorse",
        generated::apis::urlencode(annotation_id)
    )
}

fn schema_path(account_slug: &str, schema_slug: &str) -> String {
    format!(
        "/api/{}/schemas/{}",
        generated::apis::urlencode(account_slug),
        generated::apis::urlencode(schema_slug)
    )
}

fn schema_version_path(account_slug: &str, schema_slug: &str, version: &str) -> String {
    format!(
        "{}/versions/{}",
        schema_path(account_slug, schema_slug),
        generated::apis::urlencode(version)
    )
}

fn bundle_path(account_slug: &str, bundle_slug: &str) -> String {
    format!(
        "/api/{}/bundles/{}",
        generated::apis::urlencode(account_slug),
        generated::apis::urlencode(bundle_slug)
    )
}

fn bundle_version_path(account_slug: &str, bundle_slug: &str, version: &str) -> String {
    format!(
        "{}/versions/{}",
        bundle_path(account_slug, bundle_slug),
        generated::apis::urlencode(version)
    )
}

fn annotation_type_path(account_slug: &str, annotation_type_slug: &str) -> String {
    format!(
        "/api/{}/annotation_types/{}",
        generated::apis::urlencode(account_slug),
        generated::apis::urlencode(annotation_type_slug)
    )
}

fn annotation_path(annotation_id: &str) -> String {
    format!(
        "/api/v1/annotations/{}",
        generated::apis::urlencode(annotation_id)
    )
}

fn create_schema_path(account_slug: &str) -> String {
    format!("/api/{}/schemas", generated::apis::urlencode(account_slug))
}

fn schema_proposals_path(account_slug: &str, schema_slug: &str) -> String {
    format!(
        "/api/{}/schemas/{}/proposals",
        generated::apis::urlencode(account_slug),
        generated::apis::urlencode(schema_slug)
    )
}

fn schema_proposal_path(account_slug: &str, schema_slug: &str, proposal_number: i32) -> String {
    format!(
        "{}/{}",
        schema_proposals_path(account_slug, schema_slug),
        proposal_number
    )
}

fn schema_proposal_accept_path(
    account_slug: &str,
    schema_slug: &str,
    proposal_number: i32,
) -> String {
    format!(
        "{}/accept",
        schema_proposal_path(account_slug, schema_slug, proposal_number)
    )
}

fn create_bundle_path(account_slug: &str) -> String {
    format!("/api/{}/bundles", generated::apis::urlencode(account_slug))
}

fn create_annotation_type_path(account_slug: &str) -> String {
    format!(
        "/api/{}/annotation_types",
        generated::apis::urlencode(account_slug)
    )
}

fn create_bundle_versions_path(account_slug: &str, bundle_slug: &str) -> String {
    format!("{}/versions", bundle_path(account_slug, bundle_slug))
}

fn publish_bundle_version_path(account_slug: &str, bundle_slug: &str, version: &str) -> String {
    format!(
        "{}/publish",
        bundle_version_path(account_slug, bundle_slug, version)
    )
}

fn proposal_review_threads_path(
    account_slug: &str,
    schema_slug: &str,
    proposal_number: i32,
) -> String {
    format!(
        "/api/{}/schemas/{}/proposals/{}/review_threads",
        generated::apis::urlencode(account_slug),
        generated::apis::urlencode(schema_slug),
        proposal_number
    )
}

fn proposal_review_thread_comments_path(
    account_slug: &str,
    schema_slug: &str,
    proposal_number: i32,
    thread_id: &str,
) -> String {
    format!(
        "{}/{}/comments",
        proposal_review_threads_path(account_slug, schema_slug, proposal_number),
        generated::apis::urlencode(thread_id)
    )
}

fn schema_examples_path(account_slug: &str, schema_slug: &str) -> String {
    format!(
        "/api/{}/schemas/{}/example_data",
        generated::apis::urlencode(account_slug),
        generated::apis::urlencode(schema_slug)
    )
}

#[cfg(test)]
mod tests {
    use super::{
        annotation_path, annotation_type_path, bundle_path, bundle_version_path,
        create_annotation_path, create_annotation_type_path, create_bundle_path,
        create_bundle_versions_path, create_schema_path, endorse_annotation_path,
        normalize_base_url, proposal_review_thread_comments_path, proposal_review_threads_path,
        publish_bundle_version_path, raw_bundle_metadata_path, raw_schema_document_path,
        raw_schema_metadata_path, rusl_user_agent, rusl_user_agent_with_context,
        schema_examples_path, schema_path, schema_proposal_accept_path, schema_proposal_path,
        schema_proposals_path, schema_version_path,
    };

    #[test]
    fn trims_trailing_slashes_from_base_urls() {
        assert_eq!(
            normalize_base_url("http://localhost:4000/".to_string()),
            "http://localhost:4000"
        );
        assert_eq!(
            normalize_base_url("https://resources.rusl.com///".to_string()),
            "https://resources.rusl.com"
        );
    }

    #[test]
    fn formats_versioned_user_agents() {
        assert_eq!(rusl_user_agent("0.1.0"), "rusl-cli/0.1.0");
        assert_eq!(
            rusl_user_agent_with_context("0.1.0", "mcp"),
            "rusl-cli/0.1.0 (mcp)"
        );
        assert_eq!(rusl_user_agent_with_context("0.1.0", " "), "rusl-cli/0.1.0");
    }

    #[test]
    fn builds_encoded_raw_resource_paths() {
        assert_eq!(
            raw_schema_metadata_path("hass ox", "common/schema"),
            "/resources/hass+ox/schemas/common%2Fschema/metadata"
        );
        assert_eq!(
            raw_schema_document_path("hass ox", "common@v1.0.0"),
            "/resources/hass+ox/schemas/common%40v1.0.0"
        );
        assert_eq!(
            raw_bundle_metadata_path("hass ox", "bundle/main"),
            "/resources/hass+ox/bundles/bundle%2Fmain/metadata"
        );
        assert_eq!(
            create_annotation_path("hass ox"),
            "/api/hass+ox/annotations"
        );
        assert_eq!(
            endorse_annotation_path("annotations.123/456"),
            "/api/v1/annotations/annotations.123%2F456/endorse"
        );
        assert_eq!(
            schema_path("hass ox", "common/schema"),
            "/api/hass+ox/schemas/common%2Fschema"
        );
        assert_eq!(
            schema_version_path("hass ox", "common/schema", "1.2.3"),
            "/api/hass+ox/schemas/common%2Fschema/versions/1.2.3"
        );
        assert_eq!(
            bundle_path("hass ox", "bundle/main"),
            "/api/hass+ox/bundles/bundle%2Fmain"
        );
        assert_eq!(
            bundle_version_path("hass ox", "bundle/main", "1.2.3"),
            "/api/hass+ox/bundles/bundle%2Fmain/versions/1.2.3"
        );
        assert_eq!(
            annotation_type_path("hass ox", "review/type"),
            "/api/hass+ox/annotation_types/review%2Ftype"
        );
        assert_eq!(
            annotation_path("annotations.123/456"),
            "/api/v1/annotations/annotations.123%2F456"
        );
        assert_eq!(create_schema_path("hass ox"), "/api/hass+ox/schemas");
        assert_eq!(
            schema_proposals_path("hass ox", "common/schema"),
            "/api/hass+ox/schemas/common%2Fschema/proposals"
        );
        assert_eq!(
            schema_proposal_path("hass ox", "common/schema", 42),
            "/api/hass+ox/schemas/common%2Fschema/proposals/42"
        );
        assert_eq!(
            proposal_review_threads_path("hass ox", "common/schema", 42),
            "/api/hass+ox/schemas/common%2Fschema/proposals/42/review_threads"
        );
        assert_eq!(
            proposal_review_thread_comments_path("hass ox", "common/schema", 42, "thread/1"),
            "/api/hass+ox/schemas/common%2Fschema/proposals/42/review_threads/thread%2F1/comments"
        );
        assert_eq!(
            schema_examples_path("hass ox", "common/schema"),
            "/api/hass+ox/schemas/common%2Fschema/example_data"
        );
        assert_eq!(
            schema_proposal_accept_path("hass ox", "common/schema", 1),
            "/api/hass+ox/schemas/common%2Fschema/proposals/1/accept"
        );
        assert_eq!(create_bundle_path("hass ox"), "/api/hass+ox/bundles");
        assert_eq!(
            create_annotation_type_path("hass ox"),
            "/api/hass+ox/annotation_types"
        );
        assert_eq!(
            create_bundle_versions_path("hass ox", "bundle/main"),
            "/api/hass+ox/bundles/bundle%2Fmain/versions"
        );
        assert_eq!(
            publish_bundle_version_path("hass ox", "bundle/main", "1.0.0"),
            "/api/hass+ox/bundles/bundle%2Fmain/versions/1.0.0/publish"
        );
    }
}
