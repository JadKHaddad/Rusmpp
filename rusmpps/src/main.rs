use std::time::Duration;

use rusmpps::server::{Server, ServerParameters};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("rusmpps=debug")
        .init();

    let parameters = ServerParameters {
        clients: vec![],
        enquire_link_interval: Duration::from_secs(10),
        response_timeout: Duration::from_secs(3),
        session_timeout: Duration::from_secs(3),
        bind_delay: Duration::from_millis(100),
        response_delay: Duration::from_secs(1),
        socket_addr: "127.0.0.1:2775"
            .parse()
            .expect("Failed to parse socket address"),
    };

    let server = Server::new(parameters);

    tracing::info!("Starting server");

    server.run().await?;

    Ok(())
}
