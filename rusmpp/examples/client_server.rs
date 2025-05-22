//! Demonstrate a `SMPP` server and a client sending an EnquireLink
//! Run with
//!
//! ```not_rust
//! cargo run --example client_server --features="tokio-codec tracing"
//! ```
//!

use core::error::Error;
use futures::{SinkExt, StreamExt};
use rusmpp::{
    Command, CommandId, CommandStatus, Pdu,
    codec::{CommandCodec, tokio::EncodeError},
};
use tokio::io::DuplexStream;
use tokio_util::codec::Framed;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Rusmpp produces a lot of logs while decoding and encoding PDUs.
    // You can filter them out by setting the `rusmpp` target to `off`,
    // or by disabling the `tracing` feature.
    tracing_subscriber::fmt()
        .with_env_filter("client=info,server=info,rusmpp=trace")
        .init();

    // In-memory duplex stream to simulate a server and client.
    let (server_stream, client_stream) = tokio::io::duplex(4096);

    launch_server(server_stream).await?;

    // The CommandCodec encodes/decodes SMPP commands into/from bytes.
    let mut framed = Framed::new(client_stream, CommandCodec::new());

    // Rusmpp takes care of setting the correct command ID.
    let command = Command::new(CommandStatus::EsmeRok, 1, Pdu::EnquireLink);

    info!(target: "client", "EnquireLink sent");

    framed.send(command).await?;

    while let Some(Ok(command)) = framed.next().await {
        if let CommandId::EnquireLinkResp = command.id() {
            info!(target: "client", "EnquireLink response received");

            break;
        }
    }

    Ok(())
}

async fn launch_server(stream: DuplexStream) -> Result<(), Box<dyn Error>> {
    tokio::spawn(async move {
        let mut framed = Framed::new(stream, CommandCodec::new());

        while let Some(Ok(command)) = framed.next().await {
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

        Ok::<(), EncodeError>(())
    });

    Ok(())
}
