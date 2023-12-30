/*
A fake ESME that shows how to use rusmpp using the blocking API.

See esme.rs for the async version and more details.
*/

//! Run with
//!
//! ```not_rust
//! cargo run --example esme_blocking
//! ```
//!

use rusmpp::{
    pdus::{
        body::{
            bodies::{bind::Bind, s_sm::SSm, submit_sm::SubmitSm},
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
        octet_string::OctetString,
    },
};
use rusmpp_io::io::{read::IoRead, write::IoWrite};
use std::{io::BufReader, net::TcpStream, str::FromStr};

const BIND_TRANSCEIVER_SEQUENCE_NUMBER: u32 = 1;
const SUBMIT_SM_SEQUENCE_NUMBER: u32 = 2;

fn main() {
    let mut stream = TcpStream::connect("34.242.18.250:2775").expect("Failed to connect");

    let bind_transceiver_pdu = Pdu::new(
        CommandStatus::EsmeRok,
        SequenceNumber::new(BIND_TRANSCEIVER_SEQUENCE_NUMBER),
        PduBody::BindTransceiver(Bind {
            system_id: COctetString::from_str("SMPP3TEST").unwrap(),
            password: COctetString::from_str("secret08").unwrap(),
            system_type: COctetString::from_str("SUBMIT1").unwrap(),
            interface_version: InterfaceVersion::Smpp5_0,
            addr_ton: Ton::Unknown,
            addr_npi: Npi::Unknown,
            address_range: COctetString::empty(),
        }),
    )
    .unwrap();

    // Do the bind
    bind_transceiver_pdu
        .io_write(&mut stream)
        .expect("Failed to write pdu bytes");

    // Submit the message regardless of the bind status
    // we are not checking the bind status here
    let submit_sm = SubmitSm::new(
            SSm::new(ServiceType::new(GenericServiceType::default()).unwrap(),
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
            // Use default values to "not" get a delivery receipt
            RegisteredDelivery::new(
                MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure,
                SmeOriginatedAcknowledgement::NoReceiptSmeAcknowledgementRequested,
                IntermediateNotification::IntermediateNotificationRequested,
                0,
            ),
            ReplaceIfPresentFlag::default(),
            DataCoding::default(),
            0,
            OctetString::from_str("Hi, I am a short message.").unwrap()),
            // Optional TLVs
            vec![],
        );

    let submit_sm_pdu = Pdu::new(
        CommandStatus::EsmeRok,
        SequenceNumber::new(SUBMIT_SM_SEQUENCE_NUMBER),
        PduBody::SubmitSm(submit_sm),
    )
    .unwrap();

    submit_sm_pdu
        .io_write(&mut stream)
        .expect("Failed to write pdu bytes");

    std::thread::spawn(move || {
        let mut buf_reader = BufReader::new(stream);

        // Just read and print the PDUs
        // See esme.rs for more logic on how you may want to handle the PDUs
        while let Ok(pdu) = Pdu::io_read(&mut buf_reader) {
            println!("{:?}", pdu);
            println!();
        }

        println!("Connection closed");
    });

    // Here we are just waiting for a bit
    // See esme.rs for a more complete example
    std::thread::sleep(std::time::Duration::from_secs(10));
}
