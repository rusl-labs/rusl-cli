use crate::cli::LoginArgs;
use crate::config::{self, credentials::Credentials};
use anyhow::{Context, Result};
use base64::prelude::*;
use colored::Colorize;
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::net::TcpListener;

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
    let pb = crate::ui::spinner("Starting login process...");

    let config = config::load().context("Failed to evaluate network configurations")?;

    // 1. Establish the Localhost Callback Loop intrinsically to port 0
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("Failed to securely bind onto local networking loopback port.")?;

    let local_port = listener.local_addr()?.port();
    let callback_url = format!("http://127.0.0.1:{}/callback", local_port);

    // 2. Generate PKCE Mathematical proofs
    let mut random_bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut random_bytes);
    let code_verifier = BASE64_URL_SAFE_NO_PAD.encode(random_bytes);

    let mut hasher = Sha256::new();
    hasher.update(code_verifier.as_bytes());
    let code_challenge = BASE64_URL_SAFE_NO_PAD.encode(hasher.finalize());

    // 3. Spawning OS Browser implicitly
    let encoded_callback = callback_url.replace(":", "%3A").replace("/", "%2F");
    let browser_url = format!(
        "{}/login/cli?callback_url={}&code_challenge={}&code_challenge_method=S256",
        config.website_url, encoded_callback, code_challenge
    );

    pb.set_message("Opening your browser to authenticate...");

    if webbrowser::open(&browser_url).is_err() {
        pb.println(format!(
            "{} Failed to auto-open browser directly. Please manually navigate to: {}",
            "Warning:".yellow().bold(),
            browser_url
        ));
    }

    pb.set_message(format!(
        "Waiting for browser redirect on port {}...",
        local_port
    ));

    // 4. Await Browser Redirect Loop HTTP connection synchronously
    let (stream, _) = listener
        .accept()
        .await
        .context("Mathematical browser listener dropped natively")?;

    stream.readable().await?;
    let mut buffer = [0; 4096];
    let mut code = String::new();

    match stream.try_read(&mut buffer) {
        Ok(0) => pb.println(format!(
            "{} Timed out waiting for callback.",
            "Warning:".yellow().bold()
        )),
        Ok(n) => {
            let request_string = String::from_utf8_lossy(&buffer[..n]);

            if let Some(first_line) = request_string.lines().next() {
                if let Some(query_start) = first_line.find("?code=") {
                    let block = &first_line[query_start + 6..];
                    let end_idx = block
                        .find('&')
                        .or_else(|| block.find(' '))
                        .unwrap_or(block.len());
                    code = block[..end_idx].to_string();
                }
            }
        }
        Err(e) => pb.println(format!(
            "{} Connection error: {}",
            "Warning:".yellow().bold(),
            e
        )),
    }

    let response = "HTTP/1.1 200 OK\r\nConnection: close\r\nContent-Type: text/html\r\n\r\n<html><body><h1 style='font-family: sans-serif; text-align: center; margin-top: 20%; color: #333'>Rusl PKCE Handshake Complete!</h1><p style='text-align: center; color: #666; font-family: sans-serif'>You can safely close this browser window and return to your terminal.</p><script>setTimeout(()=>window.close(), 3000)</script></body></html>";
    let _ = stream.try_write(response.as_bytes());

    drop(stream);
    drop(listener);

    if code.is_empty() {
        anyhow::bail!("Login failed: No authorization code received from browser.");
    }

    pb.set_message("Verifying credentials...");

    // 5. Submit PKCE strict token request directly to the API_BASE_URL
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
        let status = token_res.status();
        let err_str = token_res.text().await.unwrap_or_default();
        pb.finish_and_clear();
        anyhow::bail!("Login completely failed ({}): {}", status, err_str);
    }

    let payload: TokenExchangeResponse = token_res
        .json()
        .await
        .context("Failed to deserialize native Token JSON structurally")?;

    pb.set_message("Saving token...");

    // 6. Dump structural tokens cleanly inside `~/.config/rusl/credentials.toml`
    let creds = Credentials {
        access_token: payload.access_token,
        refresh_token: payload.refresh_token,
    };

    creds.save().context("Failed to save credentials")?;

    pb.finish_with_message(format!(
        "{} Logged in successfully!",
        "Success:".green().bold()
    ));

    Ok(())
}
