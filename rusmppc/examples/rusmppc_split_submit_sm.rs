//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppc --example rusmppc_split_submit_sm
//! ```
//!

use std::str::FromStr;

use futures::StreamExt;
use rusmpp::{
    CommandId,
    codecs::{Gsm7Unpacked, UdhType},
    pdus::{BindTransceiver, DeliverSmResp, SubmitSm},
    types::COctetString,
    values::{EsmClass, Npi, RegisteredDelivery, ServiceType, Ton},
};
use rusmppc::{ConnectionBuilder, Event};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("rusmppc_submit_sm=info,rusmpp=off,rusmppc=debug")
        .init();

    let (client, mut events) = ConnectionBuilder::new()
        .connect("smpp://localhost:2775")
        .await?;

    client
        .bind_transceiver(
            BindTransceiver::builder()
                .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
                .password(COctetString::from_str("rEZYMq5j")?)
                .build(),
        )
        .await?;

    let client_clone = client.clone();

    let events = tokio::spawn(async move {
        while let Some(event) = events.next().await {
            tracing::info!(?event, "Event");

            if let Event::Incoming(command) = event {
                if command.id() == CommandId::DeliverSm {
                    let _ = client_clone
                        .deliver_sm_resp(command.sequence_number(), DeliverSmResp::default())
                        .await;
                }
            }
        }
    });

    let submit_sms = SubmitSm::builder()
        .service_type(ServiceType::default())
        .source_addr_ton(Ton::Unknown)
        .source_addr_npi(Npi::Unknown)
        .source_addr(COctetString::from_str("12345")?)
        .destination_addr(COctetString::from_str("491701234567")?)
        .esm_class(EsmClass::default())
        .registered_delivery(RegisteredDelivery::request_all())
        .build()
        .encode("]]]]]]]]]")
        .encoder(Gsm7Unpacked::new())
        .reference(0)
        .udh_type(UdhType::EightBit)
        .collect::<Vec<_>>()
        .map_err(|()| "Failed to encode short message")?;

    for submit_sm in submit_sms {
        let submit_sm = submit_sm?;

        let response = client.submit_sm(submit_sm).await?;

        tracing::info!(?response, "SubmitSmResp received");
    }

    client.unbind().await?;

    client.close().await?;

    events.await?;

    Ok(())
}
