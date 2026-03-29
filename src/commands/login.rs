use crate::cli::LoginArgs;
use crate::config::{self, credentials::Credentials};
use anyhow::{Context, Result};
use base64::prelude::*;
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::net::TcpListener;
use tracing::{info, warn};

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

pub async fn run(_args: LoginArgs) -> Result<()> {
    info!("Initializing OAuth2 PKCE Authentication Framework natively...");

    let config = config::load().context("Failed to evaluate network configurations")?;

    // 1. Establish the Localhost Callback Loop intrinsically to port 0
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("Failed to securely bind onto local networking loopback port.")?;

    let local_port = listener.local_addr()?.port();

    // Strict schema rules!
    let callback_url = format!("http://127.0.0.1:{}/callback", local_port);

    // 2. Generate PKCE Mathematical proofs
    let mut random_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut random_bytes);
    let code_verifier = BASE64_URL_SAFE_NO_PAD.encode(random_bytes);

    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let code_challenge = BASE64_URL_SAFE_NO_PAD.encode(hasher.finalize());

    // 3. Spawning OS Browser implicitly
    // We strictly URL-encode the nested callback_url using brutal replacement since we explicitly know its structural bounds securely.
    let encoded_callback = callback_url.replace(":", "%3A").replace("/", "%2F");

    let browser_url = format!(
        "{}/login/cli?callback_url={}&code_challenge={}&code_challenge_method=S256",
        config.website_url, encoded_callback, code_challenge
    );

    info!("Executing physical OS Browser handoff to strictly authenticate...");
    info!(
        "Waiting natively for browser interaction redirect at port {}...",
        local_port
    );

    if webbrowser::open(&browser_url).is_err() {
        warn!(
            "Failed to auto-open browser physically. Please manually navigate to: {}",
            browser_url
        );
    }

    // 4. Await Browser Redirect Loop HTTP connection synchronously via purely raw TCP stream parsing
    let (stream, _) = listener
        .accept()
        .await
        .context("Mathematical browser listener dropped natively")?;

    // Convert TCP stream dynamically without heavy http servers safely!
    stream.readable().await?;
    let mut buffer = [0; 4096];
    let mut code = String::new();

    match stream.try_read(&mut buffer) {
        Ok(0) => warn!("The local socket mathematically closed before yielding bytes"),
        Ok(n) => {
            let request_string = String::from_utf8_lossy(&buffer[..n]);

            // HTTP Request line is natively: GET /callback?code=xxx&state=yyy HTTP/1.1
            if let Some(first_line) = request_string.lines().next() {
                if let Some(query_start) = first_line.find("?code=") {
                    // Extract exact substring natively
                    let block = &first_line[query_start + 6..];
                    let end_idx = block
                        .find('&')
                        .or_else(|| block.find(' '))
                        .unwrap_or(block.len());
                    code = block[..end_idx].to_string();
                }
            }
        }
        Err(e) => warn!("Socket violently dropped physically: {}", e),
    }

    // Instantly flush a strictly defined HTML payload success screen mathematically straight to the browser socket
    let response = "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: text/html\r\n\r\n<html><body><h1 style='font-family: sans-serif; text-align: center; margin-top: 20%; color: #333'>Rusl PKCE Handshake Complete!</h1><p style='text-align: center; color: #666; font-family: sans-serif'>You can safely close this browser window and return to your terminal.</p><script>setTimeout(()=>window.close(), 3000)</script></body></html>";
    let _ = stream.try_write(response.as_bytes());

    // Systematically destroy the token reader loops dropping the native 127.0.0.1 boundary back to the OS!
    drop(stream);
    drop(listener);

    if code.is_empty() {
        anyhow::bail!(
            "Security violation: Received a local redirect that mathematically failed to encode an `authorization_code`!"
        );
    }

    info!("Natively intercepted Authorization Code mathematically from Browser Loop!");
    info!("Executing strict PKCE Code Verification Handshake with backend API...");

    // 5. Submit PKCE strict token request directly to the API_BASE_URL (not Website)
    let client = Client::new();
    let auth_endpoint = format!("{}/api/auth/cli/token", config.api_base_url);

    let token_res = client
        .post(&auth_endpoint)
        .json(&TokenExchangeRequest {
            code,
            code_verifier,
        })
        .send()
        .await
        .context("Failed mathematically to dial internal API endpoint natively.")?;

    if !token_res.status().is_success() {
        let err_str = token_res.text().await.unwrap_or_default();
        anyhow::bail!(
            "Cryptographic PKCE exchange formally exploded natively! (Validation failed code 400/401). Error: {}",
            err_str
        );
    }

    let payload: TokenExchangeResponse = token_res
        .json()
        .await
        .context("Failed to deserialize native Token JSON structurally")?;

    info!("Session rigidly verified cryptographically!");

    // 6. Dump structural tokens cleanly inside `~/.config/rusl/credentials.toml` rigidly structured
    let creds = Credentials {
        access_token: payload.access_token,
        refresh_token: payload.refresh_token,
    };

    creds
        .save()
        .context("The file permission boundary natively rejected encrypted credential storage")?;

    info!("Successfully mathematically authenticated Rusl registry!");

    Ok(())
}
