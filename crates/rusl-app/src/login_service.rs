use crate::config::{self, credentials::Credentials};
use anyhow::{Context, Result, bail};
use base64::prelude::*;
use rand::Rng;
use rusl_api_client::RuslApiClient;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginSession {
    pub browser_url: String,
    pub logo_url: String,
    pub code_verifier: String,
}

impl LoginSession {
    pub fn callback_page_html(&self, success: bool) -> String {
        let title = if success {
            "Login successful"
        } else {
            "Login failed"
        };
        let message = if success {
            "You can safely close this browser window and return to your terminal."
        } else {
            "We could not read the login callback. Please return to your terminal and try again."
        };

        format!(
            "<html><body style='margin:0;background:#0f172a;color:#e2e8f0;font-family:-apple-system,BlinkMacSystemFont,\"Segoe UI\",sans-serif'><div style='min-height:100vh;display:flex;align-items:center;justify-content:center;padding:24px'><div style='max-width:480px;width:100%;text-align:center;background:#111827;border:1px solid rgba(148,163,184,0.2);border-radius:20px;padding:32px 24px;box-shadow:0 20px 45px rgba(15,23,42,0.35)'><img src='{logo_url}' alt='Rusl' style='width:200px;height:200px;max-width:200px;max-height:200px;object-fit:contain;margin:0 auto 16px;display:block' /><h1 style='font-size:28px;line-height:1.2;margin:0 0 12px;color:#f8fafc'>{title}</h1><p style='margin:0;color:#94a3b8;font-size:16px;line-height:1.5'>{message}</p></div></div><script>setTimeout(()=>window.close(), 3000)</script></body></html>",
            logo_url = self.logo_url,
            title = title,
            message = message,
        )
    }
}

pub fn begin_login(callback_url: &str) -> Result<LoginSession> {
    let config = config::load().context("Failed to load network configuration")?;
    let website_url = config.website_url.trim_end_matches('/').to_string();

    let mut random_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut random_bytes);
    let code_verifier = BASE64_URL_SAFE_NO_PAD.encode(random_bytes);

    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let code_challenge = BASE64_URL_SAFE_NO_PAD.encode(hasher.finalize());

    let encoded_callback = callback_url.replace(":", "%3A").replace("/", "%2F");
    let browser_url = format!(
        "{website_url}/login/cli?callback_url={encoded_callback}&code_challenge={code_challenge}&code_challenge_method=S256"
    );

    Ok(LoginSession {
        browser_url,
        logo_url: format!("{website_url}/logo.png"),
        code_verifier,
    })
}

pub async fn complete_login(code: String, code_verifier: String) -> Result<()> {
    if code.trim().is_empty() {
        bail!("Login failed: no authorization code was received from the browser.");
    }

    let config = config::load().context("Failed to load network configuration")?;
    let client = RuslApiClient::new(config.api_base_url);
    let payload = client
        .exchange_cli_token(code, code_verifier)
        .await
        .context("Failed to exchange the browser code for CLI credentials")?;
    let access_token = match payload
        .access_token
        .filter(|token| !token.trim().is_empty())
    {
        Some(access_token) => access_token,
        None => client
            .exchange_refresh_token(&payload.refresh_token)
            .await
            .context("Failed to exchange the refresh token for an access token")?,
    };

    Credentials {
        access_token,
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
        assert!(session.logo_url.ends_with("/logo.png"));
        assert!(!session.code_verifier.is_empty());
    }
}
