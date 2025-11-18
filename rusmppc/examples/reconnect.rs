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
use rusmppc::{Client, ConnectionBuilder, Event};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("rusmpp=off,rusmppc=debug")
        .init();

    let on_connect = |client: Client| async {
        client
            .bind_transceiver(
                BindTransceiver::builder()
                    .system_id(COctetString::from_str("NfDfddEKVI0NCxO").expect("Valid"))
                    .password(COctetString::from_str("rEZYMq5j").expect("Valid"))
                    .system_type(COctetString::empty())
                    .addr_ton(Ton::Unknown)
                    .addr_npi(Npi::Unknown)
                    .address_range(COctetString::empty())
                    .build(),
            )
            .await?;

        tracing::info!("Bound");

        Ok(client)
    };

    let (handle, mut events) = ConnectionBuilder::new()
        .enquire_link_interval(Duration::from_secs(5))
        .response_timeout(Duration::from_secs(2))
        .factory()
        .max_retries(10)
        .max_delay(Duration::from_secs(10))
        .linear_backoff(Duration::from_millis(100))
        .on_connect(on_connect)
        .connect("smpp://localhost:2775");

    let handle_clone = handle.clone();
    let events = tokio::spawn(async move {
        // Listen for events like incoming commands and background errors.
        while let Some(event) = events.next().await {
            tracing::info!(?event, "Event");

            if let Event::Incoming(command) = event {
                if command.id() == CommandId::DeliverSm {
                    tracing::info!("Received DeliverSm");

                    match handle_clone.get().await {
                        Some(client) => {
                            let _ = client
                                .deliver_sm_resp(
                                    command.sequence_number(),
                                    DeliverSmResp::default(),
                                )
                                .await;
                        }
                        None => {
                            tracing::info!("Factory closed");

                            break;
                        }
                    }
                }
            }
        }

        tracing::info!("Connection closed");
    });

    for _ in 0..3 {
        tracing::info!("Getting a connected client");

        // This call will block until a client is available.
        match handle.get().await {
            Some(client) => {
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

                tracing::info!("Sending SubmitSm");

                let response = client.submit_sm(submit.clone()).await?;

                tracing::info!(?response, "Got SubmitSmResp");
            }
            None => {
                tracing::info!("Factory closed");

                break;
            }
        }

        tokio::time::sleep(Duration::from_secs(3)).await;
    }

    if let Some(client) = handle.get().await {
        handle.close();

        client.unbind().await?;

        client.close().await?;

        client.closed().await;
    }

    assert!(handle.get().await.is_none());

    events.await?;

    Ok(())
}
