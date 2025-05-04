// TODO: The doc test in failing on `ConnectionRefused` error. Create a Mock server to allow doc tests to pass. Remove the `ignore` tag.

//! # Rusmpp
//!
//! Rust implementation of the [SMPP v5](https://smpp.org/SMPP_v5.pdf) protocol.
//!
//! ```rust, ignore
//! use futures::{SinkExt, StreamExt};
//! use rusmpp::{
//!     codec::command_codec::CommandCodec,
//!     commands::{
//!         command::Command,
//!         pdu::Pdu,
//!         types::{command_id::CommandId, command_status::CommandStatus},
//!     },
//! };
//! use tokio::net::TcpStream;
//! use tokio_util::codec::{FramedRead, FramedWrite};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let stream = TcpStream::connect("34.242.18.250:2775").await?;
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
