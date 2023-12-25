pub mod io;
pub mod pdus;
pub mod types;

#[cfg(test)]
mod tests {
    use core::panic;
    use std::str::FromStr;

    use tokio::{
        io::{AsyncWriteExt, BufReader},
        net::{TcpListener, TcpStream},
    };

    use crate::{
        io::{read::AsyncIoRead, write::AsyncIoWrite},
        pdus::{
            body::{
                bodies::{bind::Bind, query_sm::QuerySm, submit_sm::SubmitSm},
                pdu_body::PduBody,
            },
            pdu::Pdu,
            tlvs::{
                tlv::MessageSubmissionRequestTLV, tlv_value::MessageSubmissionRequestTLVValue,
                tlv_values::alert_on_msg_delivery::AlertOnMsgDelivery,
            },
            types::{
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
            greater_than_u8::GreaterThanU8, no_fixed_size_octet_string::NoFixedSizeOctetString,
            octet_string::OctetString,
        },
    };

    fn create_default_bind() -> Bind {
        Bind {
            system_id: COctetString::from_str("SMPP3TEST").unwrap(),
            password: COctetString::from_str("secret08").unwrap(),
            system_type: COctetString::from_str("SUBMIT1").unwrap(),
            interface_version: InterfaceVersion::Smpp5_0,
            addr_ton: Ton::Unknown,
            addr_npi: Npi::Unknown,
            address_range: COctetString::empty(),
        }
    }

    fn create_default_submit_sm() -> SubmitSm {
        SubmitSm::new(
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
            RegisteredDelivery::new(
                MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure,
                SmeOriginatedAcknowledgement::NoReceiptSmeAcknowledgementRequested,
                IntermediateNotification::IntermediateNotificationRequested,
                0,
            ),
            ReplaceIfPresentFlag::default(),
            DataCoding::default(),
            GreaterThanU8::new(1).unwrap(),
            OctetString::from_str("Hi, I am a short message. I will be overridden :(").unwrap(),
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
        )
    }

    async fn write_pdu<'a, T>(pdu: &'a Pdu, stream: &'a mut T)
    where
        T: AsyncWriteExt + Send + Unpin + 'static,
    {
        println!("sending pdu: {:#?}", pdu);

        let mut pdu_bytes = Vec::new();
        pdu.async_io_write(&mut pdu_bytes)
            .await
            .expect("Failed to write pdu bytes to vec");

        println!("sending pdu bytes: {} bytes", pdu_bytes.len());
        for byte in pdu_bytes.iter() {
            print!("{:#02x}, ", byte);
        }
        println!();

        pdu.async_io_write(stream)
            .await
            .expect("Failed to write pdu bytes to steam");
    }

    async fn connect_send_recv(pdus: Vec<Pdu>) -> Vec<Pdu> {
        let mut stream = TcpStream::connect("34.242.18.250:2775")
            .await
            .expect("Failed to connect");

        for pdu in pdus.iter() {
            write_pdu(pdu, &mut stream).await;
        }

        let mut buf_reader = BufReader::new(stream);
        let mut incoming_pdus = Vec::new();

        for _ in 0..pdus.len() {
            match Pdu::async_io_read(&mut buf_reader).await {
                Ok(pdu) => {
                    println!("pdu: {:#?}", pdu);

                    incoming_pdus.push(pdu);
                }
                Err(e) => {
                    panic!("error parsing pdu: {}", e)
                }
            }
        }

        incoming_pdus
    }

    #[tokio::test]
    async fn bind_transmitter() {
        let pdu = Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(1),
            PduBody::BindTransmitter(create_default_bind()),
        )
        .unwrap();

        let pdus = connect_send_recv(vec![pdu]).await;
        let body = pdus[0].clone().into_body().expect("Expected pdu body");
        assert!(matches!(body, PduBody::BindTransmitterResp(_)));
    }

    #[tokio::test]
    async fn bind_receiver() {
        let pdu = Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(1),
            PduBody::BindReceiver(create_default_bind()),
        )
        .unwrap();

        let pdus = connect_send_recv(vec![pdu]).await;
        let body = pdus[0].clone().into_body().expect("Expected pdu body");
        assert!(matches!(body, PduBody::BindReceiverResp(_)));
    }

    #[tokio::test]
    async fn bind_transceiver() {
        let pdu = Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(1),
            PduBody::BindTransceiver(create_default_bind()),
        )
        .unwrap();

        let pdus = connect_send_recv(vec![pdu]).await;
        let body = pdus[0].clone().into_body().expect("Expected pdu body");
        assert!(matches!(body, PduBody::BindTransceiverResp(_)));
    }

    #[tokio::test]
    async fn bind_and_submit_sm_and_query_sm() {
        let stream = TcpStream::connect("34.242.18.250:2775")
            .await
            .expect("Failed to connect");

        let (read, mut write) = stream.into_split();
        let mut buf_reader = BufReader::new(read);

        let bind_transmitter_pdu = Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(1),
            PduBody::BindTransmitter(create_default_bind()),
        )
        .unwrap();
        write_pdu(&bind_transmitter_pdu, &mut write).await;

        match Pdu::async_io_read(&mut buf_reader).await {
            Ok(pdu) => {
                println!("BindTransmitterResp pdu: {:#?}", pdu);

                let body = pdu.into_body().expect("Expected pdu body");
                assert!(matches!(body, PduBody::BindTransmitterResp(_)));
            }
            Err(e) => {
                panic!("error parsing pdu: {}", e)
            }
        }

        let submit_sm_body = create_default_submit_sm();

        let source_addr_ton = submit_sm_body.source_addr_ton();
        let source_addr_npi = submit_sm_body.source_addr_npi();
        let source_addr = submit_sm_body.source_addr().clone();

        let submit_sm_pdu = Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(2),
            PduBody::SubmitSm(submit_sm_body),
        )
        .unwrap();
        write_pdu(&submit_sm_pdu, &mut write).await;

        let message_id = match Pdu::async_io_read(&mut buf_reader).await {
            Ok(pdu) => {
                println!("SubmitSmResp pdu: {:#?}", pdu);

                let body = pdu.into_body().expect("Expected pdu body");
                let PduBody::SubmitSmResp(submit_sm_resp) = body else {
                    panic!("Expected SubmitSmResp");
                };

                submit_sm_resp.message_id().clone()
            }
            Err(e) => {
                panic!("error parsing pdu: {}", e)
            }
        };

        let query_sm_pdu = Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(3),
            PduBody::QuerySm(QuerySm {
                message_id,
                source_addr_ton,
                source_addr_npi,
                source_addr,
            }),
        )
        .unwrap();
        write_pdu(&query_sm_pdu, &mut write).await;

        match Pdu::async_io_read(&mut buf_reader).await {
            Ok(pdu) => {
                println!("QuerySmResp pdu: {:#?}", pdu);

                let body = pdu.into_body().expect("Expected pdu body");
                assert!(matches!(body, PduBody::QuerySmResp(_)));
            }
            Err(e) => {
                panic!("error parsing pdu: {}", e)
            }
        };
    }

    #[tokio::test]
    #[ignore]
    async fn send_three() {
        let mut stream = TcpStream::connect("34.242.18.250:2775")
            .await
            .expect("Failed to connect");

        Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(1),
            PduBody::BindTransceiver(create_default_bind()),
        )
        .unwrap()
        .async_io_write(&mut stream)
        .await
        .expect("Failed to write pdu bytes to steam");

        Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(2),
            PduBody::BindTransmitter(create_default_bind()),
        )
        .unwrap()
        .async_io_write(&mut stream)
        .await
        .expect("Failed to write pdu bytes to steam");

        Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(3),
            PduBody::BindReceiver(create_default_bind()),
        )
        .unwrap()
        .async_io_write(&mut stream)
        .await
        .expect("Failed to write pdu bytes to steam");

        let mut buf_reader = BufReader::new(stream);
        for _ in 0..3 {
            match Pdu::async_io_read(&mut buf_reader).await {
                Ok(pdu) => {
                    println!("pdu: {:#?}", pdu);
                }
                Err(e) => {
                    eprintln!("error parsing pdu: {}", e)
                }
            }
        }
    }

    #[tokio::test]
    async fn invalid_pdu_with_generic_nack() {
        // TODO
    }

    #[tokio::test]
    #[ignore]
    async fn send_slow() {
        tokio::spawn(async move {
            let listener = TcpListener::bind("127.0.0.1:8080")
                .await
                .expect("Failed to bind");

            println!("Listening on: {}", listener.local_addr().unwrap());

            loop {
                let (stream, _) = listener.accept().await.expect("Failed to accept");
                tokio::spawn(async move {
                    let mut buf_reader = BufReader::new(stream);

                    match Pdu::async_io_read(&mut buf_reader).await {
                        Ok(pdu) => {
                            println!("Got pdu: {:#?}", pdu);
                        }
                        Err(e) => {
                            eprintln!("error parsing pdu: {}", e)
                        }
                    }
                });
            }
        });

        let pdu = Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(1),
            PduBody::BindTransceiver(create_default_bind()),
        )
        .unwrap();

        let mut pdu_bytes = Vec::new();
        pdu.async_io_write(&mut pdu_bytes)
            .await
            .expect("Failed to write pdu bytes to vec");

        let mut stream = TcpStream::connect("127.0.0.1:8080")
            .await
            .expect("Failed to connect");

        for byte in pdu_bytes {
            println!("Sending byte: {:#02x}", byte);
            stream
                .write_u8(byte)
                .await
                .expect("Failed to write pdu bytes to steam");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        println!("Done sending pdu bytes");
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
