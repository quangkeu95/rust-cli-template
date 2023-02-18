use anyhow;
use dotenvy::dotenv;
// use {{crate_name}}::{cli::Cli, config::APP_CONFIG_INSTANCE, telemetry};
use {{crate_name}}::{cli::Cli, config::APP_CONFIG_INSTANCE, telemetry};
use std::str::FromStr;
use tracing::log::LevelFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let config = &APP_CONFIG_INSTANCE;
    let crate_log_level =
        LevelFilter::from_str(config.log_level.as_str()).unwrap_or_else(|_| LevelFilter::Info);

    let tracing_options = telemetry::TracingOptionsBuilder::default()
        .crate_level(crate_log_level)
        .build()?;

    // initialize tracing
    telemetry::init_tracing("besui".into(), tracing_options);

    Cli::parse().await?;

    Ok(())
}
