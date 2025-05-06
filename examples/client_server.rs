//! Demonstrate a SMPP server and a client sending an EnquireLink
//! Run with
//!
//! ```not_rust
//! cargo run --example client_server --features="tokio-codec"
//! ```
//!

use futures::{SinkExt, StreamExt};
use rusmpp::{
    codec::CommandCodec,
    commands::{
        command::Command,
        pdu::Pdu,
        types::{command_id::CommandId, command_status::CommandStatus},
    },
};
use tokio::io::DuplexStream;
use tokio_util::codec::Framed;

async fn launch_server(server_stream: DuplexStream) -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async move {
        let mut framed = Framed::new(server_stream, CommandCodec::new());

        while let Some(Ok(command)) = framed.next().await {
            if let CommandId::EnquireLink = command.command_id() {
                println!("Server: EnquireLink received");
                let response = Command::new(
                    CommandStatus::EsmeRok,
                    command.sequence_number,
                    Pdu::EnquireLinkResp,
                );
                framed.send(&response).await.unwrap();
                println!("Server: EnquireLink response sent");
                break;
            }
        }
    });
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (server_stream, client_stream) = tokio::io::duplex(4096);
    launch_server(server_stream).await?;

    let mut framed = Framed::new(client_stream, CommandCodec::new());

    let enquire_link_command = Command::new(CommandStatus::EsmeRok, 0, Pdu::EnquireLink);
    println!("Client: EnquireLink sent");
    framed.send(&enquire_link_command).await?;

    while let Some(Ok(command)) = framed.next().await {
        if let CommandId::EnquireLinkResp = command.command_id() {
            println!("Client: EnquireLink response received");
            break;
        }
    }

    Ok(())
}
