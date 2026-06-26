use crate::cli::RemoveArgs;
use anyhow::Result;
use colored::Colorize;
use rusl_app::dependency_service::{self, RemoveDependencyRequest, RemoveDependencyResult};
use rusl_app::resolver::graph::ProgressReporter;

pub async fn run(args: RemoveArgs) -> Result<()> {
    let progress = CliDependencyProgress::new();
    let request = RemoveDependencyRequest {
        identifier: args.identifier,
    };

    match dependency_service::remove_dependency(request, &progress).await? {
        RemoveDependencyResult::Removed {
            slug,
            table_key,
            install,
        } => progress.finish_with_message(format!(
            "{} Removed {} from [{}] and installed {} schemas.",
            "Success:".green().bold(),
            slug,
            table_key,
            install.schema_count
        )),
        RemoveDependencyResult::NotPresent { slug, table_key } => {
            progress.finish_with_message(format!(
                "{} {} is not in [{}]. Nothing to remove.",
                "Status:".cyan().bold(),
                slug,
                table_key
            ))
        }
    }

    Ok(())
}

struct CliDependencyProgress {
    spinner: indicatif::ProgressBar,
}

impl CliDependencyProgress {
    fn new() -> Self {
        Self {
            spinner: crate::ui::spinner("Reading rusl.bundle.toml..."),
        }
    }

    fn finish_with_message(&self, message: String) {
        self.spinner.finish_with_message(message);
    }
}

impl ProgressReporter for CliDependencyProgress {
    fn set_message(&self, message: String) {
        self.spinner.set_message(message);
    }

    fn println(&self, message: String) {
        self.spinner.println(message);
    }
}
