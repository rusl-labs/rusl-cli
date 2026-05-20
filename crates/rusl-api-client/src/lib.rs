use reqwest::StatusCode;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::future::Future;

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
}

impl RuslApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: normalize_base_url(base_url.into()),
            client: reqwest::Client::new(),
            user_agent: Some(default_user_agent()),
            rusl_agent: None,
        }
    }

    pub fn with_http_client(base_url: impl Into<String>, client: reqwest::Client) -> Self {
        Self {
            base_url: normalize_base_url(base_url.into()),
            client,
            user_agent: Some(default_user_agent()),
            rusl_agent: None,
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

    pub async fn exchange_refresh_token(&self, refresh_token: &str) -> Result<String, ApiError> {
        let response: models::AccessTokenResponse1 = self
            .execute_json(self.request(
                reqwest::Method::POST,
                "/api/tokens/exchange",
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
            "/api/auth/cli/token",
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
                self.post_json(access_token, "/api/schemas/search", &Some(request))
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
                self.post_json(access_token, "/api/bundles/search", &Some(request))
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
                self.post_json(access_token, "/api/annotation-types/search", &Some(request))
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
                self.post_json(access_token, "/api/annotations/search", &Some(request))
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
        self.get_json(access_token, "/api/auth/sessions/me").await
    }

    async fn request_search(
        &self,
        access_token: Option<String>,
        request: models::GlobalSearchRequest,
    ) -> Result<models::SearchResponse, ApiError> {
        // Search is public but auth-enhanced, so bearer injection has to remain in this boundary.
        self.post_json(access_token, "/api/search", &Some(request))
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
        if let Some(token) = access_token.and_then(non_blank) {
            request = request.bearer_auth(token);
        }

        request
    }
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
        "/resources/{}/{}/metadata",
        generated::apis::urlencode(account),
        generated::apis::urlencode(slug)
    )
}

fn raw_schema_document_path(account: &str, schema_slug_and_version: &str) -> String {
    format!(
        "/resources/{}/{}",
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

#[cfg(test)]
mod tests {
    use super::{
        normalize_base_url, raw_bundle_metadata_path, raw_schema_document_path,
        raw_schema_metadata_path, rusl_user_agent, rusl_user_agent_with_context,
    };

    #[test]
    fn trims_trailing_slashes_from_base_urls() {
        assert_eq!(
            normalize_base_url("http://localhost:4000/".to_string()),
            "http://localhost:4000"
        );
        assert_eq!(
            normalize_base_url("https://api.rusl.app///".to_string()),
            "https://api.rusl.app"
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
            "/resources/hass+ox/common%2Fschema/metadata"
        );
        assert_eq!(
            raw_schema_document_path("hass ox", "common@v1.0.0"),
            "/resources/hass+ox/common%40v1.0.0"
        );
        assert_eq!(
            raw_bundle_metadata_path("hass ox", "bundle/main"),
            "/resources/hass+ox/bundles/bundle%2Fmain/metadata"
        );
    }
}
