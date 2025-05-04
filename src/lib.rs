// TODO: The doc test in failing on `ConnectionRefused` error. Create a Mock server to allow doc tests to pass. Remove the `ignore` tag.

//! # Rusmpp
//!
//! Rust implementation of the [SMPP v5](https://smpp.org/SMPP_v5.pdf) protocol.
//!
//! ```rust
//! use futures::{SinkExt, StreamExt};
//! use rusmpp::{
//!     codec::command_codec::CommandCodec,
//!     commands::{
//!         command::Command,
//!         pdu::Pdu,
//!         types::{command_id::CommandId, command_status::CommandStatus},
//!     },
//! };
//! use std::net::SocketAddr;
//! use tokio::net::{TcpListener, TcpStream};
//! use tokio_util::codec::{FramedRead, FramedWrite};
//!
//! async fn launch_server() -> Result<(), Box<dyn std::error::Error>> {
//!     let addr: SocketAddr = "127.0.0.1:2775".parse()?;
//!     let listener = TcpListener::bind(addr).await?;
//!     tokio::spawn(async move {
//!         loop {
//!             match listener.accept().await {
//!                 Ok((socket, _)) => {
//!                     tokio::spawn(async move {
//!                         let (reader, writer) = socket.into_split();
//!                         let mut framed_read = FramedRead::new(reader, CommandCodec {});
//!                         let mut framed_write = FramedWrite::new(writer, CommandCodec {});
//!
//!                         while let Some(Ok(command)) = framed_read.next().await {
//!                             if let CommandId::EnquireLink = command.command_id() {
//!                                 let response = Command::new(CommandStatus::EsmeRok, command.sequence_number, Pdu::EnquireLinkResp);
//!                                 framed_write.send(&response).await.unwrap();
//!                                 break;
//!                             }
//!                         }
//!                     });
//!                 }
//!                 Err(e) => {}
//!             }
//!         }
//!     });
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     launch_server().await?;
//!     let stream = TcpStream::connect("127.0.0.1:2775").await?;
//! 
//!     let (reader, writer) = stream.into_split();
//!     let mut framed_read = FramedRead::new(reader, CommandCodec {});
//!     let mut framed_write = FramedWrite::new(writer, CommandCodec {});
//!
//!     let enquire_link_command = Command::new(CommandStatus::EsmeRok, 0, Pdu::EnquireLink);
//!
//!     // Send commands.
//!     framed_write.send(&enquire_link_command).await?;
//!
//!     // Wait for responses.
//!     while let Some(Ok(command)) = framed_read.next().await {
//!         if let CommandId::EnquireLinkResp = command.command_id() {
//!             break;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]
// #![deny(missing_docs)]

#[cfg(feature = "tokio-codec")]
pub mod codec;
#[cfg(feature = "tokio-codec")]
pub use codec::command_codec::CommandCodec;

pub mod commands;
pub use commands::command::Command;
pub use commands::pdu;
pub use commands::pdu::Pdu;
pub use commands::tlvs::tlv::TLV;
pub use commands::tlvs::tlv_tag::TLVTag;
pub use commands::tlvs::tlv_value::TLVValue;
pub use commands::types::command_id::CommandId;
pub use commands::types::command_status::CommandStatus;

pub mod ende;

pub mod types;

mod macros;

pub(crate) mod utils;
