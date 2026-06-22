use crate::cli::LogoutArgs;
use anyhow::Result;
use colored::Colorize;
use rusl_app::logout_service;

pub async fn run(_args: LogoutArgs) -> Result<()> {
    let pb = crate::ui::spinner("Signing out...");

    logout_service::logout()?;

    pb.finish_with_message(format!("{} Logged out.", "Success:".green().bold()));

    Ok(())
}
