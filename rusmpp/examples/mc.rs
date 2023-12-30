/*
A fake MC that shows how to use rusmpp.
*/

//! Run with
//!
//! ```not_rust
//! cargo run --example mc
//! ```

use rusmpp::{
    io::{length::IoLength, read::AsyncIoRead, write::AsyncIoWrite},
    pdus::{
        body::{
            bodies::{
                bind_resp::BindResp, query_sm_resp::QuerySmResp,
                submit_or_data_sm_resp::SubmitOrDataSmResp,
            },
            pdu_body::PduBody,
        },
        pdu::Pdu,
        tlvs::tlv_values::message_state::MessageState,
        types::{
            command_id::CommandId, command_status::CommandStatus,
            interface_version::InterfaceVersion, sequence_number::SequenceNumber,
        },
    },
    types::{c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString},
};
use std::str::FromStr;
use tokio::{io::BufReader, net::TcpListener};

fn bind_resp() -> BindResp {
    BindResp::new(
        COctetString::from_str("Rusmpp").unwrap(),
        Some(InterfaceVersion::Smpp5_0),
    )
}

fn already_bound_pdu(command_id: CommandId, sequence_number: SequenceNumber) -> Pdu {
    Pdu::new_without_body(command_id, CommandStatus::EsmeRalybnd, sequence_number).unwrap()
}

fn incorret_bind_status_pdu(command_id: CommandId, sequence_number: SequenceNumber) -> Pdu {
    Pdu::new_without_body(command_id, CommandStatus::EsmeRinvbndsts, sequence_number).unwrap()
}

enum BindMode {
    Tx,
    Rx,
    Trx,
}

struct SentMessage {
    message_id: COctetString<1, 65>,
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind");

    loop {
        let (stream, _) = listener.accept().await.expect("Failed to accept");

        tokio::spawn(async move {
            let addr = stream.peer_addr().unwrap();
            println!("Accepted connection from {:?}", addr);

            let (read, mut write) = stream.into_split();

            let mut buf_reader = BufReader::new(read);

            let mut bind_mode: Option<BindMode> = None;
            let mut sent_messages: Vec<SentMessage> = Vec::new();
            while let Ok(pdu) = Pdu::async_io_read(&mut buf_reader).await {
                let mut bytes = Vec::new();
                pdu.async_io_write(&mut bytes).await.unwrap();

                println!("pdu: {:?}", pdu);
                println!("length: {}", pdu.length());
                print!("bytes: ");
                for byte in bytes.iter() {
                    print!("0x{:02x}, ", byte);
                }
                println!();

                let sequence_number = pdu.sequence_number();

                match pdu.command_id() {
                    CommandId::Unbind => {
                        let unbind_resp_pdu = Pdu::new_without_body(
                            CommandId::UnbindResp,
                            CommandStatus::EsmeRok,
                            sequence_number,
                        )
                        .unwrap();

                        unbind_resp_pdu.async_io_write(&mut write).await.unwrap();

                        bind_mode = None;
                    }
                    CommandId::EnquireLink => {
                        let enquire_link_resp_pdu = Pdu::new_without_body(
                            CommandId::EnquireLinkResp,
                            CommandStatus::EsmeRok,
                            sequence_number,
                        )
                        .unwrap();

                        enquire_link_resp_pdu
                            .async_io_write(&mut write)
                            .await
                            .unwrap();
                    }
                    _ => {}
                }

                match pdu.body() {
                    Some(PduBody::BindTransmitter(_)) => {
                        if bind_mode.is_some() {
                            let already_bound_pdu =
                                already_bound_pdu(CommandId::BindTransmitterResp, sequence_number);
                            already_bound_pdu.async_io_write(&mut write).await.unwrap();
                            continue;
                        }

                        let bind_transmitter_resp_pdu = Pdu::new(
                            CommandStatus::EsmeRok,
                            sequence_number,
                            PduBody::BindTransmitterResp(bind_resp()),
                        )
                        .unwrap();

                        bind_transmitter_resp_pdu
                            .async_io_write(&mut write)
                            .await
                            .unwrap();

                        bind_mode = Some(BindMode::Tx);
                    }
                    Some(PduBody::BindReceiver(_)) => {
                        if bind_mode.is_some() {
                            let already_bound_pdu =
                                already_bound_pdu(CommandId::BindReceiverResp, sequence_number);
                            already_bound_pdu.async_io_write(&mut write).await.unwrap();
                            continue;
                        }

                        let bind_receiver_resp_pdu = Pdu::new(
                            CommandStatus::EsmeRok,
                            sequence_number,
                            PduBody::BindReceiverResp(bind_resp()),
                        )
                        .unwrap();

                        bind_receiver_resp_pdu
                            .async_io_write(&mut write)
                            .await
                            .unwrap();

                        bind_mode = Some(BindMode::Rx);
                    }
                    Some(PduBody::BindTransceiver(_)) => {
                        if bind_mode.is_some() {
                            let already_bound_pdu =
                                already_bound_pdu(CommandId::BindTransceiverResp, sequence_number);
                            already_bound_pdu.async_io_write(&mut write).await.unwrap();
                            continue;
                        }

                        let bind_transceiver_resp_pdu = Pdu::new(
                            CommandStatus::EsmeRok,
                            sequence_number,
                            PduBody::BindTransceiverResp(bind_resp()),
                        )
                        .unwrap();

                        bind_transceiver_resp_pdu
                            .async_io_write(&mut write)
                            .await
                            .unwrap();

                        bind_mode = Some(BindMode::Trx);
                    }
                    Some(PduBody::SubmitSm(_)) => {
                        match bind_mode {
                            Some(BindMode::Tx) | Some(BindMode::Trx) => {}
                            _ => {
                                let incorret_bind_status_pdu = incorret_bind_status_pdu(
                                    CommandId::SubmitSmResp,
                                    sequence_number,
                                );
                                incorret_bind_status_pdu
                                    .async_io_write(&mut write)
                                    .await
                                    .unwrap();

                                continue;
                            }
                        }

                        let message_id = uuid::Uuid::new_v4().to_string();
                        let message_id = COctetString::from_str(&message_id).unwrap();

                        sent_messages.push(SentMessage {
                            message_id: message_id.clone(),
                        });

                        let submit_sm_resp_pdu = Pdu::new(
                            CommandStatus::EsmeRok,
                            sequence_number,
                            PduBody::SubmitSmResp(SubmitOrDataSmResp::new(message_id, vec![])),
                        )
                        .unwrap();

                        submit_sm_resp_pdu.async_io_write(&mut write).await.unwrap();
                    }
                    Some(PduBody::DataSm(_)) => {
                        match bind_mode {
                            Some(BindMode::Tx) | Some(BindMode::Trx) => {}
                            _ => {
                                let incorret_bind_status_pdu = incorret_bind_status_pdu(
                                    CommandId::DataSmResp,
                                    sequence_number,
                                );
                                incorret_bind_status_pdu
                                    .async_io_write(&mut write)
                                    .await
                                    .unwrap();

                                continue;
                            }
                        }

                        let message_id = uuid::Uuid::new_v4().to_string();
                        let message_id = COctetString::from_str(&message_id).unwrap();

                        sent_messages.push(SentMessage {
                            message_id: message_id.clone(),
                        });

                        let data_sm_resp_pdu = Pdu::new(
                            CommandStatus::EsmeRok,
                            sequence_number,
                            PduBody::DataSmResp(SubmitOrDataSmResp::new(message_id, vec![])),
                        )
                        .unwrap();

                        data_sm_resp_pdu.async_io_write(&mut write).await.unwrap();
                    }
                    Some(PduBody::QuerySm(body)) => {
                        let message_id = &body.message_id;

                        let query_sm_resp_pdu =
                            if sent_messages.iter().any(|m| m.message_id == *message_id) {
                                Pdu::new(
                                    CommandStatus::EsmeRok,
                                    sequence_number,
                                    PduBody::QuerySmResp(QuerySmResp::new(
                                        message_id.clone(),
                                        EmptyOrFullCOctetString::empty(),
                                        MessageState::Enroute,
                                        0,
                                    )),
                                )
                                .unwrap()
                            } else {
                                Pdu::new_without_body(
                                    CommandId::QuerySmResp,
                                    // Query failed
                                    CommandStatus::EsmeRqueryfail,
                                    sequence_number,
                                )
                                .unwrap()
                            };

                        query_sm_resp_pdu.async_io_write(&mut write).await.unwrap();
                    }
                    Some(PduBody::CancelSm(body)) => {
                        let message_id = &body.message_id;

                        let cancel_sm_resp_pdu =
                            if sent_messages.iter().any(|m| m.message_id == *message_id) {
                                Pdu::new_without_body(
                                    CommandId::CancelSmResp,
                                    CommandStatus::EsmeRok,
                                    sequence_number,
                                )
                                .unwrap()
                            } else {
                                Pdu::new_without_body(
                                    CommandId::CancelSmResp,
                                    // Invalid message id
                                    CommandStatus::EsmeRinvmsgid,
                                    sequence_number,
                                )
                                .unwrap()
                            };

                        cancel_sm_resp_pdu.async_io_write(&mut write).await.unwrap();
                    }
                    Some(PduBody::ReplaceSm(_)) => {
                        let replace_sm_resp_pdu = Pdu::new_without_body(
                            CommandId::ReplaceSmResp,
                            CommandStatus::EsmeRok,
                            sequence_number,
                        )
                        .unwrap();

                        replace_sm_resp_pdu
                            .async_io_write(&mut write)
                            .await
                            .unwrap();
                    }
                    _ => {}
                }
            }

            println!("Connection closed with {:?}", addr);
        });
    }
}
