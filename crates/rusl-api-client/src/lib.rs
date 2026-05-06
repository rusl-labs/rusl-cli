use reqwest::StatusCode;
use std::fmt;
use std::future::Future;

pub use rusl_openapi_client as generated;
pub use rusl_openapi_client::models;

pub fn rusl_user_agent(version: &str) -> String {
    format!("rusl/{}", version.trim())
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

#[derive(Debug, Clone)]
pub struct RuslApiClient {
    base_url: String,
    client: reqwest::Client,
    user_agent: Option<String>,
}

impl RuslApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: normalize_base_url(base_url.into()),
            client: reqwest::Client::new(),
            user_agent: Some(default_user_agent()),
        }
    }

    pub fn with_http_client(base_url: impl Into<String>, client: reqwest::Client) -> Self {
        Self {
            base_url: normalize_base_url(base_url.into()),
            client,
            user_agent: Some(default_user_agent()),
        }
    }

    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    pub async fn exchange_refresh_token(&self, refresh_token: &str) -> Result<String, ApiError> {
        let config = self.configuration(Some(refresh_token));
        let response =
            generated::apis::authentication_api::rusl_web_api_tokens_controller_exchange(&config)
                .await
                .map_err(map_api_error)?;

        Ok(response.access_token)
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

    pub async fn fetch_schema_metadata(
        &self,
        session: &mut SessionTokens,
        account: &str,
        slug: &str,
    ) -> Result<models::RuslWebRawSchemaMetadataControllerShow200Response, ApiError> {
        self.with_session(session, |access_token| async move {
            let config = self.configuration(access_token.as_deref());
            generated::apis::raw_schemas_api::rusl_web_raw_schema_metadata_controller_show(
                &config, account, slug,
            )
            .await
            .map_err(map_api_error)
        })
        .await
    }

    pub async fn fetch_schema_document(
        &self,
        session: &mut SessionTokens,
        account: &str,
        schema_slug_and_version: &str,
    ) -> Result<serde_json::Value, ApiError> {
        self.with_session(session, |access_token| async move {
            let config = self.configuration(access_token.as_deref());
            generated::apis::raw_schemas_api::rusl_web_raw_schema_controller_show(
                &config,
                account,
                schema_slug_and_version,
            )
            .await
            .map_err(map_api_error)
        })
        .await
    }

    pub async fn fetch_bundle_metadata(
        &self,
        session: &mut SessionTokens,
        account: &str,
        slug: &str,
    ) -> Result<models::RuslWebRawBundleMetadataControllerShow200Response, ApiError> {
        self.with_session(session, |access_token| async move {
            let config = self.configuration(access_token.as_deref());
            generated::apis::raw_bundles_api::rusl_web_raw_bundle_metadata_controller_show(
                &config, account, slug,
            )
            .await
            .map_err(map_api_error)
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
        let uri = format!("{}/api/auth/sessions/me", self.base_url);
        let mut request = self.client.get(&uri);

        if let Some(user_agent) = &self.user_agent {
            request = request.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(token) = access_token.filter(|token| !token.trim().is_empty()) {
            request = request.bearer_auth(token);
        }

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

    async fn request_search(
        &self,
        access_token: Option<String>,
        request: models::GlobalSearchRequest,
    ) -> Result<models::SearchResponse, ApiError> {
        // Search is public but auth-enhanced, so bearer injection has to remain in this boundary.
        let uri = format!("{}/api/search", self.base_url);
        let mut http_request = self.client.post(&uri);

        if let Some(user_agent) = &self.user_agent {
            http_request = http_request.header(reqwest::header::USER_AGENT, user_agent.clone());
        }
        if let Some(token) = access_token.filter(|token| !token.trim().is_empty()) {
            http_request = http_request.bearer_auth(token);
        }

        let response = http_request
            .json(&Some(request))
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

    fn configuration(
        &self,
        bearer_access_token: Option<&str>,
    ) -> generated::apis::configuration::Configuration {
        generated::apis::configuration::Configuration {
            base_path: self.base_url.clone(),
            user_agent: self.user_agent.clone(),
            client: self.client.clone(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: bearer_access_token.map(ToOwned::to_owned),
            api_key: None,
        }
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

fn map_api_error<E>(error: generated::apis::Error<E>) -> ApiError
where
    E: fmt::Debug,
{
    match error {
        generated::apis::Error::Reqwest(err) => ApiError::Transport(err.to_string()),
        generated::apis::Error::Serde(err) => ApiError::Decode(err.to_string()),
        generated::apis::Error::Io(err) => ApiError::Io(err.to_string()),
        generated::apis::Error::ResponseError(content) => {
            if content.status == StatusCode::UNAUTHORIZED {
                ApiError::Unauthorized
            } else {
                ApiError::Http {
                    status: content.status,
                    body: content.content,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize_base_url, rusl_user_agent, rusl_user_agent_with_context};

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
        assert_eq!(rusl_user_agent("0.1.0"), "rusl/0.1.0");
        assert_eq!(
            rusl_user_agent_with_context("0.1.0", "mcp"),
            "rusl/0.1.0 (mcp)"
        );
        assert_eq!(rusl_user_agent_with_context("0.1.0", " "), "rusl/0.1.0");
    }
}
