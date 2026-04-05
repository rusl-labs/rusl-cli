use crate::cli::LoginArgs;
use anyhow::{Context, Result};
use colored::Colorize;
use rusl_app::login_service;
use tokio::net::TcpListener;

pub async fn run(_args: LoginArgs) -> Result<()> {
    let pb = crate::ui::spinner("Starting login...");
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .context("Failed to bind a local callback port")?;

    let local_port = listener.local_addr()?.port();
    let callback_url = format!("http://127.0.0.1:{}/callback", local_port);
    let session = login_service::begin_login(&callback_url)?;

    pb.set_message("Opening your browser to authenticate...");

    if webbrowser::open(&session.browser_url).is_err() {
        pb.println(format!(
            "{} Failed to open your browser automatically. Open this URL instead: {}",
            "Warning:".yellow().bold(),
            session.browser_url
        ));
    }

    pb.set_message(format!(
        "Waiting for browser redirect on port {}...",
        local_port
    ));

    let (stream, _) = listener
        .accept()
        .await
        .context("Browser callback listener closed unexpectedly")?;

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

            if let Some(first_line) = request_string.lines().next()
                && let Some(query_start) = first_line.find("?code=")
            {
                let block = &first_line[query_start + 6..];
                let end_idx = block
                    .find('&')
                    .or_else(|| block.find(' '))
                    .unwrap_or(block.len());
                code = block[..end_idx].to_string();
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

    pb.set_message("Verifying credentials...");
    pb.set_message("Saving token...");
    login_service::complete_login(code, session.code_verifier).await?;

    pb.finish_with_message(format!(
        "{} Logged in successfully!",
        "Success:".green().bold()
    ));

    Ok(())
}
