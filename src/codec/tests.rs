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
            command_status::CommandStatus,
            data_coding::DataCoding,
            esm_class::EsmClass,
            interface_version::InterfaceVersion,
            npi::Npi,
            priority_flag::GsmSms,
            registered_delivery::RegisteredDelivery,
            replace_if_present_flag::ReplaceIfPresentFlag,
            service_type::{GenericServiceType, ServiceType},
            ton::Ton,
        },
    },
    types::{
        c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
        no_fixed_size_octet_string::NoFixedSizeOctetString, octet_string::OctetString,
    },
};

#[tokio::test]
async fn do_codec() {
    let stream = TcpStream::connect("34.242.18.250:2775")
        .await
        .expect("Failed to connect");

    let (reader, writer) = stream.into_split();
    let mut framed_read = FramedRead::new(reader, CommandCodec {});
    let mut framed_write = FramedWrite::new(writer, CommandCodec {});

    tokio::spawn(async move {
        while let Some(pdu) = framed_read.next().await {
            println!("{:#?}", pdu);
            println!();
        }
    });

    let enquire_link_command = Command::new(CommandStatus::EsmeRok, 0, Pdu::EnquireLink);

    framed_write
        .send(enquire_link_command)
        .await
        .expect("Failed to send PDU");

    let bind_transceiver_command = Command::new(
        CommandStatus::EsmeRok,
        1,
        Pdu::BindTransceiver(Bind {
            system_id: COctetString::from_str("NfDfddEKVI0NCxO")
                .expect("Failed to create system_id"),
            password: COctetString::from_str("rEZYMq5j").expect("Failed to create password"),
            system_type: COctetString::from_str("").expect("Failed to create system_type"),
            interface_version: InterfaceVersion::Smpp5_0,
            addr_ton: Ton::Unknown,
            addr_npi: Npi::Unknown,
            address_range: COctetString::empty(),
        }),
    );

    framed_write
        .send(bind_transceiver_command)
        .await
        .expect("Failed to send PDU");

    let submit_sm_command = Command::new(
        CommandStatus::EsmeRok,
        2,
        Pdu::SubmitSm(SubmitSm::new(
            ServiceType::new(GenericServiceType::default()).expect("Failed to create ServiceType"),
            Ton::Unknown,
            Npi::Unknown,
            COctetString::from_str("some_source").expect("Failed to create source"),
            Ton::Unknown,
            Npi::Unknown,
            COctetString::from_str("some_dest").expect("Failed to create dest"),
            EsmClass::default(),
            0,
            GsmSms::from(2).into(),
            EmptyOrFullCOctetString::empty(),
            EmptyOrFullCOctetString::empty(),
            // Use default values to "not" get a delivery receipt
            RegisteredDelivery::request_all(),
            ReplaceIfPresentFlag::default(),
            DataCoding::default(),
            0,
            OctetString::from_str("Hi, I am a short message. I will be overridden :(")
                .expect("Failed to create short message"),
            // Optional TLVs
            vec![MessageSubmissionRequestTLV::new(
                MessageSubmissionRequestTLVValue::MessagePayload(
                    NoFixedSizeOctetString::from_str(
                        "Hi, I am a very long message that will override the short message :D",
                    )
                    .expect("Failed to create message_payload"),
                ),
            )],
        )),
    );

    framed_write
        .send(submit_sm_command)
        .await
        .expect("Failed to send PDU");

    // wait for delivery receipt
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

    let unbind_command = Command::new(CommandStatus::EsmeRok, 3, Pdu::Unbind);

    framed_write
        .send(unbind_command)
        .await
        .expect("Failed to send PDU");

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
