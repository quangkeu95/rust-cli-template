use std::net::SocketAddr;

use axum::{routing::get, Extension, Router, Server};
use tower_http::trace::TraceLayer;
use tracing::info;

use crate::{
    errors::{ApiError, AppError},
    state::SharedAppState,
    utils::SHUTDOWN_SIGNAL,
};

/* -------------------------------------------------------------------------- */
/*                                   MODULES                                  */
/* -------------------------------------------------------------------------- */
mod ping;

pub struct ApiController;

impl ApiController {
    pub fn new_router(app_state: SharedAppState) -> Router {
        Router::new()
            .nest("/api", Router::new().route("/ping", get(ping::ping)))
            .layer(Extension(app_state))
            .layer(TraceLayer::new_for_http())
    }

    pub async fn serve(app: Router, port: u16) -> Result<(), AppError> {
        let addr = SocketAddr::from(([0, 0, 0, 0], port));
        info!(
            "All routes initialized, HTTP server listening on http://{}",
            addr
        );

        Server::bind(&addr)
            .serve(app.into_make_service())
            .with_graceful_shutdown(shutdown_signal())
            .await
            .map_err(|err| ApiError::RouterError(err.to_string()))?;

        Ok(())
    }
}

async fn shutdown_signal() {
    let shutdown_signal = SHUTDOWN_SIGNAL.clone();
    shutdown_signal.wait_shutdown_triggered().await;
    info!("Shutting down HTTP server...");
}
