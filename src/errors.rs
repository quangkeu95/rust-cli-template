use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("configuration error: {0}")]
    ConfigurationError(String),
    #[error(transparent)]
    ApiError(#[from] ApiError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("router error: {0}")]
    RouterError(String),
}
