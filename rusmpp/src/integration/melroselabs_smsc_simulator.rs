// https://melroselabs.com/services/smsc-simulator

use crate::{
    pdus::{
        body::bodies::{
            bind::Bind, cancel_sm::CancelSm, query_sm::QuerySm, s_sm::SSm, submit_sm::SubmitSm,
        },
        types::{
            data_coding::DataCoding,
            esm_class::EsmClass,
            priority_flag::PriorityFlag,
            registered_delivery::{
                IntermediateNotification, MCDeliveryReceipt, RegisteredDelivery,
                SmeOriginatedAcknowledgement,
            },
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
use std::{collections::HashMap, str::FromStr};
use tokio::{
    io::BufReader,
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
};

const EXPECTED_INTERFACE_VERSION: InterfaceVersion = InterfaceVersion::Smpp3_4;

async fn connect(host: &str, port: u16) -> TcpStream {
    TcpStream::connect(format!("{}:{}", host, port))
        .await
        .expect("Failed to connect")
}

async fn bind_tranmitter(
    sequence_number: SequenceNumber,
    system_id: &str,
    password: &str,
    buf_reader: &mut BufReader<OwnedReadHalf>,
    write: &mut OwnedWriteHalf,
) {
    let bind_transmitter_pdu = Pdu::new(
        CommandStatus::EsmeRok,
        sequence_number,
        PduBody::BindTransmitter(Bind {
            system_id: COctetString::from_str(system_id).unwrap(),
            password: COctetString::from_str(password).unwrap(),
            system_type: COctetString::empty(),
            interface_version: InterfaceVersion::Smpp5_0,
            addr_ton: Ton::Unknown,
            addr_npi: Npi::Unknown,
            address_range: COctetString::empty(),
        }),
    )
    .expect("Failed to create bind transmitter pdu");

    bind_transmitter_pdu
        .async_io_write(write)
        .await
        .expect("Failed to write pdu bytes");

    while let Ok(pdu) = Pdu::async_io_read(buf_reader).await {
        println!("{pdu:?}\n");
        if let CommandId::BindTransmitterResp = pdu.command_id() {
            if pdu.sequence_number() == sequence_number {
                assert!(matches!(pdu.command_status(), CommandStatus::EsmeRok));

                let Some(body) = pdu.into_body() else {
                    panic!("No body found");
                };

                let PduBody::BindTransmitterResp(resp) = body else {
                    panic!("Unexpected body: {body:?}");
                };

                assert!(matches!(
                    resp.sc_interface_version.unwrap().value(),
                    Some(TLVValue::ScInterfaceVersion(EXPECTED_INTERFACE_VERSION))
                ));

                break;
            }
        }
    }
}

async fn bind_receiver(
    sequence_number: SequenceNumber,
    system_id: &str,
    password: &str,
    buf_reader: &mut BufReader<OwnedReadHalf>,
    write: &mut OwnedWriteHalf,
) {
    let bind_receiver_pdu = Pdu::new(
        CommandStatus::EsmeRok,
        sequence_number,
        PduBody::BindReceiver(Bind {
            system_id: COctetString::from_str(system_id).unwrap(),
            password: COctetString::from_str(password).unwrap(),
            system_type: COctetString::empty(),
            interface_version: InterfaceVersion::Smpp5_0,
            addr_ton: Ton::Unknown,
            addr_npi: Npi::Unknown,
            address_range: COctetString::empty(),
        }),
    )
    .expect("Failed to create bind receiver pdu");

    bind_receiver_pdu
        .async_io_write(write)
        .await
        .expect("Failed to write pdu bytes");

    while let Ok(pdu) = Pdu::async_io_read(buf_reader).await {
        println!("{pdu:?}\n");
        if let CommandId::BindReceiverResp = pdu.command_id() {
            if pdu.sequence_number() == sequence_number {
                assert!(matches!(pdu.command_status(), CommandStatus::EsmeRok));

                let Some(body) = pdu.into_body() else {
                    panic!("No body found");
                };

                let PduBody::BindReceiverResp(resp) = body else {
                    panic!("Unexpected body: {body:?}");
                };

                assert!(matches!(
                    resp.sc_interface_version.unwrap().value(),
                    Some(TLVValue::ScInterfaceVersion(EXPECTED_INTERFACE_VERSION))
                ));

                break;
            }
        }
    }
}

async fn bind_transceiver(
    sequence_number: SequenceNumber,
    system_id: &str,
    password: &str,
    buf_reader: &mut BufReader<OwnedReadHalf>,
    write: &mut OwnedWriteHalf,
) {
    let bind_transceiver_pdu = Pdu::new(
        CommandStatus::EsmeRok,
        sequence_number,
        PduBody::BindTransceiver(Bind {
            system_id: COctetString::from_str(system_id).unwrap(),
            password: COctetString::from_str(password).unwrap(),
            system_type: COctetString::empty(),
            interface_version: InterfaceVersion::Smpp5_0,
            addr_ton: Ton::Unknown,
            addr_npi: Npi::Unknown,
            address_range: COctetString::empty(),
        }),
    )
    .expect("Failed to create bind transceiver pdu");

    bind_transceiver_pdu
        .async_io_write(write)
        .await
        .expect("Failed to write pdu bytes");

    while let Ok(pdu) = Pdu::async_io_read(buf_reader).await {
        println!("{pdu:?}\n");
        if let CommandId::BindTransceiverResp = pdu.command_id() {
            if pdu.sequence_number() == sequence_number {
                assert!(matches!(pdu.command_status(), CommandStatus::EsmeRok));

                let Some(body) = pdu.into_body() else {
                    panic!("No body found");
                };

                let PduBody::BindTransceiverResp(resp) = body else {
                    panic!("Unexpected body: {body:?}");
                };

                assert!(matches!(
                    resp.sc_interface_version.unwrap().value(),
                    Some(TLVValue::ScInterfaceVersion(EXPECTED_INTERFACE_VERSION))
                ));

                break;
            }
        }
    }
}

async fn enquire_link(
    sequence_number: SequenceNumber,
    buf_reader: &mut BufReader<OwnedReadHalf>,
    write: &mut OwnedWriteHalf,
) {
    let enquire_link_pdu = Pdu::new_without_body(
        CommandId::EnquireLink,
        CommandStatus::EsmeRok,
        sequence_number,
    )
    .expect("Failed to create enquire link pdu");

    enquire_link_pdu
        .async_io_write(write)
        .await
        .expect("Failed to write pdu bytes");

    while let Ok(pdu) = Pdu::async_io_read(buf_reader).await {
        println!("{pdu:?}\n");
        if let CommandId::EnquireLinkResp = pdu.command_id() {
            if pdu.sequence_number() == sequence_number {
                assert!(matches!(pdu.command_status(), CommandStatus::EsmeRok));
                break;
            }
        }
    }
}

async fn submit_sm_short_message(
    sequence_number: SequenceNumber,
    source_addr: &str,
    destination_addr: &str,
    buf_reader: &mut BufReader<OwnedReadHalf>,
    write: &mut OwnedWriteHalf,
) -> COctetString<1, 65> {
    let short_message = OctetString::from_str("Hi, I am a short message.").unwrap();

    let submit_sm = SubmitSm::new(
        SSm::new(
            ServiceType::new(GenericServiceType::default()).unwrap(),
            Ton::Unknown,
            Npi::Unknown,
            COctetString::from_str(source_addr).unwrap(),
            Ton::Unknown,
            Npi::Unknown,
            COctetString::from_str(destination_addr).unwrap(),
            EsmClass::default(),
            0,
            PriorityFlag::default(),
            EmptyOrFullCOctetString::empty(),
            EmptyOrFullCOctetString::empty(),
            RegisteredDelivery::default(),
            ReplaceIfPresentFlag::default(),
            DataCoding::default(),
            0,
            short_message.clone(),
        ),
        vec![],
    );

    let submit_sm_pdu = Pdu::new(
        CommandStatus::EsmeRok,
        sequence_number,
        PduBody::SubmitSm(submit_sm),
    )
    .unwrap();

    submit_sm_pdu
        .async_io_write(write)
        .await
        .expect("Failed to write pdu bytes");

    while let Ok(pdu) = Pdu::async_io_read(buf_reader).await {
        println!("{pdu:?}\n");
        assert!(matches!(pdu.command_status(), CommandStatus::EsmeRok));
        if let Some(PduBody::SubmitSmResp(submit_sm_resp)) = pdu.body() {
            if pdu.sequence_number() == sequence_number {
                return submit_sm_resp.message_id().clone();
            }
        }
    }

    COctetString::<1, 65>::empty()
}

async fn submit_sm_short_message_deliver_sm_receive_delivery(
    sequence_number: SequenceNumber,
    addr: &str,
    buf_reader: &mut BufReader<OwnedReadHalf>,
    write: &mut OwnedWriteHalf,
) -> COctetString<1, 65> {
    let short_message = OctetString::from_str("Hi, I am a short message.").unwrap();
    let submit_sm = SubmitSm::new(
        SSm::new(
            ServiceType::new(GenericServiceType::default()).unwrap(),
            Ton::Unknown,
            Npi::Unknown,
            COctetString::from_str(addr).unwrap(),
            Ton::Unknown,
            Npi::Unknown,
            COctetString::from_str(addr).unwrap(),
            EsmClass::default(),
            0,
            PriorityFlag::default(),
            EmptyOrFullCOctetString::empty(),
            EmptyOrFullCOctetString::empty(),
            RegisteredDelivery::new(
                MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure,
                SmeOriginatedAcknowledgement::BothDeliveryAndUserAcknowledgmentRequested,
                IntermediateNotification::IntermediateNotificationRequested,
                0,
            ),
            ReplaceIfPresentFlag::default(),
            DataCoding::default(),
            0,
            short_message.clone()
        ),
        vec![],
    );

    let submit_sm_pdu = Pdu::new(
        CommandStatus::EsmeRok,
        sequence_number,
        PduBody::SubmitSm(submit_sm),
    )
    .unwrap();

    submit_sm_pdu
        .async_io_write(write)
        .await
        .expect("Failed to write pdu bytes");

    let mut message_id = COctetString::<1, 65>::empty();
    let mut results = HashMap::<&str, bool>::new();

    results.insert("submit", false);
    results.insert("rec", false);
    results.insert("delivery", false);

    'pdu: while let Ok(pdu) = Pdu::async_io_read(buf_reader).await {
        println!("{pdu:?}\n");
        assert!(matches!(pdu.command_status(), CommandStatus::EsmeRok));
        match pdu.body() {
            Some(PduBody::SubmitSmResp(submit_sm_resp)) => {
                if pdu.sequence_number() == sequence_number {
                    message_id = submit_sm_resp.message_id().clone();

                    results.insert("submit", true);

                    if results.values().all(|v| *v) {
                        break 'pdu;
                    }
                }
            }
            Some(PduBody::DeliverSm(deliver_sm)) => {
                if deliver_sm.ssm().short_message() == &short_message {
                    results.insert("rec", true);

                    if results.values().all(|v| *v) {
                        break 'pdu;
                    }
                }
                for tlv in deliver_sm.tlvs().iter() {
                    if let Some(TLVValue::ReceiptedMessageId(msg_id)) = tlv.value() {
                        if msg_id == &message_id {
                            results.insert("delivery", true);

                            if results.values().all(|v| *v) {
                                break 'pdu;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    message_id
}

async fn submit_sm_message_payload_deliver_sm(
    sequence_number: SequenceNumber,
    addr: &str,
    buf_reader: &mut BufReader<OwnedReadHalf>,
    write: &mut OwnedWriteHalf,
) {
    todo!()
}

async fn query_sm(
    sequence_number: SequenceNumber,
    message_id: COctetString<1, 65>,
    addr: &str,
    buf_reader: &mut BufReader<OwnedReadHalf>,
    write: &mut OwnedWriteHalf,
) {
    let query_sm_pdu = Pdu::new(
        CommandStatus::EsmeRok,
        sequence_number,
        PduBody::QuerySm(QuerySm {
            message_id,
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr: COctetString::from_str(addr).unwrap(),
        }),
    )
    .unwrap();

    query_sm_pdu
        .async_io_write(write)
        .await
        .expect("Failed to write pdu bytes");

    while let Ok(pdu) = Pdu::async_io_read(buf_reader).await {
        println!("{pdu:?}\n");
        if let CommandId::QuerySmResp = pdu.command_id() {
            if pdu.sequence_number() == sequence_number {
                assert!(matches!(pdu.command_status(), CommandStatus::EsmeRok));
                break;
            }
        }
    }
}

async fn cancel_sm(
    sequence_number: SequenceNumber,
    message_id: COctetString<1, 65>,
    addr: &str,
    buf_reader: &mut BufReader<OwnedReadHalf>,
    write: &mut OwnedWriteHalf,
) {
    let query_sm_pdu = Pdu::new(
        CommandStatus::EsmeRok,
        sequence_number,
        PduBody::CancelSm(CancelSm {
            serivce_type: ServiceType::new(GenericServiceType::default()).unwrap(),
            message_id,
            source_addr_ton: Ton::Unknown,
            source_addr_npi: Npi::Unknown,
            source_addr: COctetString::from_str(addr).unwrap(),
            dest_addr_ton: Ton::Unknown,
            dest_addr_npi: Npi::Unknown,
            destination_addr: COctetString::from_str(addr).unwrap(),
        }),
    )
    .unwrap();

    query_sm_pdu
        .async_io_write(write)
        .await
        .expect("Failed to write pdu bytes");

    while let Ok(pdu) = Pdu::async_io_read(buf_reader).await {
        println!("{pdu:?}\n");
        if let CommandId::CancelSmResp = pdu.command_id() {
            if pdu.sequence_number() == sequence_number {
                assert!(matches!(pdu.command_status(), CommandStatus::EsmeRok));
                break;
            }
        }
    }
}

async fn submit_data() {
    todo!()
}

async fn generic_nack() {
    todo!()
}

async fn unbind() {
    todo!()
}

// Other pdus are not supported by the simulator

#[tokio::test]
#[ignore]
async fn integration() {
    dotenv::dotenv().ok();

    let host = std::env::var("HOST").expect("HOST not set");
    let port = std::env::var("PORT")
        .expect("PORT not set")
        .parse::<u16>()
        .expect("PORT is not a valid u16");
    let system_id = std::env::var("SYSTEM_ID").expect("SYSTEM_ID not set");
    let password = std::env::var("PASSWORD").expect("PASSWORD not set");

    {
        let stream = connect(&host, port).await;
        let (read, mut write) = stream.into_split();
        let mut buf_reader = BufReader::new(read);

        let sequence_number = SequenceNumber::new(1);
        bind_tranmitter(
            sequence_number,
            &system_id,
            &password,
            &mut buf_reader,
            &mut write,
        )
        .await;
    }

    {
        let stream = connect(&host, port).await;
        let (read, mut write) = stream.into_split();
        let mut buf_reader = BufReader::new(read);

        let sequence_number = SequenceNumber::new(1);
        bind_receiver(
            sequence_number,
            &system_id,
            &password,
            &mut buf_reader,
            &mut write,
        )
        .await;
    }

    let stream = connect(&host, port).await;
    let (read, mut write) = stream.into_split();
    let mut buf_reader = BufReader::new(read);

    let sequence_number = SequenceNumber::new(1);
    bind_transceiver(
        sequence_number,
        &system_id,
        &password,
        &mut buf_reader,
        &mut write,
    )
    .await;

    let sequence_number = SequenceNumber::new(2);
    enquire_link(sequence_number, &mut buf_reader, &mut write).await;

    let sequence_number = SequenceNumber::new(3);
    let message_id = submit_sm_short_message_deliver_sm_receive_delivery(
        sequence_number,
        &system_id,
        &mut buf_reader,
        &mut write,
    )
    .await;

    let sequence_number = SequenceNumber::new(4);
    query_sm(
        sequence_number,
        message_id,
        &system_id,
        &mut buf_reader,
        &mut write,
    )
    .await;

    let sequence_number = SequenceNumber::new(5);
    let message_id = submit_sm_short_message(
        sequence_number,
        &system_id,
        "SomeOtherAddress",
        &mut buf_reader,
        &mut write,
    )
    .await;

    let sequence_number = SequenceNumber::new(6);
    cancel_sm(
        sequence_number,
        message_id,
        &system_id,
        &mut buf_reader,
        &mut write,
    )
    .await;
}
