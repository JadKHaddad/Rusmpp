pub mod io;
pub mod pdus;
pub mod types;

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use tokio::{
        io::{AsyncWriteExt, BufReader},
        net::{TcpListener, TcpStream},
    };

    use crate::{
        io::{read::AsyncIoRead, write::AsyncIoWrite},
        pdus::{
            body::{
                bodies::{bind::Bind, submit_sm::SubmitSm},
                pdu_body::PduBody,
            },
            pdu::Pdu,
            types::{
                command_status::CommandStatus,
                data_coding::DataCoding,
                esm_class::EsmClass,
                interface_version::InterfaceVersion,
                npi::Npi,
                priority_flag::PriorityFlag,
                registered_delivery::RegisteredDelivery,
                replace_if_present_flag::ReplaceIfPresentFlag,
                sequence_number::SequenceNumber,
                service_type::{GenericServiceType, ServiceType},
                ton::Ton,
            },
        },
        types::{
            c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
            greater_than_u8::GreaterThanU8, octet_string::OctetString,
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
            RegisteredDelivery::default(),
            ReplaceIfPresentFlag::default(),
            DataCoding::default(),
            GreaterThanU8::new(1).unwrap(),
            OctetString::from_str("Hi I am a message").unwrap(),
        )
    }

    async fn connect_send_recv(pdus: Vec<Pdu>) -> Vec<Pdu> {
        let mut stream = TcpStream::connect("34.242.18.250:2775")
            .await
            .expect("Failed to connect");

        for pdu in pdus.iter() {
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

            pdu.async_io_write(&mut stream)
                .await
                .expect("Failed to write pdu bytes to steam");
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
    async fn submit_sm() {
        let bind_transmitter_pdu = Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(1),
            PduBody::BindTransmitter(create_default_bind()),
        )
        .unwrap();

        let submit_pdu = Pdu::new(
            CommandStatus::EsmeRok,
            SequenceNumber::new(1),
            PduBody::SubmitSm(create_default_submit_sm()),
        )
        .unwrap();

        let pdus = connect_send_recv(vec![bind_transmitter_pdu, submit_pdu]).await;

        println!("BindTransmitterResp: {:#?}", pdus[0]);
        let body = pdus[0].clone().into_body().expect("Expected pdu body");
        assert!(matches!(body, PduBody::BindTransmitterResp(_)));

        println!("SubmitSmResp: {:#?}", pdus[1]);
        let body = pdus[1].clone().into_body().expect("Expected pdu body");
        assert!(matches!(body, PduBody::SubmitSmResp(_)));
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
