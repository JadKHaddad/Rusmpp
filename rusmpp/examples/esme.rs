/*
A fake ESME that shows how to use rusmpp.
*/

//! Run with
//!
//! ```not_rust
//! cargo run --example esme
//! ```

use rusmpp::{
    io::{read::AsyncIoRead, write::AsyncIoWrite},
    pdus::{
        body::{
            bodies::{bind::Bind, query_sm::QuerySm, s_sm::SSm, submit_sm::SubmitSm},
            pdu_body::PduBody,
        },
        pdu::Pdu,
        tlvs::{
            tlv::MessageSubmissionRequestTLV,
            tlv_value::{MessageSubmissionRequestTLVValue, TLVValue},
            tlv_values::alert_on_msg_delivery::AlertOnMsgDelivery,
        },
        types::{
            command_id::CommandId,
            command_status::CommandStatus,
            data_coding::DataCoding,
            esm_class::EsmClass,
            interface_version::InterfaceVersion,
            npi::Npi,
            priority_flag::PriorityFlag,
            registered_delivery::{
                IntermediateNotification, MCDeliveryReceipt, RegisteredDelivery,
                SmeOriginatedAcknowledgement,
            },
            replace_if_present_flag::ReplaceIfPresentFlag,
            sequence_number::SequenceNumber,
            service_type::{GenericServiceType, ServiceType},
            ton::Ton,
        },
    },
    types::{
        c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
        no_fixed_size_octet_string::NoFixedSizeOctetString, octet_string::OctetString,
    },
};
use std::{str::FromStr, sync::Arc};
use tokio::{
    io::BufReader,
    net::TcpStream,
    sync::{mpsc, Notify},
};

const BIND_TRANSCEIVER_SEQUENCE_NUMBER: u32 = 1;
const SUBMIT_SM_SEQUENCE_NUMBER: u32 = 2;
const QUERY_SM_SEQUENCE_NUMBER: u32 = 3;
const UNBIND_SEQUENCE_NUMBER: u32 = 4;

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("34.242.18.250:2775")
        .await
        .expect("Failed to connect");

    let (read, mut write) = stream.into_split();
    let notify = Arc::new(Notify::new());
    let notify_clone = notify.clone();
    let (query_sm_tx, mut query_sm_rx) = mpsc::channel::<COctetString<1, 65>>(1);
    let (is_delivered_tx, mut is_delivered_rx) = mpsc::channel::<COctetString<1, 65>>(1);

    tokio::spawn(async move {
        let bind_transceiver_pdu = Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(BIND_TRANSCEIVER_SEQUENCE_NUMBER),
            PduBody::BindTransceiver(Bind {
                system_id: COctetString::from_str("SMPP3TEST").unwrap(),
                password: COctetString::from_str("SUBMIT1").unwrap(),
                system_type: COctetString::from_str("").unwrap(),
                interface_version: InterfaceVersion::Smpp5_0,
                addr_ton: Ton::Unknown,
                addr_npi: Npi::Unknown,
                address_range: COctetString::empty(),
            }),
        )
        .unwrap();

        // Do the bind
        bind_transceiver_pdu
            .async_io_write(&mut write)
            .await
            .expect("Failed to write pdu bytes");

        // Wait until we are bound
        notify.notified().await;
        println!("We are bound!");
        println!();

        let submit_sm = SubmitSm::new(
            SSm::new(
                ServiceType::new(GenericServiceType::default()).unwrap(),
                Ton::Unknown,
                Npi::Unknown,
                COctetString::from_str("596848").unwrap(),
                Ton::Unknown,
                Npi::Unknown,
                COctetString::from_str("596848").unwrap(),
                EsmClass::default(),
                0,
                PriorityFlag::default(),
                EmptyOrFullCOctetString::from_str("").unwrap(),
                EmptyOrFullCOctetString::from_str("").unwrap(),
                // Use default values to "not" get a delivery receipt
                RegisteredDelivery::new(
                    MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure,
                    SmeOriginatedAcknowledgement::BothDeliveryAndUserAcknowledgmentRequested,
                    IntermediateNotification::IntermediateNotificationRequested,
                    0,
                ),
                ReplaceIfPresentFlag::default(),
                DataCoding::default(),
                0,
                OctetString::from_str("Hi, I am a short message. I will be overridden :(").unwrap(),
            ),
            // Optional TLVs
            vec![
                MessageSubmissionRequestTLV::new(MessageSubmissionRequestTLVValue::MessagePayload(
                    NoFixedSizeOctetString::from_str(
                        "Hi, I am a very long message that will override the short message :D",
                    )
                    .unwrap(),
                )),
                MessageSubmissionRequestTLV::new(
                    MessageSubmissionRequestTLVValue::AlertOnMsgDelivery(
                        AlertOnMsgDelivery::UseMobileDefaultAlert,
                    ),
                ),
                MessageSubmissionRequestTLV::new(MessageSubmissionRequestTLVValue::CallbackNum(
                    OctetString::from_str("0000").unwrap(),
                )),
            ],
        );

        let source_addr_ton = submit_sm.ssm().source_addr_ton();
        let source_addr_npi = submit_sm.ssm().source_addr_npi();
        let source_addr = submit_sm.ssm().source_addr().clone();

        // Submit the message
        let submit_sm_pdu = Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(SUBMIT_SM_SEQUENCE_NUMBER),
            PduBody::SubmitSm(submit_sm),
        )
        .unwrap();

        submit_sm_pdu
            .async_io_write(&mut write)
            .await
            .expect("Failed to write pdu bytes");

        let Some(message_id) = query_sm_rx.recv().await else {
            panic!("What happened to our channel!?");
        };

        println!("We did submit_sm!");
        println!("Message id: {}", message_id.to_string());
        println!();

        // Ok now wait until the message is delivered
        let Some(message_id) = is_delivered_rx.recv().await else {
            panic!("What happened to our channel!?");
        };

        println!(
            "Well, Message with id: {} is delivered!",
            message_id.to_string()
        );
        println!();

        // Query the status of the message, if you want :v
        let query_sm_pdu = Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(QUERY_SM_SEQUENCE_NUMBER),
            PduBody::QuerySm(QuerySm {
                message_id,
                source_addr_ton,
                source_addr_npi,
                source_addr,
            }),
        )
        .unwrap();

        query_sm_pdu
            .async_io_write(&mut write)
            .await
            .expect("Failed to write pdu bytes");

        notify.notified().await;
        println!("We did query_sm!");

        // Ok now we are done, let's unbind
        let unbind_pdu = Pdu::new_without_body(
            CommandId::Unbind,
            CommandStatus::EsmeRok,
            SequenceNumber::new(UNBIND_SEQUENCE_NUMBER),
        )
        .unwrap();

        unbind_pdu
            .async_io_write(&mut write)
            .await
            .expect("Failed to write pdu bytes");

        // Wait until we are unbound
        notify.notified().await;
        println!("We are unbound!");
    });

    let mut buf_reader = BufReader::new(read);

    while let Ok(pdu) = Pdu::async_io_read(&mut buf_reader).await {
        println!("{:?}", pdu);
        println!();

        match pdu.command_id() {
            CommandId::QuerySmResp => {
                if pdu.sequence_number().value == QUERY_SM_SEQUENCE_NUMBER {
                    notify_clone.notify_one();
                }
            }
            CommandId::UnbindResp => {
                if pdu.sequence_number().value == UNBIND_SEQUENCE_NUMBER {
                    notify_clone.notify_one();
                }
            }
            _ => {}
        }

        match pdu.body() {
            Some(PduBody::BindTransceiverResp(_)) => {
                if pdu.sequence_number().value == BIND_TRANSCEIVER_SEQUENCE_NUMBER {
                    notify_clone.notify_one();
                }
            }
            Some(PduBody::SubmitSmResp(submit_sm_resp)) => {
                if pdu.sequence_number().value == SUBMIT_SM_SEQUENCE_NUMBER {
                    let message_id = submit_sm_resp.message_id().clone();
                    if query_sm_tx.send(message_id).await.is_err() {
                        panic!("What happened to our channel!?");
                    }
                }
            }
            Some(PduBody::DeliverSm(deliver_sm)) => {
                for tlv in deliver_sm.tlvs().iter() {
                    if let Some(TLVValue::ReceiptedMessageId(message_id)) = tlv.value() {
                        if is_delivered_tx.send(message_id.clone()).await.is_err() {
                            panic!("What happened to our channel!?");
                        }
                    }
                }
            }
            _ => {}
        }
    }

    println!("Connection closed");
}
