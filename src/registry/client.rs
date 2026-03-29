use crate::config::Config;
use anyhow::{Context, Result};
use reqwest::Client;
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

    /// Fetches the index of all available semantic versions for a Schema via the true Metadata API
    pub async fn fetch_schema_meta(
        &self,
        account: &str,
        slug: &str,
    ) -> Result<RegistryMetadataResponse> {
        let url = format!(
            "{}/schemas/{}/{}/metadata",
            self.config.registry_url, account, slug
        );
        let res = self.client.get(&url).send().await?.error_for_status()?;
        let meta: RegistryMetadataResponse = res
            .json()
            .await
            .context("Failed to parse Schema Metadata json")?;
        Ok(meta)
    }

    /// Fetches the index of all available semantic versions for a Bundle natively
    pub async fn fetch_bundle_meta(
        &self,
        account: &str,
        slug: &str,
    ) -> Result<RegistryMetadataResponse> {
        let url = format!(
            "{}/bundles/{}/{}/metadata",
            self.config.registry_url, account, slug
        );
        let res = self.client.get(&url).send().await?.error_for_status()?;
        let meta: RegistryMetadataResponse = res
            .json()
            .await
            .context("Failed to parse Bundle Metadata json")?;
        Ok(meta)
    }

    /// Downloads the exact JSON/YAML payload mapping isolated to a specific semantic version natively via `@v{version}`
    pub async fn download_schema_blob(
        &self,
        account: &str,
        slug: &str,
        version: &str,
    ) -> Result<Vec<u8>> {
        let url = format!(
            "{}/schemas/{}/{}@v{}",
            self.config.registry_url, account, slug, version
        );
        let res = self.client.get(&url).send().await?.error_for_status()?;
        let bytes = res
            .bytes()
            .await
            .context(format!("Failed to download raw blob from {}", url))?;
        Ok(bytes.to_vec())
    }
}
