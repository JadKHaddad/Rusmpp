use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use crate::ConnectionBuilder;

pub fn init_tracing() {
    _ = tracing_subscriber::fmt()
        .with_env_filter("rusmppc=trace")
        .try_init();
}

// cargo test --package rusmppc --lib -- tests::bind --exact --show-output --ignored
#[tokio::test]
#[ignore = "Integration test"]
async fn bind() {
    use futures::StreamExt;

    init_tracing();

    let (client, mut events) = ConnectionBuilder::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        2775,
    ))
    .transmitter()
    .enquire_link_timeout(Duration::from_secs(5))
    .connect()
    .await
    .expect("Failed to connect");

    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            tracing::debug!(?event, "Event",);
        }

        tracing::debug!("Event stream closed");
    });

    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
}
