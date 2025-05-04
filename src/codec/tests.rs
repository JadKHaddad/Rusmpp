use std::str::FromStr;

use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{Framed, FramedRead, FramedWrite};
use tracing_test::traced_test;

use crate::{
    codec::command_codec::CommandCodec,
    commands::{
        command::Command,
        pdu::{bind::Bind, submit_sm::SubmitSm, Pdu},
        tlvs::tlv::message_submission_request::{
            MessageSubmissionRequestTLV, MessageSubmissionRequestTLVValue,
        },
        types::{
            command_status::CommandStatus, data_coding::DataCoding, esm_class::EsmClass,
            interface_version::InterfaceVersion, npi::Npi, registered_delivery::RegisteredDelivery,
            replace_if_present_flag::ReplaceIfPresentFlag, service_type::ServiceType, ton::Ton,
        },
    },
    pdu::{
        AlertNotification, BindResp, BroadcastSm, BroadcastSmResp, CancelBroadcastSm, CancelSm,
        DataSm, DeliverSm, Outbind, QueryBroadcastSm, QueryBroadcastSmResp, QuerySm, QuerySmResp,
        ReplaceSm, SmResp, SubmitMulti, SubmitMultiResp, SubmitSmResp,
    },
    types::{
        any_octet_string::AnyOctetString, c_octet_string::COctetString, octet_string::OctetString,
    },
    CommandId,
};

#[tokio::test]
#[traced_test]
async fn default_encode_decode() {
    let commands = vec![
        Command::new(
            Default::default(),
            Default::default(),
            Pdu::BindTransmitter(Bind::default()),
        ),
        Command::new(
            Default::default(),
            Default::default(),
            Pdu::BindTransmitterResp(BindResp::default()),
        ),
        Command::new(
            Default::default(),
            Default::default(),
            Pdu::BindReceiver(Bind::default()),
        ),
        Command::new(
            Default::default(),
            Default::default(),
            Pdu::BindReceiverResp(BindResp::default()),
        ),
        Command::new(
            Default::default(),
            Default::default(),
            Pdu::BindTransceiver(Bind::default()),
        ),
        Command::new(
            Default::default(),
            Default::default(),
            Pdu::BindTransceiverResp(BindResp::default()),
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
            Pdu::DataSm(DataSm::default()),
        ),
        Command::new(
            Default::default(),
            Default::default(),
            Pdu::DataSmResp(SmResp::default()),
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

    let mut framed_server = Framed::new(server, CommandCodec {});
    let mut framed_client = Framed::new(client, CommandCodec {});

    let server_commands = commands.clone();
    tokio::spawn(async move {
        for command in server_commands.iter() {
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
    let stream = TcpStream::connect("34.242.18.250:2775")
        .await
        .expect("Failed to connect");

    let (reader, writer) = stream.into_split();
    let mut framed_read = FramedRead::new(reader, CommandCodec {});
    let mut framed_write = FramedWrite::new(writer, CommandCodec {});

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
        Bind::builder()
            .system_id(
                COctetString::from_str("NfDfddEKVI0NCxO").expect("Failed to create system_id"), // cspell:disable-line
            )
            .password(COctetString::from_str("rEZYMq5j").expect("Failed to create password"))
            .system_type(COctetString::empty())
            .interface_version(InterfaceVersion::Smpp5_0)
            .addr_ton(Ton::Unknown)
            .addr_npi(Npi::Unknown)
            .address_range(COctetString::empty())
            .build()
            .into_bind_transceiver(),
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
            .source_addr(COctetString::from_str("some_source").expect("Failed to create source"))
            .dest_addr_ton(Ton::Unknown)
            .dest_addr_npi(Npi::Unknown)
            .destination_addr(COctetString::from_str("some_dest").expect("Failed to create dest"))
            .esm_class(EsmClass::default())
            // Use default values to "not" get a delivery receipt
            .registered_delivery(RegisteredDelivery::request_all())
            .replace_if_present_flag(ReplaceIfPresentFlag::default())
            .data_coding(DataCoding::default())
            .short_message(
                OctetString::from_str("Hi, I am a short message. I will be overridden :(")
                    .expect("Failed to create short message"),
            )
            .push_tlv(MessageSubmissionRequestTLV::new(
                MessageSubmissionRequestTLVValue::MessagePayload(
                    AnyOctetString::from_str(
                        "Hi, I am a very long message that will override the short message :D",
                    )
                    .expect("Failed to create message_payload"),
                ),
            ))
            .build()
            .into_submit_sm(),
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
