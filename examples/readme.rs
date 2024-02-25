//! Run with
//!
//! ```not_rust
//! cargo run --example readme --features tokio-codec
//! ```
//!

use futures::{SinkExt, StreamExt};
use rusmpp::{
    commands::types::{EsmClass, Npi, RegisteredDelivery, ServiceType, Ton},
    pdu::SubmitSm,
    types::OctetString,
    Command, CommandCodec, CommandId, CommandStatus,
};
use std::str::FromStr;
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("34.242.18.250:2775").await?;

    let (reader, writer) = stream.into_split();
    let mut framed_read = FramedRead::new(reader, CommandCodec {});
    let mut framed_write = FramedWrite::new(writer, CommandCodec {});

    // Build commands. Omitted values will be set to default.
    let submit_sm_command = Command::builder()
        .command_status(CommandStatus::EsmeRok)
        .sequence_number(2)
        .pdu(
            SubmitSm::builder()
                .serivce_type(ServiceType::default())
                .source_addr_ton(Ton::Unknown)
                .source_addr_npi(Npi::Unknown)
                .esm_class(EsmClass::default())
                .registered_delivery(RegisteredDelivery::request_all())
                .short_message(OctetString::from_str("Hi, I am a short message.")?)
                .build()
                .into_submit_sm(),
        )
        .build();

    // Send commands.
    framed_write.send(submit_sm_command).await?;

    // Wait for responses.
    while let Some(Ok(command)) = framed_read.next().await {
        if let CommandId::SubmitSmResp = command.command_id() {
            break;
        }
    }

    Ok(())
}
