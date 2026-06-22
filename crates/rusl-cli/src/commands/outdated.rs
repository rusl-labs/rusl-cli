use crate::cli::OutdatedArgs;
use anyhow::Result;
use colored::Colorize;
use rusl_app::outdated_service::{self, OutdatedOutput};

pub async fn run(_args: OutdatedArgs) -> Result<()> {
    let pb = crate::ui::spinner("Checking registry for updates...");
    let output = outdated_service::load_outdated_dependencies().await?;
    pb.finish_and_clear();

    match output {
        OutdatedOutput::MissingLockfile => {
            println!(
                "{}",
                "No schemas installed. `rusl.lock` not found.".yellow()
            );
        }
        OutdatedOutput::EmptyLockfile => {
            println!("{}", "No dependencies found in rusl.lock.".yellow());
        }
        OutdatedOutput::Items(items) if items.is_empty() => {
            println!("{}", "Everything is up to date!".green().bold());
        }
        OutdatedOutput::Items(items) => {
            println!("\n{}", "Outdated Schemas".bold());
            for (index, item) in items.iter().enumerate() {
                let prefix = if index + 1 == items.len() {
                    "└── "
                } else {
                    "├── "
                };
                println!(
                    "{}{}{} -> {}",
                    prefix.dimmed(),
                    item.display_name.bold(),
                    format!("@v{}", item.current_version).yellow(),
                    format!("@v{}", item.latest_version).green().bold()
                );
            }
        }
    }

    Ok(())
}
