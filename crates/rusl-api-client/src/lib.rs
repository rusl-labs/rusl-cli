use reqwest::StatusCode;
use std::fmt;

pub use rusl_openapi_client as generated;
pub use rusl_openapi_client::models;

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
            user_agent: Some(format!("rusl/{}", env!("CARGO_PKG_VERSION"))),
        }
    }

    pub fn with_http_client(base_url: impl Into<String>, client: reqwest::Client) -> Self {
        Self {
            base_url: normalize_base_url(base_url.into()),
            client,
            user_agent: Some(format!("rusl/{}", env!("CARGO_PKG_VERSION"))),
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

    pub async fn fetch_schema_metadata(
        &self,
        session: &mut SessionTokens,
        account: &str,
        slug: &str,
    ) -> Result<models::RuslWebRawSchemaMetadataControllerShow200Response, ApiError> {
        let config = self.configuration(session.access_token.as_deref());
        let response =
            generated::apis::raw_schemas_api::rusl_web_raw_schema_metadata_controller_show(
                &config, account, slug,
            )
            .await;

        match response {
            Ok(metadata) => Ok(metadata),
            Err(error) if is_unauthorized(&error) => {
                self.refresh_session(session).await?;
                let retry_config = self.configuration(session.access_token.as_deref());
                generated::apis::raw_schemas_api::rusl_web_raw_schema_metadata_controller_show(
                    &retry_config,
                    account,
                    slug,
                )
                .await
                .map_err(map_api_error)
            }
            Err(error) => Err(map_api_error(error)),
        }
    }

    pub async fn fetch_bundle_metadata(
        &self,
        session: &mut SessionTokens,
        account: &str,
        slug: &str,
    ) -> Result<models::RuslWebRawBundleMetadataControllerShow200Response, ApiError> {
        let config = self.configuration(session.access_token.as_deref());
        let response =
            generated::apis::raw_bundles_api::rusl_web_raw_bundle_metadata_controller_show(
                &config, account, slug,
            )
            .await;

        match response {
            Ok(metadata) => Ok(metadata),
            Err(error) if is_unauthorized(&error) => {
                self.refresh_session(session).await?;
                let retry_config = self.configuration(session.access_token.as_deref());
                generated::apis::raw_bundles_api::rusl_web_raw_bundle_metadata_controller_show(
                    &retry_config,
                    account,
                    slug,
                )
                .await
                .map_err(map_api_error)
            }
            Err(error) => Err(map_api_error(error)),
        }
    }

    pub async fn fetch_session_me(
        &self,
        session: &mut SessionTokens,
    ) -> Result<models::MeResponse, ApiError> {
        let config = self.configuration(session.access_token.as_deref());
        let response =
            generated::apis::authentication_api::rusl_web_api_session_controller_me(&config).await;

        match response {
            Ok(me) => Ok(me),
            Err(error) if is_unauthorized(&error) => {
                self.refresh_session(session).await?;
                let retry_config = self.configuration(session.access_token.as_deref());
                generated::apis::authentication_api::rusl_web_api_session_controller_me(
                    &retry_config,
                )
                .await
                .map_err(map_api_error)
            }
            Err(error) => Err(map_api_error(error)),
        }
    }

    async fn refresh_session(&self, session: &mut SessionTokens) -> Result<(), ApiError> {
        let refresh_token = session
            .refresh_token
            .as_deref()
            .filter(|token| !token.trim().is_empty())
            .ok_or(ApiError::Unauthorized)?;

        let access_token = self.exchange_refresh_token(refresh_token).await?;
        session.access_token = Some(access_token);
        Ok(())
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

fn is_unauthorized<E>(error: &generated::apis::Error<E>) -> bool {
    matches!(
        error,
        generated::apis::Error::ResponseError(content) if content.status == StatusCode::UNAUTHORIZED
    )
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
    use super::normalize_base_url;

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
}
