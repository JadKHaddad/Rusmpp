//! Run with
//!
//! ```not_rust
//! cargo run --example simple --features tokio-codec
//! ```
//!

use futures::{SinkExt, StreamExt};
use rusmpp::{Command, CommandId, CommandStatus, Pdu, codec::CommandCodec};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let stream = TcpStream::connect("127.0.0.1:2775").await?;

    let mut framed = Framed::new(stream, CommandCodec::new());

    let command = Command::new(CommandStatus::EsmeRok, 1, Pdu::EnquireLink);

    // Send commands.
    framed.send(command).await?;

    // Wait for responses.
    while let Some(Ok(command)) = framed.next().await {
        if let CommandId::EnquireLinkResp = command.id() {
            break;
        }
    }

    Ok(())
}
