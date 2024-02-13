use std::str::FromStr;

use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite};

use crate::{
    codec::pdu_codec::PduCodec,
    pdus::{
        body::{bind::Bind, Body},
        pdu::Pdu,
        types::{
            command_id::CommandId, command_status::CommandStatus,
            interface_version::InterfaceVersion, npi::Npi, ton::Ton,
        },
    },
    types::c_octet_string::COctetString,
};

#[tokio::test]
async fn do_codec() {
    let stream = TcpStream::connect("34.242.18.250:2775")
        .await
        .expect("Failed to connect");

    let (reader, writer) = stream.into_split();
    let mut framed_read = FramedRead::new(reader, PduCodec {});
    let mut framed_write = FramedWrite::new(writer, PduCodec {});

    tokio::spawn(async move {
        while let Some(pdu) = framed_read.next().await {
            println!("{:?}", pdu);
            println!();
        }
    });

    let accuire_link_pdu = Pdu {
        command_id: CommandId::EnquireLink,
        command_status: CommandStatus::EsmeRok,
        sequence_number: 0,
        body: None,
    };

    framed_write
        .send(accuire_link_pdu)
        .await
        .expect("Failed to send PDU");

    let bind_transceiver_pdu = Pdu {
        command_id: CommandId::BindTransceiver,
        command_status: CommandStatus::EsmeRok,
        sequence_number: 1,
        body: Some(Body::BindTransceiver(Bind {
            system_id: COctetString::from_str("NfDfddEKVI0NCxO")
                .expect("Failed to create system_id"),
            password: COctetString::from_str("rEZYMq5j").expect("Failed to create password"),
            system_type: COctetString::from_str("").expect("Failed to create system_type"),
            interface_version: InterfaceVersion::Smpp5_0,
            addr_ton: Ton::Unknown,
            addr_npi: Npi::Unknown,
            address_range: COctetString::empty(),
        })),
    };

    framed_write
        .send(bind_transceiver_pdu)
        .await
        .expect("Failed to send PDU");

    let unbind_pdu = Pdu {
        command_id: CommandId::Unbind,
        command_status: CommandStatus::EsmeRok,
        sequence_number: 2,
        body: None,
    };

    framed_write
        .send(unbind_pdu)
        .await
        .expect("Failed to send PDU");

    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
}
