use clap::Parser;
use rusl_app::Cli;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("rusl=info".parse()?))
        .init();

    let cli = Cli::parse();
    rusl_app::run(cli.command).await
}
