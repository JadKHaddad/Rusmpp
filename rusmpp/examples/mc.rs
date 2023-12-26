use std::str::FromStr;

use rusmpp::{
    io::{read::AsyncIoRead, write::AsyncIoWrite},
    pdus::{
        body::{
            bodies::{bind_resp::BindResp, submit_sm_resp::SubmitSmResp},
            pdu_body::PduBody,
        },
        pdu::Pdu,
        tlvs::{tlv::TLV, tlv_value::TLVValue},
        types::{
            command_id::{CommandId, NoBodyCommandId},
            command_status::CommandStatus,
            interface_version::InterfaceVersion,
        },
    },
    types::c_octet_string::COctetString,
};
use tokio::{io::BufReader, net::TcpListener};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind");

    loop {
        let (stream, _) = listener.accept().await.expect("Failed to accept");

        tokio::spawn(async move {
            println!("Accepted connection from {:?}", stream.peer_addr().unwrap());
            let (read, mut write) = stream.into_split();

            let mut buf_reader = BufReader::new(read);

            while let Ok(pdu) = Pdu::async_io_read(&mut buf_reader).await {
                println!("{:?}", pdu);
                println!();

                let sequence_number = pdu.sequence_number();

                match pdu.command_id() {
                    CommandId::Unbind => {}
                    CommandId::EnquireLink => {
                        let enquire_link_resp_pdu = Pdu::new_without_body(
                            NoBodyCommandId::EnquireLinkResp,
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
                    Some(PduBody::BindTransmitter(_)) => {}
                    Some(PduBody::BindReceiver(_)) => {}
                    Some(PduBody::BindTransceiver(_)) => {
                        let bind_transceiver_resp_pdu = Pdu::new(
                            CommandStatus::EsmeRok,
                            sequence_number,
                            PduBody::BindTransceiverResp(BindResp::new(
                                COctetString::from_str("Rusmpp").unwrap(),
                                Some(TLV::new(TLVValue::ScInterfaceVersion(
                                    InterfaceVersion::Smpp5_0,
                                ))),
                            )),
                        )
                        .unwrap();

                        bind_transceiver_resp_pdu
                            .async_io_write(&mut write)
                            .await
                            .unwrap();
                    }
                    Some(PduBody::SubmitSm(_)) => {
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
                    _ => {}
                }
            }
        });
    }
}
