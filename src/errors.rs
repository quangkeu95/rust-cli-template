use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("configuration error: {0}")]
    ConfigurationError(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
