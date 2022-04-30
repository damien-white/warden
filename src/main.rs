use std::net::TcpListener;

use tracing::debug;

use warden::{app, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    telemetry::init_logging()?;

    let listener = TcpListener::bind("0.0.0.0:4567")?;

    let address = listener.local_addr()?;
    debug!("Starting service. Listening at: {address}");

    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app::router().into_make_service())
        .await
        .expect("server failed to start");

    // TODO: integrate shutdown hooks such as SIGTERM, etc.
    debug!("Shutdown signal received. Shutting down service.");
    Ok(())
}
