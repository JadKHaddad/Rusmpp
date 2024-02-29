//! Run with
//!
//! ```not_rust
//! cargo run --example readme_example --features tokio-codec
//! ```
//!

use futures::{SinkExt, StreamExt};
use rusmpp::{
    codec::command_codec::CommandCodec,
    commands::{
        command::Command,
        pdu::Pdu,
        types::{command_id::CommandId, command_status::CommandStatus},
    },
};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream = TcpStream::connect("34.242.18.250:2775").await?;

    let (reader, writer) = stream.into_split();
    let mut framed_read = FramedRead::new(reader, CommandCodec {});
    let mut framed_write = FramedWrite::new(writer, CommandCodec {});

    let enquire_link_command = Command::new(CommandStatus::EsmeRok, 0, Pdu::EnquireLink);

    // Send commands.
    framed_write.send(&enquire_link_command).await?;

    // Wait for responses.
    while let Some(Ok(command)) = framed_read.next().await {
        if let CommandId::EnquireLinkResp = command.command_id() {
            break;
        }
    }

    Ok(())
}
