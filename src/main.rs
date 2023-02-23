use anyhow;
use dotenvy::dotenv;
use {{crate_name}}::{cli::Cli, telemetry};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // initialize tracing
    telemetry::init_tracing()?;

    Cli::parse().await?;

    Ok(())
}
