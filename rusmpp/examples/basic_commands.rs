//! Run with
//!
//! ```not_rust
//! cargo run --example basic_commands
//! ```
//!

use rusmpp::{
    pdus::{
        body::bodies::{bind::Bind, submit_multi::SubmitMulti, submit_or_deliver_sm::SubmitSm},
        types::{
            data_coding::DataCoding,
            dest_address::{DestAddress, DistributionListName, SmeAddress},
            esm_class::EsmClass,
            priority_flag::PriorityFlag,
            registered_delivery::{MCDeliveryReceipt, RegisteredDelivery},
            replace_if_present_flag::ReplaceIfPresentFlag,
            service_type::{GenericServiceType, ServiceType},
        },
    },
    prelude::*,
};
use rusmpp_io::types::{
    c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
    octet_string::OctetString,
};
use std::str::FromStr;
use tokio::{io::BufReader, net::TcpStream};

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("34.242.18.250:2775")
        .await
        .expect("Failed to connect");

    let (reader, mut writer) = stream.into_split();

    let mut reader = BufReader::new(reader);

    // BindTransceiver
    let pdu = Pdu::new(
        CommandStatus::EsmeRok,
        SequenceNumber::new(1),
        PduBody::BindTransceiver(Bind {
            system_id: COctetString::from_str("system_id").unwrap(),
            password: COctetString::from_str("pass").unwrap(),
            system_type: COctetString::from_str("a_type").unwrap(),
            interface_version: InterfaceVersion::Smpp5_0,
            addr_ton: Ton::Unknown,
            addr_npi: Npi::Unknown,
            address_range: COctetString::empty(),
        }),
    )
    .unwrap();

    pdu.async_io_write(&mut writer)
        .await
        .expect("Failed to write pdu bytes");

    while let Ok(pdu) = Pdu::async_io_read(&mut reader).await {
        println!("pdu: {:?}", pdu);
        if let CommandId::BindTransceiverResp = pdu.command_id() {
            println!("BindTransceiverResp received");
            break;
        }
    }

    let pdu = Pdu::new_without_body(
        CommandId::EnquireLink,
        CommandStatus::EsmeRok,
        SequenceNumber::new(2),
    )
    .unwrap();

    pdu.async_io_write(&mut writer)
        .await
        .expect("Failed to write pdu bytes");

    while let Ok(pdu) = Pdu::async_io_read(&mut reader).await {
        println!("pdu: {:?}", pdu);
        if let CommandId::EnquireLinkResp = pdu.command_id() {
            println!("EnquireLinkResp received");
            break;
        }
    }

    // SubmitSm
    let pdu = Pdu::new(
        CommandStatus::EsmeRok,
        SequenceNumber::new(3),
        PduBody::SubmitSm(SubmitSm::new(
                ServiceType::new(GenericServiceType::default()).unwrap(),
                Ton::Unknown,
                Npi::Unknown,
                COctetString::from_str("SomeSource").unwrap(),
                Ton::Unknown,
                Npi::Unknown,
                COctetString::from_str("SomeDest").unwrap(),
                EsmClass::default(),
                0,
                PriorityFlag::default(),
                EmptyOrFullCOctetString::default(),
                EmptyOrFullCOctetString::default(),
                RegisteredDelivery::new(
                    MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure,
                    Default::default(),
                    Default::default(),
                    Default::default(),
                ),
                ReplaceIfPresentFlag::default(),
                DataCoding::default(),
                0,
                OctetString::from_str("Hi, I am a short message.").unwrap(),
            vec![],
        )),
    )
    .unwrap();

    pdu.async_io_write(&mut writer)
        .await
        .expect("Failed to write pdu bytes");

    while let Ok(pdu) = Pdu::async_io_read(&mut reader).await {
        println!("pdu: {:?}", pdu);
        if let CommandId::SubmitSmResp = pdu.command_id() {
            println!("SubmitSmResp received");
            break;
        }
    }

    // SubmitMulti
    let pdu = Pdu::new(
        CommandStatus::EsmeRok,
        SequenceNumber::new(4),
        PduBody::SubmitMulti(SubmitMulti::new(
            ServiceType::new(GenericServiceType::default()).unwrap(),
            Ton::Unknown,
            Npi::Unknown,
            COctetString::from_str("SomeSource").unwrap(),
            vec![
                DestAddress::SmeAddress(SmeAddress::new(
                    Ton::Unknown,
                    Npi::Unknown,
                    COctetString::from_str("SomeDest").unwrap(),
                )),
                DestAddress::DistributionListName(DistributionListName::new(
                    COctetString::from_str("SomeDest").unwrap(),
                )),
            ],
            EsmClass::default(),
            0,
            PriorityFlag::default(),
            EmptyOrFullCOctetString::default(),
            EmptyOrFullCOctetString::default(),
            RegisteredDelivery::default(),
            ReplaceIfPresentFlag::default(),
            DataCoding::default(),
            0,
            OctetString::from_str("Hi, I am a short message.").unwrap(),
            vec![],
        )),
    )
    .unwrap();

    pdu.async_io_write(&mut writer)
        .await
        .expect("Failed to write pdu bytes");

    // Our test server doesn't support SubmitMulti, so we'll just ignore the response

    // lets give the server some time to send us the delivery receipt, if not already received
    // if already received, this will timeout
    tokio::select! {
        _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
            println!("No delivery receipt received after 5 seconds");
        },
        _ = async {
            while let Ok(pdu) = Pdu::async_io_read(&mut reader).await {
                println!("pdu: {:?}", pdu);
                if let Some(PduBody::DeliverSm(deliver_sm)) = pdu.body() {
                    for tlv in deliver_sm.tlvs().iter() {
                        if let Some(TLVValue::ReceiptedMessageId(message_id)) = tlv.value() {
                            println!("Delivery receipt received for message id: {:?}", message_id);
                            return;
                        }
                    }
                }
            }
        } => {

        }
    }

    // Unbind
    let pdu = Pdu::new_without_body(
        CommandId::Unbind,
        CommandStatus::EsmeRok,
        SequenceNumber::new(5),
    )
    .unwrap();

    pdu.async_io_write(&mut writer)
        .await
        .expect("Failed to write pdu bytes");

    while let Ok(pdu) = Pdu::async_io_read(&mut reader).await {
        println!("pdu: {:?}", pdu);
        if let CommandId::UnbindResp = pdu.command_id() {
            println!("UnbindResp received");
            break;
        }
    }
}
