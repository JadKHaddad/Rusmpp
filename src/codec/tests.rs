use std::str::FromStr;

use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::{
    codec::command_codec::CommandCodec,
    commands::{
        command::Command,
        pdu::{bind::Bind, submit_sm::SubmitSm, Pdu},
        tlvs::tlv::message_submission_request::{
            MessageSubmissionRequestTLV, MessageSubmissionRequestTLVValue,
        },
        types::{
            command_status::CommandStatus, data_coding::DataCoding, esm_class::EsmClass,
            interface_version::InterfaceVersion, npi::Npi, registered_delivery::RegisteredDelivery,
            replace_if_present_flag::ReplaceIfPresentFlag, service_type::ServiceType, ton::Ton,
        },
    },
    types::{
        c_octet_string::COctetString, no_fixed_size_octet_string::NoFixedSizeOctetString,
        octet_string::OctetString,
    },
};

#[tokio::test]
#[ignore = "integration test"]
async fn do_codec() {
    let stream = TcpStream::connect("34.242.18.250:2775")
        .await
        .expect("Failed to connect");

    let (reader, writer) = stream.into_split();
    let mut framed_read = FramedRead::new(reader, CommandCodec {});
    let mut framed_write = FramedWrite::new(writer, CommandCodec {});

    tokio::spawn(async move {
        while let Some(command) = framed_read.next().await {
            println!("{:#?}", command);
            println!();
        }
    });

    let enquire_link_command = Command::builder()
        .command_status(CommandStatus::EsmeRok)
        .sequence_number(0)
        .pdu(Pdu::EnquireLink)
        .build();

    framed_write
        .send(enquire_link_command)
        .await
        .expect("Failed to send PDU");

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

    framed_write
        .send(bind_transceiver_command)
        .await
        .expect("Failed to send PDU");

    let submit_sm_command = Command::builder()
        .command_status(CommandStatus::EsmeRok)
        .sequence_number(2)
        .pdu(
            SubmitSm::builder()
                .serivce_type(ServiceType::default())
                .source_addr_ton(Ton::Unknown)
                .source_addr_npi(Npi::Unknown)
                .source_addr(
                    COctetString::from_str("some_source").expect("Failed to create source"),
                )
                .dest_addr_ton(Ton::Unknown)
                .dest_addr_npi(Npi::Unknown)
                .destination_addr(
                    COctetString::from_str("some_dest").expect("Failed to create dest"),
                )
                .esm_class(EsmClass::default())
                // Use default values to "not" get a delivery receipt
                .registered_delivery(RegisteredDelivery::request_all())
                .replace_if_present_flag(ReplaceIfPresentFlag::default())
                .data_coding(DataCoding::default())
                .short_message(
                    OctetString::from_str("Hi, I am a short message. I will be overridden :(")
                        .expect("Failed to create short message"),
                )
                .push_tlv(MessageSubmissionRequestTLV::new(
                    MessageSubmissionRequestTLVValue::MessagePayload(
                        NoFixedSizeOctetString::from_str(
                            "Hi, I am a very long message that will override the short message :D",
                        )
                        .expect("Failed to create message_payload"),
                    ),
                ))
                .build()
                .into_submit_sm(),
        )
        .build();

    framed_write
        .send(submit_sm_command)
        .await
        .expect("Failed to send PDU");

    // wait for delivery receipt
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let unbind_command = Command::builder()
        .command_status(CommandStatus::EsmeRok)
        .sequence_number(3)
        .pdu(Pdu::Unbind)
        .build();

    framed_write
        .send(unbind_command)
        .await
        .expect("Failed to send PDU");

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
