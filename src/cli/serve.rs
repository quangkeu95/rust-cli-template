use std::sync::Arc;

use clap::{ArgMatches, Command};

use crate::{
    config::APP_CONFIG_INSTANCE,
    errors::AppError,
    router::{self, ApiController},
    state::AppState,
};

pub const COMMAND_NAME: &str = "serve";

pub fn command() -> impl Into<Command> {
    Command::new(COMMAND_NAME).about("Serve HTTP server")
}

pub async fn execute(_args: &ArgMatches) -> Result<(), AppError> {
    let config = &APP_CONFIG_INSTANCE;
    let app_state = AppState::new().await?;
    let shared_app_state = Arc::new(app_state);
    let router = ApiController::new_router(shared_app_state);
    let http_port = config.http_server.port;
    router::ApiController::serve(router, http_port).await
}
