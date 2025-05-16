#[cfg(feature = "tokio-codec")]
mod tokio {
    use std::str::FromStr;

    use futures::{SinkExt, StreamExt};
    use testcontainers::{
        core::{ContainerPort, WaitFor},
        runners::AsyncRunner,
        GenericImage,
    };
    use tokio::net::TcpStream;
    use tokio_util::codec::{Framed, FramedRead, FramedWrite};
    use tracing_test::traced_test;

    use crate::{
        codec::{command_codec::CommandCodec, tokio::DecodeError},
        commands::types::{
            broadcast_area_identifier::BroadcastAreaFormat, data_coding::DataCoding,
            esm_class::EsmClass, interface_version::InterfaceVersion, npi::Npi,
            registered_delivery::RegisteredDelivery, replace_if_present_flag::ReplaceIfPresentFlag,
            service_type::ServiceType, ton::Ton, AlertOnMessageDelivery, BroadcastAreaIdentifier,
            BroadcastMessageClass, MessagePayload, MsAvailabilityStatus,
        },
        encode::Length,
        pdus::{AlertNotification, BindTransceiver, BindTransmitter, BroadcastSm, SubmitSm},
        tests::test_commands,
        tlvs::{BroadcastRequestTlvValue, MessageSubmissionRequestTlvValue},
        types::{AnyOctetString, COctetString, OctetString},
        Command, CommandStatus, Pdu,
    };

    #[tokio::test]
    #[traced_test]
    async fn encode_decode() {
        let commands = test_commands();
        let (writer, reader) = tokio::io::duplex(128);

        let mut framed_writer = Framed::new(writer, CommandCodec::new());
        let mut framed_reader = Framed::new(reader, CommandCodec::new());

        let writer_commands = commands.clone();
        tokio::spawn(async move {
            for command in writer_commands {
                framed_writer
                    .send(command)
                    .await
                    .expect("Failed to send PDU");
            }
        });

        let mut client_commands = Vec::new();

        while let Some(Ok(command)) = framed_reader.next().await {
            client_commands.push(command);
        }

        assert_eq!(client_commands, commands);
    }

    #[tokio::test]
    async fn max_command_length() {
        let max_command_length = 16;

        let (writer, reader) = tokio::io::duplex(1024);

        let mut framed_writer = Framed::new(writer, CommandCodec::new());
        let mut framed_reader = Framed::new(
            reader,
            CommandCodec::new().with_max_command_length(max_command_length),
        );

        let command = Command::new(Default::default(), Default::default(), SubmitSm::default());
        let command_length = 4 + command.length();

        framed_writer.send(&command).await.unwrap();

        match framed_reader.next().await.unwrap().unwrap_err() {
            DecodeError::Length { actual, max } => {
                assert_eq!(actual, command_length);
                assert_eq!(max, max_command_length);
            }
            _ => {
                panic!("Decode must fail with `DecodeError::Length`")
            }
        }
    }

    // cargo test do_codec --features tokio-codec -- --ignored --nocapture
    #[tokio::test]
    #[ignore = "integration test"]
    async fn do_codec() {
        // See https://github.com/JadKHaddad/smpp-smsc-simulator
        let container = GenericImage::new("jadkhaddad/smpp-smsc-simulator", "1.0.0")
            .with_wait_for(WaitFor::message_on_stdout(
                "Listening for SMPP on port 2775",
            ))
            .with_exposed_port(ContainerPort::Tcp(2775))
            .start()
            .await
            .expect("Failed to start smpp-smsc-simulator");

        let port = container
            .get_host_port_ipv4(2775)
            .await
            .expect("Failed to get container port");

        let stream = TcpStream::connect(format!("127.0.0.1:{port}"))
            .await
            .expect("Failed to connect");

        let (reader, writer) = stream.into_split();
        let mut framed_read = FramedRead::new(reader, CommandCodec::new());
        let mut framed_write = FramedWrite::new(writer, CommandCodec::new());

        tokio::spawn(async move {
            while let Some(command) = framed_read.next().await {
                println!("{:#?}", command);
                println!();
            }
        });

        let enquire_link_command = Command::new(CommandStatus::EsmeRok, 0, Pdu::EnquireLink);

        framed_write
            .send(&enquire_link_command)
            .await
            .expect("Failed to send PDU");

        let bind_transceiver_command = Command::new(
            CommandStatus::EsmeRok,
            1,
            BindTransceiver::builder()
                .system_id(
                    COctetString::from_str("NfDfddEKVI0NCxO").expect("Failed to create system_id"), // cspell:disable-line
                )
                .password(COctetString::from_str("rEZYMq5j").expect("Failed to create password"))
                .system_type(COctetString::empty())
                .interface_version(InterfaceVersion::Smpp5_0)
                .addr_ton(Ton::Unknown)
                .addr_npi(Npi::Unknown)
                .address_range(COctetString::empty())
                .build(),
        );

        framed_write
            .send(&bind_transceiver_command)
            .await
            .expect("Failed to send PDU");

        let submit_sm_command = Command::new(
            CommandStatus::EsmeRok,
            2,
            SubmitSm::builder()
                .service_type(ServiceType::default())
                .source_addr_ton(Ton::Unknown)
                .source_addr_npi(Npi::Unknown)
                .source_addr(
                    COctetString::from_str("some_source").expect("Failed to create source"),
                )
                .dest_addr_ton(Ton::Unknown)
                .dest_addr_npi(Npi::Unknown)
                .destination_addr(
                    COctetString::from_str("some_dest").expect("Failed to create dest"),
                )
                .esm_class(EsmClass::default())
                // Use default values to "not" get a delivery receipt
                .registered_delivery(RegisteredDelivery::request_all())
                .replace_if_present_flag(ReplaceIfPresentFlag::default())
                .data_coding(DataCoding::default())
                .short_message(
                    OctetString::from_str("Hi, I am a short message. I will be overridden :(")
                        .expect("Failed to create short message"),
                )
                .push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                    MessagePayload::new(
                        AnyOctetString::from_str(
                            "Hi, I am a very long message that will override the short message :D",
                        )
                        .expect("Failed to create message_payload"),
                    ),
                ))
                .build(),
        );

        framed_write
            .send(&submit_sm_command)
            .await
            .expect("Failed to send PDU");

        // wait for delivery receipt
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        let unbind_command = Command::new(CommandStatus::EsmeRok, 3, Pdu::Unbind);

        framed_write
            .send(&unbind_command)
            .await
            .expect("Failed to send PDU");

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

    // cargo test bind_transmitter --features tokio-codec -- --ignored --nocapture
    #[tokio::test]
    #[ignore = "integration test"]
    async fn bind_transmitter() {
        let stream = TcpStream::connect("127.0.0.1:2775")
            .await
            .expect("Failed to connect");

        let mut framed = Framed::new(stream, CommandCodec::new());

        let bind_transmitter = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(BindTransmitter::builder().build());

        framed
            .send(bind_transmitter)
            .await
            .expect("Failed to send PDU");

        while let Some(command) = framed.next().await {
            println!("Received: {:#?}", command);
        }
    }

    // cargo test alert_notification --features tokio-codec -- --ignored --nocapture
    #[tokio::test]
    #[ignore = "integration test"]
    async fn alert_notification() {
        let stream = TcpStream::connect("127.0.0.1:2775")
            .await
            .expect("Failed to connect");

        let mut framed = Framed::new(stream, CommandCodec::new());

        let alert_notification = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(
                AlertNotification::builder()
                    .ms_availability_status(Some(MsAvailabilityStatus::Denied))
                    .build(),
            );

        framed
            .send(alert_notification)
            .await
            .expect("Failed to send PDU");

        while let Some(command) = framed.next().await {
            println!("Received: {:#?}", command);
        }
    }

    // cargo test broadcast_sm --features tokio-codec -- --ignored --nocapture
    #[tokio::test]
    #[ignore = "integration test"]
    async fn broadcast_sm() {
        let stream = TcpStream::connect("127.0.0.1:2775")
            .await
            .expect("Failed to connect");

        let mut framed = Framed::new(stream, CommandCodec::new());

        let broadcast_sm = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(
                BroadcastSm::builder()
                    .push_tlv(BroadcastRequestTlvValue::AlertOnMessageDelivery(
                        AlertOnMessageDelivery::UseMediumPriorityAlert,
                    ))
                    .push_tlv(BroadcastRequestTlvValue::BroadcastAreaIdentifier(
                        BroadcastAreaIdentifier::new(
                            BroadcastAreaFormat::Polygon,
                            AnyOctetString::new(b"Polygon Area"),
                        ),
                    ))
                    .push_tlv(BroadcastRequestTlvValue::BroadcastAreaIdentifier(
                        BroadcastAreaIdentifier::new(
                            BroadcastAreaFormat::AliasName,
                            AnyOctetString::new(b"AliasName Area"),
                        ),
                    ))
                    .push_tlv(BroadcastRequestTlvValue::BroadcastAreaIdentifier(
                        BroadcastAreaIdentifier::new(
                            BroadcastAreaFormat::EllipsoidArc,
                            AnyOctetString::new(b"EllipsoidArc Area"),
                        ),
                    ))
                    .push_tlv(BroadcastRequestTlvValue::BroadcastAreaIdentifier(
                        BroadcastAreaIdentifier::new(
                            BroadcastAreaFormat::EllipsoidArc,
                            AnyOctetString::new(b"EllipsoidArc Area 2"),
                        ),
                    ))
                    .push_tlv(BroadcastRequestTlvValue::BroadcastAreaIdentifier(
                        BroadcastAreaIdentifier::new(
                            BroadcastAreaFormat::EllipsoidArc,
                            AnyOctetString::new(b"EllipsoidArc Area 3"),
                        ),
                    ))
                    .push_tlv(BroadcastRequestTlvValue::BroadcastMessageClass(
                        BroadcastMessageClass::Class2,
                    ))
                    .build(),
            );

        framed.send(broadcast_sm).await.expect("Failed to send PDU");

        while let Some(command) = framed.next().await {
            println!("Received: {:#?}", command);
        }
    }

    // cargo test submit_sm --features tokio-codec -- --ignored --nocapture
    #[tokio::test]
    #[ignore = "integration test"]
    async fn submit_sm() {
        let stream = TcpStream::connect("127.0.0.1:2775")
            .await
            .expect("Failed to connect");

        let mut framed = Framed::new(stream, CommandCodec::new());

        let submit_sm = Command::builder()
            .status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(
                SubmitSm::builder()
                    .short_message(OctetString::new(b"Short Message").unwrap())
                    .push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                        MessagePayload::new(AnyOctetString::new(b"Message Payload")),
                    ))
                    .build(),
            );

        framed.send(submit_sm).await.expect("Failed to send PDU");

        while let Some(command) = framed.next().await {
            println!("Received: {:#?}", command);
        }
    }
}
