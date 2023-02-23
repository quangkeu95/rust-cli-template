use clap::crate_name;
use derive_builder::Builder;
use std::str::FromStr;
use tracing::log::LevelFilter;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::{config::APP_CONFIG_INSTANCE, errors::AppError};

#[derive(Debug, Builder)]
pub struct TracingOptions {
    #[builder(default = "LevelFilter::Info")]
    pub crate_level: LevelFilter,
    #[builder(default = "LevelFilter::Off")]
    pub tower_http_level: LevelFilter,
}

pub fn init_tracing() -> Result<(), AppError> {
    let config = &APP_CONFIG_INSTANCE;
    let crate_log_level =
        LevelFilter::from_str(config.log_level.as_str()).unwrap_or_else(|_| LevelFilter::Info);
    let http_log_level =
        LevelFilter::from_str(config.http_server.log_level.as_str()).unwrap_or(LevelFilter::Info);

    let tracing_options = TracingOptionsBuilder::default()
        .crate_level(crate_log_level)
        .tower_http_level(http_log_level)
        .build()
        .map_err(|err| AppError::ConfigurationError(err.to_string()))?;

    let crate_name = crate_name!().to_owned().replace("-", "_");
    let crate_level = tracing_options.crate_level.as_str().to_lowercase();
    let tower_http_level = tracing_options.tower_http_level.as_str().to_lowercase();

    let env_filter_level = format!(
        "{}={},tower_http={}",
        crate_name, crate_level, tower_http_level
    );

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| env_filter_level.into());
    let formatting_layer = BunyanFormattingLayer::new(
        crate_name,
        // Output the formatted spans to stdout.
        std::io::stdout,
    );

    tracing_subscriber::registry()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .init();

    Ok(())
}
