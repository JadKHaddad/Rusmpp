//! # Rusmpp
//!
//! Rust implementation of the [SMPP v5](https://smpp.org/SMPP_v5.pdf) protocol.
//!
//! ```rust
//! use futures::{SinkExt, StreamExt};
//! use rusmpp::{
//!     codec::CommandCodec,
//!     commands::{
//!         command::Command,
//!         pdu::Pdu,
//!         types::{command_id::CommandId, command_status::CommandStatus},
//!     },
//! };
//! use tokio::io::DuplexStream;
//! use tokio_util::codec::Framed;
//!
//! async fn launch_server(server_stream: DuplexStream) -> Result<(), Box<dyn core::error::Error>> {
//!     tokio::spawn(async move {
//!         let mut framed = Framed::new(server_stream, CommandCodec::new());
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
//!         if let CommandId::EnquireLinkResp = command.command_id() {
//!             break;
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]
// #![deny(missing_docs)]

pub mod codec;

pub mod commands;
pub use commands::command::Command;
pub use commands::pdu;
pub use commands::pdu::Pdu;
pub use commands::types::command_id::CommandId;
pub use commands::types::command_status::CommandStatus;

pub mod session;
pub use session::session_state::SessionState;

pub mod tlvs;
pub mod types;

mod macros;

pub(crate) mod utils;

pub mod decode;
pub mod encode;

#[cfg(test)]
pub(crate) mod tests;

// New stuff

mod dev;

#[cfg(any(test, feature = "tokio-codec"))]
pub(crate) use macros::debug;
#[cfg(any(test, feature = "tokio-codec"))]
pub(crate) use macros::error;
#[cfg(any(test, feature = "tokio-codec"))]
pub(crate) use macros::trace;

// TODO: no std
// TODO: rework the exports
