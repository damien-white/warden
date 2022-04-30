use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:4567")?;

    let address = listener.local_addr()?;
    println!("Starting service. Listening at: {address}");

    let app = warden::app();
    axum::Server::from_tcp(listener)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .expect("server failed to start");

    Ok(())
}
