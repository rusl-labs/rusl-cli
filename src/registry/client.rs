use crate::config::Config;
use crate::config::credentials::Credentials;
use anyhow::{Context, Result};
use colored::Colorize;
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

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

#[derive(Deserialize)]
struct TokenExchangeResponse {
    access_token: String,
}

pub struct RegistryClient {
    client: Client,
    config: Config,
}

impl RegistryClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    /// Append the `Authorization: Bearer <token>` header when credentials exist.
    fn inject_auth(&self, req: RequestBuilder, creds: &Option<Credentials>) -> RequestBuilder {
        if let Some(c) = creds {
            req.bearer_auth(&c.access_token)
        } else {
            req
        }
    }

    /// Execute the request and retry once after refreshing on `401`.
    async fn execute_with_auth_retry(
        &self,
        req_builder: RequestBuilder,
    ) -> Result<reqwest::Response> {
        let mut creds = Credentials::load();

        if let Some(ref mut current_creds) = creds
            && current_creds.access_token.trim().is_empty()
            && !current_creds.refresh_token.trim().is_empty()
        {
            let refresh_url = format!("{}/api/tokens/exchange", self.config.api_base_url);
            let refresh_res = self
                .client
                .post(&refresh_url)
                .bearer_auth(&current_creds.refresh_token)
                .send()
                .await?;

            if refresh_res.status().is_success() {
                let token_data: TokenExchangeResponse = refresh_res
                    .json()
                    .await
                    .context("Failed to deserialize token exchange json")?;
                current_creds.access_token = token_data.access_token;
                let _ = current_creds.save();
            } else {
                println!(
                    "{} Your session has expired. You may need to run `rusl login` again.",
                    "Warning:".yellow().bold()
                );
            }
        }

        let initial_req = req_builder
            .try_clone()
            .context("Failed to securely duplicate HTTP request payload")?;

        let mut res = self.inject_auth(initial_req, &creds).send().await?;

        if res.status() == StatusCode::UNAUTHORIZED
            && let Some(mut current_creds) = creds.take()
        {
            debug!("Received 401 Unauthorized. Attempting refresh-token exchange.");
            let refresh_url = format!("{}/api/tokens/exchange", self.config.api_base_url);

            let refresh_res = self
                .client
                .post(&refresh_url)
                .bearer_auth(&current_creds.refresh_token)
                .send()
                .await?;

            if refresh_res.status().is_success() {
                let token_data: TokenExchangeResponse = refresh_res
                    .json()
                    .await
                    .context("Failed to deserialize refreshed API token json natively")?;

                current_creds.access_token = token_data.access_token.clone();
                current_creds
                    .save()
                    .context("Failed to persist refreshed session")?;

                creds = Some(current_creds);
                let retry_req = req_builder
                    .try_clone()
                    .context("Failed to clone HTTP request payload")?;
                res = self.inject_auth(retry_req, &creds).send().await?;
            } else {
                println!(
                    "{} Session refresh failed (HTTP {}). Please run `rusl login` to re-authenticate.",
                    "Error:".red().bold(),
                    refresh_res.status()
                );
            }
        }

        res.error_for_status()
            .map_err(|e| anyhow::anyhow!("Registry request failed: {}", e))
    }

    /// Fetch the version history for a schema package.
    pub async fn fetch_schema_meta(
        &self,
        account: &str,
        slug: &str,
    ) -> Result<RegistryMetadataResponse> {
        let url = format!(
            "{}/schemas/{}/{}/metadata",
            self.config.api_base_url, account, slug
        );
        let req = self.client.get(&url);
        let res = self.execute_with_auth_retry(req).await?;
        res.json().await.context("Failed to parse schema metadata")
    }

    /// Fetch the version history for a bundle package.
    pub async fn fetch_bundle_meta(
        &self,
        account: &str,
        slug: &str,
    ) -> Result<RegistryMetadataResponse> {
        let url = format!(
            "{}/bundles/{}/{}/metadata",
            self.config.api_base_url, account, slug
        );
        let req = self.client.get(&url);
        let res = self.execute_with_auth_retry(req).await?;
        res.json().await.context("Failed to parse bundle metadata")
    }

    /// Download a schema artifact at an exact version.
    pub async fn download_schema_blob(
        &self,
        account: &str,
        slug: &str,
        version: &str,
    ) -> Result<Vec<u8>> {
        let url = format!(
            "{}/schemas/{}/{}@v{}",
            self.config.api_base_url, account, slug, version
        );
        let req = self.client.get(&url);
        let res = self.execute_with_auth_retry(req).await?;
        let bytes = res
            .bytes()
            .await
            .context(format!("Failed to download raw blob from {}", url))?;
        Ok(bytes.to_vec())
    }

    /// Fetch the current authenticated session.
    pub async fn fetch_me(&self) -> Result<serde_json::Value> {
        let url = format!("{}/api/auth/sessions/me", self.config.api_base_url);
        let req = self.client.get(&url);
        let res = self.execute_with_auth_retry(req).await?;
        res.json()
            .await
            .context("Failed to deserialize session profile")
    }
}
