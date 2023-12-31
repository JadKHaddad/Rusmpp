use rusmpp::{
    pdus::{
        body::bodies::{bind::Bind, s_sm::SSm, submit_sm::SubmitSm},
        types::{
            data_coding::DataCoding,
            esm_class::EsmClass,
            priority_flag::PriorityFlag,
            registered_delivery::RegisteredDelivery,
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

async fn connect() -> TcpStream {
    TcpStream::connect("34.242.18.250:2775")
        .await
        .expect("Failed to connect")
}

async fn bind() {
    let mut stream = connect().await;

    let pdu = Pdu::new(
        CommandStatus::EsmeRok,
        SequenceNumber::new(1),
        PduBody::BindTransmitter(Bind {
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

    pdu.async_io_write(&mut stream)
        .await
        .expect("Failed to write pdu bytes");
}

async fn submit_sm() {
    let mut stream = connect().await;

    let pdu = Pdu::new(
        CommandStatus::EsmeRok,
        SequenceNumber::new(1),
        PduBody::SubmitSm(SubmitSm::new(
            SSm::new(
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
                EmptyOrFullCOctetString::from_str("").unwrap(),
                EmptyOrFullCOctetString::from_str("").unwrap(),
                RegisteredDelivery::default(),
                ReplaceIfPresentFlag::default(),
                DataCoding::default(),
                0,
                OctetString::from_str("Hi, I am a short message.").unwrap(),
            ),
            vec![],
        )),
    )
    .unwrap();

    pdu.async_io_write(&mut stream)
        .await
        .expect("Failed to write pdu bytes");
}

async fn read() {
    let stream = connect().await;

    let mut reader = BufReader::new(stream);

    while let Ok(pdu) = Pdu::async_io_read(&mut reader).await {
        println!("pdu: {:#?}", pdu);
    }
}

async fn receive_delivery() {
    let stream = connect().await;

    let mut reader = BufReader::new(stream);

    while let Ok(pdu) = Pdu::async_io_read(&mut reader).await {
        if let Some(PduBody::DeliverSm(deliver_sm)) = pdu.into_body() {
            for tlv in deliver_sm.tlvs().iter() {
                if let Some(TLVValue::ReceiptedMessageId(message_id)) = tlv.value() {
                    println!("message_id: {:?} was delivered", message_id);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {}
