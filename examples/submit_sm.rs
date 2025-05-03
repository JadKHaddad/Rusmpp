//! Run with
//!
//! ```not_rust
//! cargo run --example submit_sm --features="tokio-codec tracing pretty-hex-fmt"
//! ```
//!

use futures::{SinkExt, StreamExt};
use rusmpp::{
    commands::{
        tlvs::tlv::message_submission_request::MessageSubmissionRequestTLVValue,
        types::{EsmClass, InterfaceVersion, Npi, RegisteredDelivery, ServiceType, Ton},
    },
    pdu::{Bind, SubmitSm},
    types::{AnyOctetString, COctetString, OctetString},
    Command, CommandCodec, CommandId, CommandStatus, Pdu, TLVTag,
};
use std::str::FromStr;
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up powerful logging.
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "rusmpp::codec::encode=trace,rusmpp::codec::decode=trace",
        );
    }

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let stream = TcpStream::connect("34.242.18.250:2775").await?;

    let (reader, writer) = stream.into_split();
    let mut framed_read = FramedRead::new(reader, CommandCodec {});
    let mut framed_write = FramedWrite::new(writer, CommandCodec {});

    // Build commands. Omitted values will be set to default.
    let bind_transceiver_command = Command::new(
        CommandStatus::EsmeRok,
        1,
        Bind::builder()
            .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
            .password(COctetString::from_str("rEZYMq5j")?)
            .system_type(COctetString::empty())
            .interface_version(InterfaceVersion::Smpp5_0)
            .addr_ton(Ton::Unknown)
            .addr_npi(Npi::Unknown)
            .address_range(COctetString::empty())
            .build()
            .into_bind_transceiver(),
    );

    // Send commands.
    framed_write.send(&bind_transceiver_command).await?;

    // Wait for responses.
    while let Some(Ok(command)) = framed_read.next().await {
        if let Some(Pdu::BindTransceiverResp(_)) = command.pdu() {
            println!("BindTransceiverResp received.");

            if let CommandStatus::EsmeRok = command.command_status {
                println!("Successful bind.");
                break;
            }
        }
    }

    let submit_sm_command = Command::new(
        CommandStatus::EsmeRok,
        2,
        SubmitSm::builder()
            .service_type(ServiceType::default())
            .source_addr_ton(Ton::Unknown)
            .source_addr_npi(Npi::Unknown)
            .source_addr(COctetString::from_str("A-Source")?)
            .destination_addr(COctetString::from_str("A-Dest")?)
            .esm_class(EsmClass::default())
            .registered_delivery(RegisteredDelivery::request_all())
            .short_message(OctetString::from_str(
                "Hi, I am a short message. I will be overridden :(",
            )?)
            .push_tlv(
                MessageSubmissionRequestTLVValue::MessagePayload(AnyOctetString::from_str(
                    "Hi, I am a very long message. I will override the short message :D",
                )?)
                .into(),
            )
            .build()
            .into_submit_sm(),
    );

    framed_write.send(&submit_sm_command).await?;

    'outer: while let Some(Ok(command)) = framed_read.next().await {
        match command.pdu() {
            Some(Pdu::SubmitSmResp(_)) => {
                println!("SubmitSmResp received.");

                if let CommandStatus::EsmeRok = command.command_status {
                    println!("Successful submit.");
                }
            }
            Some(Pdu::DeliverSm(deliver_sm)) => {
                println!("DeliverSm received.");

                for tlv in deliver_sm.tlvs().iter() {
                    if let TLVTag::ReceiptedMessageId = tlv.tag() {
                        println!("Delivery receipt received.");

                        break 'outer;
                    }
                }
            }
            _ => {}
        }
    }

    let unbind_command = Command::new(CommandStatus::EsmeRok, 3, Pdu::Unbind);

    framed_write.send(&unbind_command).await?;

    while let Some(Ok(command)) = framed_read.next().await {
        if let CommandId::UnbindResp = command.command_id() {
            println!("UnbindResp received.");

            if let CommandStatus::EsmeRok = command.command_status {
                println!("Successful unbind.");
                break;
            }
        }
    }

    Ok(())
}
