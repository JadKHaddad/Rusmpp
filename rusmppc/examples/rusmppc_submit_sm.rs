//! You can run this example using [SMPP SMSC Simulator](https://github.com/melroselabs/smpp-smsc-simulator).
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppc --example rusmppc_submit_sm
//! ```
//!

use std::{str::FromStr, time::Duration};

use futures::StreamExt;
use rusmpp::{
    CommandId,
    pdus::{DeliverSmResp, SubmitSm},
    types::{COctetString, OctetString},
    values::{EsmClass, Npi, RegisteredDelivery, ServiceType, Ton},
};
use rusmppc::{ConnectionBuilder, Event};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Rusmppc produces a lot of logs while managing the SMPP connection in the background.
    // You can filter them out by setting the `rusmppc` target to `off`.
    tracing_subscriber::fmt()
        .with_env_filter("submit_sm=info,rusmpp=off,rusmppc=debug")
        .init();

    let (client, mut events) = ConnectionBuilder::new()
        .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
        .password(COctetString::from_str("rEZYMq5j")?)
        .system_type(COctetString::empty())
        .addr_ton(Ton::Unknown)
        .addr_npi(Npi::Unknown)
        .address_range(COctetString::empty())
        // bind as a transceiver
        .transceiver()
        // every 5 seconds send an enquire link command to the server
        .enquire_link_interval(Duration::from_secs(5))
        // if the server does not respond within 2 seconds, consider it a timeout
        .response_timeout(Duration::from_secs(2))
        .connect("127.0.0.1:2775")
        .await?;

    let client_clone = client.clone();

    let events_task = tokio::spawn(async move {
        // listen for events like incoming commands and background errors
        while let Some(event) = events.next().await {
            tracing::info!(?event, "Event");

            if let Event::Command(command) = event {
                if command.id() == CommandId::DeliverSm {
                    tracing::info!("Received DeliverSm");

                    let _ = client_clone
                        .deliver_sm_resp(command.sequence_number(), DeliverSmResp::default())
                        .await;
                }
            }
        }

        tracing::info!("Events stream closed");
    });

    client
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
        .await?;

    tracing::info!("SubmitSm command sent");

    // tokio::time::sleep(Duration::from_secs(20)).await;

    // tracing::info!("Unbinding from the server");

    // // Issue an unbind command to close the connection gracefully.

    // // You don't have to manually preform an unbind.
    // // When all clients are dropped, the connection will be closed automatically.

    // client.unbind().await?;

    // tracing::info!("Waiting for the client to terminate");

    // Wait for the client to terminate.
    let _ = client.terminated().await;

    let _ = events_task.await;

    Ok(())
}
