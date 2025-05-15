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
        codec::command_codec::CommandCodec,
        commands::{
            command::Command,
            pdu::{submit_sm::SubmitSm, Pdu},
            tlvs::tlv::{
                broadcast_request::BroadcastRequestTlvValue,
                message_submission_request::MessageSubmissionRequestTlvValue,
            },
            types::{
                command_status::CommandStatus, data_coding::DataCoding, esm_class::EsmClass,
                interface_version::InterfaceVersion, npi::Npi,
                registered_delivery::RegisteredDelivery,
                replace_if_present_flag::ReplaceIfPresentFlag, service_type::ServiceType, ton::Ton,
                AlertOnMessageDelivery, BroadcastMessageClass, MessagePayload,
                MsAvailabilityStatus,
            },
        },
        pdu::{
            AlertNotification, BindReceiver, BindReceiverResp, BindTransceiver,
            BindTransceiverResp, BindTransmitter, BindTransmitterResp, BroadcastSm,
            BroadcastSmResp, CancelBroadcastSm, CancelSm, DataSm, DataSmResp, DeliverSm,
            DeliverSmResp, Outbind, QueryBroadcastSm, QueryBroadcastSmResp, QuerySm, QuerySmResp,
            ReplaceSm, SubmitMulti, SubmitMultiResp, SubmitSmResp,
        },
        types::{AnyOctetString, COctetString, OctetString},
        CommandId,
    };

    #[tokio::test]
    #[traced_test]
    async fn encode_decode() {
        let commands = vec![
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::BindTransmitter(BindTransmitter::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::BindTransmitterResp(BindTransmitterResp::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::BindReceiver(BindReceiver::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::BindReceiverResp(BindReceiverResp::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::BindTransceiver(BindTransceiver::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::BindTransceiverResp(BindTransceiverResp::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::Outbind(Outbind::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::AlertNotification(AlertNotification::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::SubmitSm(SubmitSm::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::SubmitSmResp(SubmitSmResp::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::QuerySm(QuerySm::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::QuerySmResp(QuerySmResp::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::DeliverSm(DeliverSm::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::DeliverSmResp(DeliverSmResp::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::DataSm(DataSm::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::DataSmResp(DataSmResp::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::CancelSm(CancelSm::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::ReplaceSm(ReplaceSm::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::SubmitMulti(SubmitMulti::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::SubmitMultiResp(SubmitMultiResp::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::BroadcastSm(BroadcastSm::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::BroadcastSmResp(BroadcastSmResp::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::QueryBroadcastSm(QueryBroadcastSm::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::QueryBroadcastSmResp(QueryBroadcastSmResp::default()),
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::CancelBroadcastSm(CancelBroadcastSm::default()),
            ),
            Command::new(Default::default(), Default::default(), Pdu::Unbind),
            Command::new(Default::default(), Default::default(), Pdu::UnbindResp),
            Command::new(Default::default(), Default::default(), Pdu::EnquireLink),
            Command::new(Default::default(), Default::default(), Pdu::EnquireLinkResp),
            Command::new(Default::default(), Default::default(), Pdu::GenericNack),
            Command::new(Default::default(), Default::default(), Pdu::CancelSmResp),
            Command::new(Default::default(), Default::default(), Pdu::ReplaceSmResp),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::CancelBroadcastSmResp,
            ),
            Command::new(
                Default::default(),
                Default::default(),
                Pdu::Other {
                    command_id: CommandId::Other(100),
                    body: AnyOctetString::new(b"SMPP"),
                },
            ),
        ];

        let (server, client) = tokio::io::duplex(1024);

        let mut framed_server = Framed::new(server, CommandCodec::new());
        let mut framed_client = Framed::new(client, CommandCodec::new());

        let server_commands = commands.clone();
        tokio::spawn(async move {
            for command in server_commands {
                framed_server
                    .send(command)
                    .await
                    .expect("Failed to send PDU");
            }
        });

        let mut client_commands = Vec::new();

        while let Some(Ok(command)) = framed_client.next().await {
            client_commands.push(command);
        }

        assert_eq!(client_commands.last(), commands.last());
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
            .command_status(CommandStatus::EsmeRok)
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
            .command_status(CommandStatus::EsmeRok)
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
            .command_status(CommandStatus::EsmeRok)
            .sequence_number(1)
            .pdu(
                BroadcastSm::builder()
                    .push_tlv(BroadcastRequestTlvValue::AlertOnMessageDelivery(
                        AlertOnMessageDelivery::UseMediumPriorityAlert,
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
}
