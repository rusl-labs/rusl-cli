use crate::config::Config;
use crate::config::credentials::Credentials;
use anyhow::{Context, Result, anyhow};
use rusl_api_client::{ApiError, RuslApiClient, SessionTokens, models};
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
}

impl RegistryClient {
    pub fn new(config: Config) -> Self {
        Self {
            api: RuslApiClient::new(config.api_base_url),
        }
    }

    pub async fn fetch_schema_meta(
        &self,
        account: &str,
        slug: &str,
    ) -> Result<RegistryMetadataResponse> {
        let (mut credentials, mut session) = self.load_session()?;
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
        let (mut credentials, mut session) = self.load_session()?;
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
        let (mut credentials, mut session) = self.load_session()?;
        let schema_slug_and_version = format!("{slug}@v{version}");
        let document = self
            .api
            .fetch_schema_document(&mut session, account, &schema_slug_and_version)
            .await
            .map_err(map_api_error)
            .with_context(|| format!("Failed to download schema {account}/{slug}@v{version}"))?;

        self.persist_session(&mut credentials, &session)?;
        serde_json::to_vec(&document).context("Failed to serialize raw schema document")
    }

    pub async fn fetch_me(&self) -> Result<models::MeResponse> {
        let (mut credentials, mut session) = self.load_session()?;
        let me = self
            .api
            .fetch_session_me(&mut session)
            .await
            .map_err(map_api_error)
            .context("Failed to fetch current session")?;

        self.persist_session(&mut credentials, &session)?;
        Ok(me)
    }

    fn load_session(&self) -> Result<(Option<Credentials>, SessionTokens)> {
        let credentials = Credentials::load();
        let session = session_tokens(&credentials);
        Ok((credentials, session))
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
        let next_refresh_token = session.refresh_token.as_deref().unwrap_or_default();
        if stored.access_token == next_access_token && stored.refresh_token == next_refresh_token {
            return Ok(());
        }

        stored.access_token = next_access_token.to_string();
        stored.refresh_token = next_refresh_token.to_string();
        stored.save().context("Failed to persist refreshed session")
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

fn map_api_error(error: ApiError) -> anyhow::Error {
    anyhow!(error)
}
