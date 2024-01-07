#![allow(unused_variables)]

use rusmpp::{
    pdus::body::bodies::bind::{Bind, BindBuilder},
    prelude::*,
};
use rusmpp_io::types::c_octet_string::COctetString;
use std::str::FromStr;
use tokio::{io::BufReader, net::TcpStream};

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("34.242.18.250:2775")
        .await
        .expect("Failed to connect");

    let (reader, mut writer) = stream.into_split();

    let mut reader = BufReader::new(reader);

    let bind = Bind {
        system_id: COctetString::from_str("system_id").unwrap(),
        password: COctetString::from_str("pass").unwrap(),
        system_type: COctetString::from_str("a_type").unwrap(),
        interface_version: InterfaceVersion::Smpp5_0,
        addr_ton: Ton::Unknown,
        addr_npi: Npi::Unknown,
        address_range: COctetString::empty(),
    };

    // Alternative:
    // Unset values are set to default
    let bind = BindBuilder::default()
        .system_id(COctetString::from_str("system_id").unwrap())
        .password(COctetString::from_str("pass").unwrap())
        .system_type(COctetString::from_str("a_type").unwrap())
        .interface_version(InterfaceVersion::Smpp5_0)
        .build()
        .unwrap();

    // BindTransceiver
    let pdu = Pdu::new(
        CommandStatus::EsmeRok,
        SequenceNumber::new(1),
        PduBody::BindTransceiver(bind),
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
}
