use crate::cli::InstallArgs;
use anyhow::Result;
use colored::Colorize;
use rusl_app::install_service;
use rusl_app::resolver::graph::ProgressReporter;

pub async fn run(_args: InstallArgs) -> Result<()> {
    let progress = CliInstallProgress::new();
    let result = install_service::install_project(&progress).await?;

    progress.finish_with_message(format!(
        "{} Installed {} schemas.",
        "Success:".green().bold(),
        result.schema_count
    ));
    Ok(())
}

struct CliInstallProgress {
    spinner: indicatif::ProgressBar,
}

impl CliInstallProgress {
    fn new() -> Self {
        Self {
            spinner: crate::ui::spinner("Starting installation..."),
        }
    }

    fn finish_with_message(&self, message: String) {
        self.spinner.finish_with_message(message);
    }
}

impl ProgressReporter for CliInstallProgress {
    fn set_message(&self, message: String) {
        self.spinner.set_message(message);
    }

    fn println(&self, message: String) {
        self.spinner.println(message);
    }
}
