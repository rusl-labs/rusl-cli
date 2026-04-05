use crate::cli::GenerateArgs;
use anyhow::Result;
use colored::Colorize;
use rusl_app::generate_service::{GenerateOutput, GenerateRequestArgs, run_generate};

pub async fn run(args: GenerateArgs) -> Result<()> {
    let request = GenerateRequestArgs {
        name: args.name,
        list: args.list,
        print_request: args.print_request,
    };

    match run_generate(request).await? {
        GenerateOutput::Listed(generators) => {
            if generators.is_empty() {
                println!("{}", "No generators configured.".yellow());
                return Ok(());
            }

            println!("{}", "Generators:".bold());
            for generator in generators {
                let default_marker = if generator.is_default { " *" } else { "" };
                println!(
                    "  {}{:<4} {:<10} {:<36} -> {}",
                    generator.name.bold(),
                    default_marker.yellow(),
                    generator.tier_label.dimmed(),
                    generator.command,
                    generator.output_dir.cyan()
                );
            }
            println!();
            println!("{}", "* = default".dimmed());
        }
        GenerateOutput::PrintedRequest(json) => {
            println!("{}", json);
        }
        GenerateOutput::Generated(result) => {
            let progress =
                crate::ui::spinner(&format!("Generating with {}...", result.generator_name));
            progress.finish_with_message(format!(
                "{} Generated {} files -> {}",
                "Success:".green().bold(),
                result.file_count,
                result.output_dir
            ));
        }
    }

    Ok(())
}
