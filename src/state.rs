use std::sync::Arc;

use crate::errors::AppError;

pub type SharedAppState = Arc<AppState>;

#[derive(Debug, Clone)]
pub struct AppState {}

impl AppState {
    pub async fn new() -> Result<Self, AppError> {
        Ok(AppState {})
    }
}
