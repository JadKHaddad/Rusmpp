pub mod pdus;
pub use rusmpp_io::io;
pub use rusmpp_io::types;

#[cfg(test)]
mod tests {
    use core::panic;
    use std::str::FromStr;

    use crate::pdus::{
        body::{bodies::bind::Bind, pdu_body::PduBody},
        pdu::Pdu,
        types::{
            command_status::CommandStatus, interface_version::InterfaceVersion, npi::Npi,
            sequence_number::SequenceNumber, ton::Ton,
        },
    };
    use rusmpp_io::{
        io::{read::AsyncIoRead, write::AsyncIoWrite},
        types::c_octet_string::COctetString,
    };
    use tokio::{
        io::{AsyncWriteExt, BufReader},
        net::TcpStream,
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
}
