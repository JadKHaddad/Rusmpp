//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppc --example reconnect
//! ```
//!

use std::{str::FromStr, time::Duration};

use futures::StreamExt;
use rusmpp::{
    CommandId,
    pdus::{BindTransceiver, DeliverSmResp, SubmitSm},
    types::{COctetString, OctetString},
    values::{EsmClass, Npi, RegisteredDelivery, ServiceType, Ton},
};
use rusmppc::{ConnectionBuilder, Event, reconnect::ReconnectingEvent};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("reconnect=info,rusmpp=off,rusmppc=trace,tokio_util=trace")
        .init();

    let (client, mut events) = ConnectionBuilder::new()
        .enquire_link_interval(Duration::from_secs(5))
        .response_timeout(Duration::from_secs(2))
        .reconnect_with(|| TcpStream::connect("127.0.0.1:2775"))
        .delay(Duration::from_secs(5))
        .max_retries(5)
        .connect()
        .await?;

    let bind = BindTransceiver::builder()
        .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
        .password(COctetString::from_str("rEZYMq5j")?)
        .system_type(COctetString::empty())
        .addr_ton(Ton::Unknown)
        .addr_npi(Npi::Unknown)
        .address_range(COctetString::empty())
        .build();

    client.bind_transceiver(bind.clone()).await?;

    let client_clone = client.clone();

    let events = tokio::spawn(async move {
        while let Some(event) = events.next().await {
            tracing::info!(?event, "Event");

            match event {
                ReconnectingEvent::Connection(Event::Incoming(command)) => {
                    if command.id() == CommandId::DeliverSm {
                        tracing::info!("Received DeliverSm");

                        let _ = client_clone
                            .deliver_sm_resp(command.sequence_number(), DeliverSmResp::default())
                            .await;
                    }
                }
                ReconnectingEvent::Reconnected => {
                    tracing::info!("Reconnected");

                    if (client_clone.bind_transceiver(bind.clone()).await).is_ok() {
                        tracing::info!("Rebound after reconnect");
                    }
                }
                _ => {}
            }
        }

        tracing::info!("Connection closed");
    });

    for _ in 0..10 {
        // Client might not be bound after a reconnect event.

        // If the connection is in a reconnecting state, client requests will wait for the connection to be established.
        // Waiting clients will queue their requests as soon as the connection is established again.
        // These requests have a higher priority than the bind request we send on reconnect. (See events). This means that these requests will be sent in a unbound state.
        if let Err(err) = client
            .submit_sm(
                SubmitSm::builder()
                    .service_type(ServiceType::default())
                    .source_addr_ton(Ton::Unknown)
                    .source_addr_npi(Npi::Unknown)
                    .source_addr(COctetString::from_str("12345")?)
                    .destination_addr(COctetString::from_str("491701234567")?)
                    .esm_class(EsmClass::default())
                    .registered_delivery(RegisteredDelivery::request_all())
                    .short_message(OctetString::from_str("Hi, I am a short message.")?)
                    .build(),
            )
            .await
        {
            tracing::error!(?err, "Failed to send SubmitSm");
        }

        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    client.unbind().await?;

    client.close().await?;

    client.closed().await;

    events.await?;

    Ok(())
}
