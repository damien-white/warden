use tokio::io;
use tracing::{debug, error};

use warden::{service::app, settings::Settings, telemetry};

#[tokio::main]
async fn main() -> io::Result<()> {
    telemetry::init_logging()?;

    let settings = Settings::new().expect("failed to load settings");

    let address = &settings.server_address();

    debug!(%address, "starting service");
    if let Err(err) = axum::Server::bind(address)
        .serve(app().into_make_service())
        .await
    {
        error!(reason = ?err, "failed to start service")
    }

    // TODO: implement graceful shutdown via signal handler
    debug!("Shutdown signal received. Shutting down service.");
    Ok(())
}
