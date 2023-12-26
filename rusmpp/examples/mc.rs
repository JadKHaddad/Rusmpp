use std::str::FromStr;

use rusmpp::{
    io::{length::IoLength, read::AsyncIoRead, write::AsyncIoWrite},
    pdus::{
        body::{
            bodies::{
                bind_resp::BindResp, query_sm_resp::QuerySmResp, submit_sm_resp::SubmitSmResp,
            },
            pdu_body::PduBody,
        },
        pdu::Pdu,
        tlvs::{tlv::TLV, tlv_value::TLVValue, tlv_values::message_state::MessageState},
        types::{
            command_id::CommandId, command_status::CommandStatus,
            interface_version::InterfaceVersion, sequence_number::SequenceNumber,
        },
    },
    types::{c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString},
};
use tokio::{io::BufReader, net::TcpListener};

fn bind_resp() -> BindResp {
    BindResp::new(
        COctetString::from_str("Rusmpp").unwrap(),
        Some(TLV::new(TLVValue::ScInterfaceVersion(
            InterfaceVersion::Smpp5_0,
        ))),
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
            while let Ok(pdu) = Pdu::async_io_read(&mut buf_reader).await {
                let mut bytes = Vec::new();
                pdu.async_io_write(&mut bytes).await.unwrap();

                println!("pdu: {:?}", pdu);
                println!("pdu length: {}", pdu.length());
                println!();
                print!("bytes: ");
                for byte in bytes.iter() {
                    print!("{:02x} ", byte);
                }
                println!();
                println!("bytes length: {}", bytes.len());
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

                        // fake it until you make it :v
                        let message_id =
                            COctetString::from_str("d482c07ab005d1e290891aff750b51aeae0e").unwrap();
                        let submit_sm_resp_pdu = Pdu::new(
                            CommandStatus::EsmeRok,
                            sequence_number,
                            PduBody::SubmitSmResp(SubmitSmResp::new(message_id, vec![])),
                        )
                        .unwrap();

                        submit_sm_resp_pdu.async_io_write(&mut write).await.unwrap();
                    }
                    Some(PduBody::QuerySm(body)) => {
                        // fake it until you make it. again :v
                        let message_id = body.message_id.clone();
                        let query_sm_resp_pdu = Pdu::new(
                            CommandStatus::EsmeRok,
                            sequence_number,
                            PduBody::QuerySmResp(QuerySmResp::new(
                                message_id,
                                EmptyOrFullCOctetString::empty(),
                                MessageState::Enroute,
                                0,
                            )),
                        )
                        .unwrap();

                        query_sm_resp_pdu.async_io_write(&mut write).await.unwrap();
                    }
                    _ => {}
                }
            }

            println!("Connection closed with {:?}", addr);
        });
    }
}
