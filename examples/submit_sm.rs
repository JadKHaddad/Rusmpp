//! Run with
//!
//! ```not_rust
//! cargo run --example submit_sm --features tokio-codec
//! ```
//!

use futures::{SinkExt, StreamExt};
use rusmpp::{
    commands::{
        tlvs::tlv::message_submission_request::MessageSubmissionRequestTLVValue,
        types::{EsmClass, InterfaceVersion, Npi, RegisteredDelivery, ServiceType, Ton},
    },
    pdu::{Bind, SubmitSm},
    types::{COctetString, NoFixedSizeOctetString, OctetString},
    Command, CommandCodec, CommandId, CommandStatus, Pdu,
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
    let bind_transceiver_command = Command::builder()
        .command_status(CommandStatus::EsmeRok)
        .sequence_number(1)
        .pdu(
            Bind::builder()
                .system_id(
                    COctetString::from_str("NfDfddEKVI0NCxO").expect("Failed to create system_id"),
                )
                .password(COctetString::from_str("rEZYMq5j").expect("Failed to create password"))
                .system_type(COctetString::empty())
                .interface_version(InterfaceVersion::Smpp5_0)
                .addr_ton(Ton::Unknown)
                .addr_npi(Npi::Unknown)
                .address_range(COctetString::empty())
                .build()
                .into_bind_transceiver(),
        )
        .build();

    // Send commands.
    framed_write.send(bind_transceiver_command).await?;

    // Wait for responses.
    while let Some(Ok(command)) = framed_read.next().await {
        if let Pdu::BindTransceiverResp(_) = command.pdu() {
            println!("BindTransceiverResp received.");

            if let CommandStatus::EsmeRok = command.command_status {
                println!("Successful bind.");
                break;
            }
        }
    }

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
                .short_message(OctetString::from_str(
                    "Hi, I am a short message. I will be overridden :(",
                )?)
                .push_tlv(
                    MessageSubmissionRequestTLVValue::MessagePayload(
                        NoFixedSizeOctetString::from_str(
                            "Hi, I am a very long message. I will override the short message :D",
                        )?,
                    )
                    .into(),
                )
                .build()
                .into_submit_sm(),
        )
        .build();

    framed_write.send(submit_sm_command).await?;

    while let Some(Ok(command)) = framed_read.next().await {
        if let CommandId::SubmitSmResp = command.command_id() {
            println!("SubmitSmResp received.");

            if let CommandStatus::EsmeRok = command.command_status {
                println!("Successful submit.");
                break;
            }
        }
    }

    // TODO handle delivery receipt
    Ok(())
}
