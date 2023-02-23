use async_shutdown::Shutdown;
use once_cell::sync::Lazy;
use tracing::{error, info};

pub static SHUTDOWN_SIGNAL: Lazy<Shutdown> = Lazy::new(|| {
    let shutdown = Shutdown::new();
    // Spawn a task to wait for CTRL+C and trigger a shutdown.
    tokio::spawn({
        let shutdown = shutdown.clone();
        async move {
            if let Err(e) = tokio::signal::ctrl_c().await {
                error!("Failed to wait for CTRL+C: {}", e);
                std::process::exit(1);
            } else {
                info!("\nReceived interrupt signal. Shutting down server...");
                shutdown.shutdown();
            }
        }
    });

    shutdown
});
