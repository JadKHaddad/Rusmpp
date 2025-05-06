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
//! use tokio::io::DuplexStream;
//! use tokio_util::codec::Framed;
//!
//! async fn launch_server(server_stream: DuplexStream) -> Result<(), Box<dyn std::error::Error>> {
//!     tokio::spawn(async move {
//!         let mut framed = Framed::new(server_stream, CommandCodec {});
//!
//!         while let Some(Ok(command)) = framed.next().await {
//!             if let CommandId::EnquireLink = command.command_id() {
//!                 let response = Command::new(CommandStatus::EsmeRok, command.sequence_number, Pdu::EnquireLinkResp);
//!                 framed.send(&response).await.unwrap();
//!                 break;
//!             }
//!         }
//!     });
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let (server_stream, client_stream) = tokio::io::duplex(4096);
//!     launch_server(server_stream).await?;
//!
//!     let mut framed = Framed::new(client_stream, CommandCodec {});
//!
//!     let enquire_link_command = Command::new(CommandStatus::EsmeRok, 0, Pdu::EnquireLink);
//!     framed.send(&enquire_link_command).await?;
//!
//!     while let Some(Ok(command)) = framed.next().await {
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
