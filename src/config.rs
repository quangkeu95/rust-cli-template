use config::{Config, File};
use once_cell::sync::Lazy;
use serde::Deserialize;
use tracing::info;

use crate::errors::AppError;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub log_level: String,
}

impl AppConfig {
    pub fn parse() -> Result<AppConfig, AppError> {
        let project_root = rusty_robin::os::get_project_root().map_err(|_| {
            AppError::ConfigurationError("Failed to determine the project root".to_owned())
        })?;
        let configuration_dir = project_root.join("config");

        // Detect the running environment.
        // Default to `local` if unspecified.
        let environment: Environment = get_app_environment()?;
        let environment_filename = format!("{}.toml", environment.as_str());

        info!("Using APP_ENVIRONMENT = {:?}", environment);

        let s = Config::builder()
            // Start off by merging in the "default" configuration file
            .add_source(File::from(configuration_dir.join("base.toml")))
            .add_source(File::from(configuration_dir.join(environment_filename)))
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .prefix_separator("_")
                    .separator("_"),
            )
            .build()
            .map_err(|err| AppError::ConfigurationError(err.to_string()))?;

        s.try_deserialize::<AppConfig>()
            .map_err(|err| AppError::ConfigurationError(err.to_string()))
    }
}

pub fn get_app_environment() -> Result<Environment, AppError> {
    std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .map_err(|err: String| AppError::ConfigurationError(err))
}

#[derive(Debug, Clone)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local` or `production`.",
                other
            )),
        }
    }
}

pub static APP_CONFIG_INSTANCE: Lazy<AppConfig> = Lazy::new(|| match AppConfig::parse() {
    Ok(app_config) => app_config,
    Err(e) => panic!("App configuration error: {}", e),
});
