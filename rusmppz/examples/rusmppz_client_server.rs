//! Demonstrate a `SMPP` server and a client sending an EnquireLink
//!
//! Run with
//!
//! ```not_rust
//! cargo run -p rusmppz --example rusmppz_client_server --features="framez tracing"
//! ```
//!

use core::error::Error;
use embedded_io_adapters::tokio_1::FromTokio;
use framez::{Framed, next};
use rusmppz::{Command, CommandId, CommandStatus, Pdu, codec::CommandCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Rusmppz produces a lot of logs while decoding and encoding PDUs.
    // You can filter them out by setting the `rusmppz` target to `off`,
    // or by disabling the `tracing` feature.
    tracing_subscriber::fmt()
        .with_env_filter("client=info,server=info,rusmppz=trace,framez=debug")
        .init();

    // In-memory duplex stream to simulate a server and client.
    let (client, server) = tokio::io::duplex(4096);

    let client = async move {
        let read_buf = &mut [0u8; 1024];
        let write_buf = &mut [0u8; 1024];

        // The CommandCodec encodes/decodes SMPP commands into/from bytes.
        let mut framed = Framed::new(
            CommandCodec::new(),
            FromTokio::new(client),
            read_buf,
            write_buf,
        );

        // Rusmppz takes care of setting the correct command ID.
        let command = Command::new(CommandStatus::EsmeRok, 1, Pdu::EnquireLink);

        info!(target: "client", "EnquireLink sent");

        framed.send(command).await?;

        while let Some(command) = next!(framed).transpose()? {
            if let CommandId::EnquireLinkResp = command.id() {
                info!(target: "client", "EnquireLink response received");

                break;
            }
        }

        Ok::<(), Box<dyn Error>>(())
    };

    let server = async move {
        let read_buf = &mut [0u8; 1024];
        let write_buf = &mut [0u8; 1024];

        let mut framed = Framed::new(
            CommandCodec::new(),
            FromTokio::new(server),
            read_buf,
            write_buf,
        );

        while let Some(command) = next!(framed).transpose()? {
            if let CommandId::EnquireLink = command.id() {
                info!(target: "server", "EnquireLink received");

                // We can also use the Command::builder() to create commands.
                let response = Command::builder()
                    .status(CommandStatus::EsmeRok)
                    .sequence_number(command.sequence_number())
                    .pdu(Pdu::EnquireLinkResp);

                framed.send(response).await?;

                info!(target: "server", "EnquireLink response sent");

                break;
            }
        }

        Ok::<(), Box<dyn Error>>(())
    };

    let (client, server) = tokio::join!(client, server);

    client?;
    server?;

    Ok(())
}
