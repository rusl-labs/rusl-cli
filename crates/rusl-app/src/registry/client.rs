use crate::config::Config;
use crate::config::credentials::Credentials;
use anyhow::{Context, Result, anyhow};
use colored::Colorize;
use rusl_api_client::{ApiError, RuslApiClient, SessionTokens, generated, models};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryVersion {
    pub version: String,
    #[serde(default)]
    pub schemas: HashMap<String, String>,
    #[serde(default)]
    pub bundles: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryMetadataResponse {
    pub name: String,
    pub versions: Vec<RegistryVersion>,
}

pub struct RegistryClient {
    api: RuslApiClient,
    api_base_url: String,
}

impl RegistryClient {
    pub fn new(config: Config) -> Self {
        let api_base_url = config.api_base_url;
        let api = RuslApiClient::new(api_base_url.clone());
        Self { api, api_base_url }
    }

    pub async fn fetch_schema_meta(
        &self,
        account: &str,
        slug: &str,
    ) -> Result<RegistryMetadataResponse> {
        let (mut credentials, mut session) = self.load_session().await?;
        let metadata = self
            .api
            .fetch_schema_metadata(&mut session, account, slug)
            .await
            .map_err(map_api_error)
            .with_context(|| format!("Failed to fetch schema metadata for {account}/{slug}"))?;

        self.persist_session(&mut credentials, &session)?;
        Ok(map_metadata_response(metadata.name, metadata.versions))
    }

    pub async fn fetch_bundle_meta(
        &self,
        account: &str,
        slug: &str,
    ) -> Result<RegistryMetadataResponse> {
        let (mut credentials, mut session) = self.load_session().await?;
        let metadata = self
            .api
            .fetch_bundle_metadata(&mut session, account, slug)
            .await
            .map_err(map_api_error)
            .with_context(|| format!("Failed to fetch bundle metadata for {account}/{slug}"))?;

        self.persist_session(&mut credentials, &session)?;
        Ok(map_metadata_response(metadata.name, metadata.versions))
    }

    pub async fn download_schema_blob(
        &self,
        account: &str,
        slug: &str,
        version: &str,
    ) -> Result<Vec<u8>> {
        let (mut credentials, mut session) = self.load_session().await?;
        let schema_slug_and_version = format!("{slug}@v{version}");
        let document = self
            .fetch_schema_document(&mut session, account, &schema_slug_and_version)
            .await
            .with_context(|| format!("Failed to download schema {account}/{slug}@v{version}"))?;

        self.persist_session(&mut credentials, &session)?;
        serde_json::to_vec(&document).context("Failed to serialize raw schema document")
    }

    pub async fn fetch_me(&self) -> Result<serde_json::Value> {
        let (mut credentials, mut session) = self.load_session().await?;
        let me = self
            .api
            .fetch_session_me(&mut session)
            .await
            .map_err(map_api_error)
            .context("Failed to fetch current session")?;

        self.persist_session(&mut credentials, &session)?;
        serde_json::to_value(me).context("Failed to serialize session profile")
    }

    async fn load_session(&self) -> Result<(Option<Credentials>, SessionTokens)> {
        let mut credentials = Credentials::load();
        let mut session = session_tokens(&credentials);

        let needs_access_token = session
            .access_token
            .as_deref()
            .is_none_or(|token| token.trim().is_empty());

        if needs_access_token
            && let Some(refresh_token) = session
                .refresh_token
                .as_deref()
                .filter(|token| !token.trim().is_empty())
        {
            match self.api.exchange_refresh_token(refresh_token).await {
                Ok(access_token) => {
                    session.access_token = Some(access_token);
                    self.persist_session(&mut credentials, &session)?;
                }
                Err(ApiError::Unauthorized) => {
                    println!(
                        "{} Your session has expired. You may need to run `rusl login` again.",
                        "Warning:".yellow().bold()
                    );
                }
                Err(err) => return Err(map_api_error(err)),
            }
        }

        Ok((credentials, session))
    }

    async fn fetch_schema_document(
        &self,
        session: &mut SessionTokens,
        account: &str,
        schema_slug_and_version: &str,
    ) -> Result<serde_json::Value> {
        let config = self.generated_configuration(session.access_token.as_deref());
        let response = generated::apis::raw_schemas_api::rusl_web_raw_schema_controller_show(
            &config,
            account,
            schema_slug_and_version,
        )
        .await;

        match response {
            Ok(document) => Ok(document),
            Err(error) if is_unauthorized(&error) => {
                self.refresh_session(session).await?;
                let retry_config = self.generated_configuration(session.access_token.as_deref());
                generated::apis::raw_schemas_api::rusl_web_raw_schema_controller_show(
                    &retry_config,
                    account,
                    schema_slug_and_version,
                )
                .await
                .map_err(map_generated_error)
            }
            Err(error) => Err(map_generated_error(error)),
        }
    }

    async fn refresh_session(&self, session: &mut SessionTokens) -> Result<()> {
        let refresh_token = session
            .refresh_token
            .as_deref()
            .filter(|token| !token.trim().is_empty())
            .ok_or_else(|| anyhow!("Missing refresh token. Please run `rusl login` again."))?;

        let access_token = self
            .api
            .exchange_refresh_token(refresh_token)
            .await
            .map_err(map_api_error)?;
        session.access_token = Some(access_token);
        Ok(())
    }

    fn persist_session(
        &self,
        credentials: &mut Option<Credentials>,
        session: &SessionTokens,
    ) -> Result<()> {
        let Some(stored) = credentials.as_mut() else {
            return Ok(());
        };

        let next_access_token = session.access_token.as_deref().unwrap_or_default();
        if stored.access_token == next_access_token {
            return Ok(());
        }

        stored.access_token = next_access_token.to_string();
        stored.save().context("Failed to persist refreshed session")
    }

    fn generated_configuration(
        &self,
        bearer_access_token: Option<&str>,
    ) -> generated::apis::configuration::Configuration {
        generated::apis::configuration::Configuration {
            base_path: self.api_base_url.clone(),
            user_agent: Some(format!("rusl/{}", env!("CARGO_PKG_VERSION"))),
            client: reqwest::Client::new(),
            basic_auth: None,
            oauth_access_token: None,
            bearer_access_token: bearer_access_token.map(ToOwned::to_owned),
            api_key: None,
        }
    }
}

fn session_tokens(credentials: &Option<Credentials>) -> SessionTokens {
    match credentials {
        Some(credentials) => SessionTokens::new(
            Some(credentials.access_token.clone()),
            Some(credentials.refresh_token.clone()),
        ),
        None => SessionTokens::default(),
    }
}

fn map_metadata_response(
    name: Option<String>,
    versions: Option<Vec<models::RuslWebRawSchemaMetadataControllerShow200ResponseVersionsInner>>,
) -> RegistryMetadataResponse {
    RegistryMetadataResponse {
        name: name.unwrap_or_default(),
        versions: versions
            .unwrap_or_default()
            .into_iter()
            .map(map_version)
            .collect(),
    }
}

fn map_version(
    version: models::RuslWebRawSchemaMetadataControllerShow200ResponseVersionsInner,
) -> RegistryVersion {
    RegistryVersion {
        version: version.version.unwrap_or_default(),
        schemas: version.schemas.unwrap_or_default(),
        bundles: version.bundles.unwrap_or_default(),
    }
}

fn is_unauthorized<E>(error: &generated::apis::Error<E>) -> bool {
    matches!(
        error,
        generated::apis::Error::ResponseError(content)
            if content.status == reqwest::StatusCode::UNAUTHORIZED
    )
}

fn map_api_error(error: ApiError) -> anyhow::Error {
    anyhow!(error)
}

fn map_generated_error<E>(error: generated::apis::Error<E>) -> anyhow::Error
where
    E: std::fmt::Debug,
{
    match error {
        generated::apis::Error::Reqwest(err) => anyhow!("Rusl API request failed: {err}"),
        generated::apis::Error::Serde(err) => anyhow!("Rusl API decode failed: {err}"),
        generated::apis::Error::Io(err) => anyhow!("Rusl API I/O failed: {err}"),
        generated::apis::Error::ResponseError(content) => anyhow!(
            "Rusl API returned HTTP {}: {}",
            content.status,
            content.content
        ),
    }
}
