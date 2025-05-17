//! # Rusmpp
//!
//! Rust implementation of the [SMPP v5](https://smpp.org/SMPP_v5.pdf) protocol.
//!
//! ```rust
//! use futures::{SinkExt, StreamExt};
//! use rusmpp::{codec::CommandCodec, Command, CommandId, CommandStatus, Pdu};
//! use tokio::io::DuplexStream;
//! use tokio_util::codec::Framed;
//!
//! async fn launch_server(server_stream: DuplexStream) -> Result<(), Box<dyn core::error::Error>> {
//!     tokio::spawn(async move {
//!         let mut framed = Framed::new(server_stream, CommandCodec::new());
//!
//!         while let Some(Ok(command)) = framed.next().await {
//!             if let CommandId::EnquireLink = command.id() {
//!                 let response = Command::new(CommandStatus::EsmeRok, command.sequence_number, Pdu::EnquireLinkResp);
//!                 framed.send(&response).await.unwrap();
//!                 break;
//!             }
//!         }
//!     });
//!
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn core::error::Error>> {
//!     let (server_stream, client_stream) = tokio::io::duplex(4096);
//!     launch_server(server_stream).await?;
//!
//!     let mut framed = Framed::new(client_stream, CommandCodec::new());
//!
//!     let enquire_link_command = Command::new(CommandStatus::EsmeRok, 0, Pdu::EnquireLink);
//!     framed.send(&enquire_link_command).await?;
//!
//!     while let Some(Ok(command)) = framed.next().await {
//!         if let CommandId::EnquireLinkResp = command.id() {
//!             break;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]
// #![deny(missing_docs)]

#[cfg(any(test, feature = "tokio-codec"))]
extern crate std;

extern crate alloc;

pub mod codec;

pub mod command;
pub use command::command_id::CommandId;
pub use command::command_status::CommandStatus;
pub use command::inner::Command;

pub mod pdus;
pub use pdus::pdu::Pdu;

pub mod session;

pub mod tlvs;

pub mod types;

pub mod values;

mod macros;

pub(crate) mod utils;

pub mod decode;
pub mod encode;

#[cfg(test)]
pub(crate) mod tests;

#[cfg(any(test, feature = "tokio-codec"))]
pub(crate) use macros::debug;
#[cfg(any(test, feature = "tokio-codec"))]
pub(crate) use macros::error;
#[cfg(any(test, feature = "tokio-codec"))]
pub(crate) use macros::trace;
