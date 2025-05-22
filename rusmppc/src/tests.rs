use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
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
    use rusmpp::{
        pdus::SubmitSm,
        types::{COctetString, OctetString},
        values::{EsmClass, Npi, RegisteredDelivery, ServiceType, Ton},
    };

    init_tracing();

    let (client, mut events) = ConnectionBuilder::new(SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        2775,
    ))
    .system_id(COctetString::from_str("NfDfddEKVI0NCxO").unwrap()) // cspell:disable-line
    .password(COctetString::from_str("rEZYMq5j").unwrap())
    .system_type(COctetString::empty())
    .addr_ton(Ton::Unknown)
    .addr_npi(Npi::Unknown)
    .address_range(COctetString::empty())
    .transmitter()
    .enquire_link_timeout(Duration::from_secs(10))
    .connect()
    .await
    .expect("Failed to connect");

    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            tracing::debug!(?event, "Event",);
        }

        tracing::debug!("Event stream closed");
    });

    client
        .submit_sm(
            SubmitSm::builder()
                .service_type(ServiceType::default())
                .source_addr_ton(Ton::Unknown)
                .source_addr_npi(Npi::Unknown)
                .source_addr(COctetString::from_str("12345").unwrap())
                .destination_addr(COctetString::from_str("491701234567").unwrap())
                .esm_class(EsmClass::default())
                .registered_delivery(RegisteredDelivery::request_all())
                .short_message(OctetString::from_str("Hi, I am a short message.").unwrap())
                .build(),
        )
        .await
        .expect("Failed to submit_sm");

    tokio::time::sleep(std::time::Duration::from_secs(60)).await;
}
