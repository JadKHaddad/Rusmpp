//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppc --example rusmppc_submit_sm_multipart
//! ```
//!

use std::str::FromStr;

use futures::StreamExt;
use rusmpp::{
    CommandId,
    pdus::{BindTransceiver, DeliverSmResp, SubmitSm},
    types::{COctetString, OctetString},
    values::{DataCoding, EsmClass, Npi, RegisteredDelivery, ServiceType, Ton},
};
use rusmppc::{ConnectionBuilder, Event};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("rusmppc_submit_sm_multipart=info,rusmpp=off,rusmppc=debug")
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

    // c-spell: disable
    let message = r##"GSM 3 parts : Hello world!

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€Hello world!

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€"##;
    // c-spell: enable

    let multipart = SubmitSm::builder()
        .source_addr_ton(Ton::Unknown)
        .source_addr_npi(Npi::Unknown)
        .source_addr(COctetString::from_str("12345")?)
        .destination_addr(COctetString::from_str("491701234567")?)
        // esm_class will be updated with UDHI indicator by the multipart builder.
        .esm_class(EsmClass::default())
        // data_coding will be overridden by the multipart builder to match the encoder.
        .data_coding(DataCoding::default())
        // short_message will be overridden by `short_message` of the multipart builder.
        .short_message(OctetString::from_str("Hi, I am a short message.")?)
        .build()
        .multipart()
        .short_message(message.as_bytes())
        .reference_u16(1)
        .gsm7_unpacked()
        .build()?;

    let total = multipart.len();

    tracing::info!(total = total, "Submitting multipart message");

    for (idx, sm) in multipart.enumerate() {
        tracing::info!(?sm, "Submitting part {}/{}", idx + 1, total);

        let response = client.submit_sm(sm).await?;

        tracing::info!(?response, "Submitted part {}/{}", idx + 1, total);
    }

    client.unbind().await?;

    client.close().await?;

    events.await?;

    Ok(())
}
