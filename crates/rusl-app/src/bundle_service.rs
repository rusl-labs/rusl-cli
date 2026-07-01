use crate::{config, config::credentials::Credentials, registry::client::RegistryClient};
use anyhow::{Context, Result, bail};
use rusl_api_client::{models, rusl_user_agent_with_context};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleVisibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateBundleRequest {
    pub account_slug: String,
    pub bundle_slug: String,
    pub description: Option<String>,
    pub visibility: BundleVisibility,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateBundleVersionRequest {
    pub account_slug: String,
    pub bundle_slug: String,
    pub version: String,
    pub manifest: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishBundleVersionRequest {
    pub account_slug: String,
    pub bundle_slug: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleOutput {
    pub id: String,
    pub guid: Option<String>,
    pub account_slug: String,
    pub bundle_slug: String,
    pub identifier: String,
    pub description: Option<String>,
    pub status: String,
    pub visibility: Option<String>,
    pub inserted_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BundleVersionOutput {
    pub id: String,
    pub guid: String,
    pub version: String,
    pub status: String,
    pub manifest: String,
    pub description: Option<String>,
    pub published_at: Option<String>,
    pub inserted_at: Option<String>,
    pub updated_at: Option<String>,
}

pub async fn create_bundle_with_user_agent_context(
    request: CreateBundleRequest,
    context: &str,
) -> Result<BundleOutput> {
    require_login()?;
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let account_slug = request.account_slug.clone();
    let response = client
        .create_bundle(&account_slug, to_api_create_bundle_request(request))
        .await?;
    let Some(bundle) = response.data.flatten() else {
        bail!("Rusl API did not return created bundle data");
    };

    Ok(map_bundle(*bundle))
}

pub async fn create_bundle_version_with_user_agent_context(
    request: CreateBundleVersionRequest,
    context: &str,
) -> Result<BundleVersionOutput> {
    require_login()?;
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let account_slug = request.account_slug.clone();
    let bundle_slug = request.bundle_slug.clone();
    let api_request = to_api_create_bundle_version_request(request);
    let response = client
        .create_bundle_version(&account_slug, &bundle_slug, api_request)
        .await?;
    let Some(version) = response.data.flatten() else {
        bail!("Rusl API did not return created bundle version data");
    };

    Ok(map_bundle_version(*version))
}

pub async fn publish_bundle_version_with_user_agent_context(
    request: PublishBundleVersionRequest,
    context: &str,
) -> Result<BundleVersionOutput> {
    require_login()?;
    let config = config::load().context("Failed to load network configurations")?;
    let user_agent = rusl_user_agent_with_context(env!("CARGO_PKG_VERSION"), context);
    let client = RegistryClient::with_user_agent_and_rusl_agent(config, user_agent, context);
    let response = client
        .publish_bundle_version(
            &request.account_slug,
            &request.bundle_slug,
            &request.version,
        )
        .await?;
    let Some(version) = response.data.flatten() else {
        bail!("Rusl API did not return published bundle version data");
    };

    Ok(map_bundle_version(*version))
}

fn require_login() -> Result<()> {
    let Some(credentials) = Credentials::load() else {
        bail!("This tool requires user authorization. Run `rusl login` first.");
    };

    if credentials.access_token.trim().is_empty() && credentials.refresh_token.trim().is_empty() {
        bail!("This tool requires user authorization. Run `rusl login` first.");
    }

    Ok(())
}

fn to_api_create_bundle_request(
    request: CreateBundleRequest,
) -> rusl_api_client::CreateBundleRequest {
    rusl_api_client::CreateBundleRequest {
        slug: request.bundle_slug,
        description: request.description,
        visibility: Some(bundle_visibility_label(request.visibility).to_string()),
    }
}

fn to_api_create_bundle_version_request(
    request: CreateBundleVersionRequest,
) -> rusl_api_client::CreateBundleVersionRequest {
    rusl_api_client::CreateBundleVersionRequest {
        version: request.version,
        manifest: request.manifest,
        description: request.description,
    }
}

fn map_bundle(bundle: models::Bundle1) -> BundleOutput {
    BundleOutput {
        id: bundle.id,
        guid: bundle.guid,
        account_slug: bundle.account_slug,
        bundle_slug: bundle.slug,
        identifier: bundle.identifier,
        description: bundle.description.flatten(),
        status: bundle_status_label(bundle.status).to_string(),
        visibility: bundle
            .visibility
            .map(bundle_visibility_api_label)
            .map(str::to_string),
        inserted_at: bundle.inserted_at,
        updated_at: bundle.updated_at,
    }
}

fn map_bundle_version(version: models::BundleVersion1) -> BundleVersionOutput {
    BundleVersionOutput {
        id: version.id,
        guid: version.guid,
        version: version.version,
        status: bundle_version_status_label(version.status).to_string(),
        manifest: version.manifest,
        description: version.description.flatten(),
        published_at: version.published_at.flatten(),
        inserted_at: version.inserted_at,
        updated_at: version.updated_at,
    }
}

fn bundle_visibility_label(visibility: BundleVisibility) -> &'static str {
    match visibility {
        BundleVisibility::Public => "PUBLIC",
        BundleVisibility::Private => "PRIVATE",
    }
}

fn bundle_visibility_api_label(visibility: models::bundle_1::Visibility) -> &'static str {
    match visibility {
        models::bundle_1::Visibility::Public => "PUBLIC",
        models::bundle_1::Visibility::Private => "PRIVATE",
    }
}

fn bundle_status_label(status: models::bundle_1::Status) -> &'static str {
    match status {
        models::bundle_1::Status::Active => "ACTIVE",
        models::bundle_1::Status::Archived => "ARCHIVED",
    }
}

fn bundle_version_status_label(status: models::bundle_version_1::Status) -> &'static str {
    match status {
        models::bundle_version_1::Status::Draft => "DRAFT",
        models::bundle_version_1::Status::Active => "ACTIVE",
        models::bundle_version_1::Status::Deprecated => "DEPRECATED",
        models::bundle_version_1::Status::Yanked => "YANKED",
    }
}
