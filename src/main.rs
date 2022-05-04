use tokio::io;
use tracing::{debug, error};

use warden::{service::app, telemetry};

#[tokio::main]
async fn main() -> io::Result<()> {
    telemetry::init_logging()?;

    let address = "0.0.0.0:4567"
        .parse()
        .expect("socket address must be valid");

    debug!(%address, "starting service");
    if let Err(err) = axum::Server::bind(&address)
        .serve(app().into_make_service())
        .await
    {
        error!(reason = ?err, "failed to start service")
    }

    // TODO: implement graceful shutdown via signal handler
    debug!("Shutdown signal received. Shutting down service.");
    Ok(())
}
