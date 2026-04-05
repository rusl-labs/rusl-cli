use crate::config::{self, credentials::Credentials};
use anyhow::{Context, Result, bail};
use base64::prelude::*;
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginSession {
    pub browser_url: String,
    pub code_verifier: String,
}

#[derive(Serialize)]
struct TokenExchangeRequest {
    code: String,
    code_verifier: String,
}

#[derive(Deserialize)]
struct TokenExchangeResponse {
    access_token: String,
    refresh_token: String,
}

pub fn begin_login(callback_url: &str) -> Result<LoginSession> {
    let config = config::load().context("Failed to load network configuration")?;

    let mut random_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut random_bytes);
    let code_verifier = BASE64_URL_SAFE_NO_PAD.encode(random_bytes);

    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let code_challenge = BASE64_URL_SAFE_NO_PAD.encode(hasher.finalize());

    let encoded_callback = callback_url.replace(":", "%3A").replace("/", "%2F");
    let browser_url = format!(
        "{}/login/cli?callback_url={encoded_callback}&code_challenge={code_challenge}&code_challenge_method=S256",
        config.website_url
    );

    Ok(LoginSession {
        browser_url,
        code_verifier,
    })
}

pub async fn complete_login(code: String, code_verifier: String) -> Result<()> {
    if code.trim().is_empty() {
        bail!("Login failed: no authorization code was received from the browser.");
    }

    let config = config::load().context("Failed to load network configuration")?;
    let auth_endpoint = format!("{}/api/auth/cli/token", config.api_base_url);

    let token_response = Client::new()
        .post(&auth_endpoint)
        .json(&TokenExchangeRequest {
            code,
            code_verifier,
        })
        .send()
        .await
        .context("Failed to exchange the browser code for CLI credentials")?;

    if !token_response.status().is_success() {
        let status = token_response.status();
        let body = token_response.text().await.unwrap_or_default();
        bail!("Login failed ({status}): {body}");
    }

    let payload: TokenExchangeResponse = token_response
        .json()
        .await
        .context("Failed to parse the login token response")?;

    Credentials {
        access_token: payload.access_token,
        refresh_token: payload.refresh_token,
    }
    .save()
    .context("Failed to save credentials")
}

#[cfg(test)]
mod tests {
    use super::begin_login;

    #[test]
    fn begin_login_embeds_callback_and_pkce_parameters() {
        let session = begin_login("http://127.0.0.1:4123/callback").expect("build login session");

        assert!(
            session
                .browser_url
                .contains("callback_url=http%3A%2F%2F127.0.0.1%3A4123%2Fcallback")
        );
        assert!(session.browser_url.contains("code_challenge="));
        assert!(session.browser_url.contains("code_challenge_method=S256"));
        assert!(!session.code_verifier.is_empty());
    }
}
