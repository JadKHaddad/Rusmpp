//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppc --example reconnect_dev
//! ```
//!

// If disconnected and tried to reconnect, how do we bind?

use std::{str::FromStr, time::Duration};

use futures::StreamExt;
use rusmpp::{
    CommandId,
    pdus::{BindTransceiver, DeliverSmResp, SubmitSm},
    types::{COctetString, OctetString},
    values::{EsmClass, Npi, RegisteredDelivery, ServiceType, Ton},
};
use rusmppc::{ClientBuilder, Event};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Rusmppc produces a lot of logs while managing the SMPP connection in the background.
    // You can filter them out by setting the `rusmppc` target to `off`.
    tracing_subscriber::fmt()
        .with_env_filter("submit_sm=info,rusmpp=off,rusmppc=debug")
        .init();

    let (client, mut events) = ClientBuilder::new()
        // Every 5 seconds send an enquire link command to the server.
        .enquire_link_interval(Duration::from_secs(5))
        // If the server does not respond within 2 seconds, consider it a timeout.
        .response_timeout(Duration::from_secs(2))
        .reconnect(|| {
            Box::pin(async {
                let stream = TcpStream::connect("127.0.0.1:2775").await?;

                Ok(stream)
            })
        })
        .await?;

    client
        .bind_transceiver(
            BindTransceiver::builder()
                .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
                .password(COctetString::from_str("rEZYMq5j")?)
                .system_type(COctetString::empty())
                .addr_ton(Ton::Unknown)
                .addr_npi(Npi::Unknown)
                .address_range(COctetString::empty())
                .build(),
        )
        .await?;

    let client_clone = client.clone();

    let events = tokio::spawn(async move {
        // Listen for events like incoming commands and background errors.
        while let Some(event) = events.next().await {
            tracing::info!(?event, "Event");

            if let Event::Incoming(command) = event {
                if command.id() == CommandId::DeliverSm {
                    tracing::info!("Received DeliverSm");

                    let _ = client_clone
                        .deliver_sm_resp(command.sequence_number(), DeliverSmResp::default())
                        .await;
                }
            }
        }

        tracing::info!("Connection closed");
    });

    let submit = SubmitSm::builder()
        .service_type(ServiceType::default())
        .source_addr_ton(Ton::Unknown)
        .source_addr_npi(Npi::Unknown)
        .source_addr(COctetString::from_str("12345")?)
        .destination_addr(COctetString::from_str("491701234567")?)
        .esm_class(EsmClass::default())
        .registered_delivery(RegisteredDelivery::request_all())
        .short_message(OctetString::from_str("Hi, I am a short message.")?)
        .build();

    let response = client.submit_sm(submit.clone()).await?;

    tracing::info!(?response, "Got SubmitSmResp");

    events.await?;

    Ok(())
}
