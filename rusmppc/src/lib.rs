//! # Rusmppc
//!
//! A [`tokio`](https://docs.rs/tokio/latest/tokio/) based [SMPP v5](https://smpp.org/SMPP_v5.pdf) client.
//!
//!```rust, no_run
//! use std::{str::FromStr, time::Duration};
//!
//! use futures::StreamExt;
//! use rusmpp::{
//!     CommandId,
//!     pdus::{BindTransceiver, DeliverSmResp, SubmitSm},
//!     types::{COctetString, OctetString},
//!     values::{EsmClass, Npi, RegisteredDelivery, ServiceType, Ton},
//! };
//! use rusmppc::{ConnectionBuilder, Event};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let (client, mut events) = ConnectionBuilder::new()
//!         // Set the maximum command length for incoming commands.
//!         .max_command_length(1024)
//!         // Every 5 seconds send an enquire link command to the server.
//!         .enquire_link_interval(Duration::from_secs(5))
//!         // If the server did not respond within 3 seconds, close the connection.
//!         .enquire_link_response_timeout(Duration::from_secs(3))
//!         // If the server does not respond within 2 seconds, consider it a timeout.
//!         // The operation is assumed to have failed.
//!         .response_timeout(Duration::from_secs(2))
//!         .connect("127.0.0.1:2775")
//!         .await?;
//!     
//!     // Bind the client as a transceiver.
//!     client
//!         .bind_transceiver(
//!             BindTransceiver::builder()
//!                 .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
//!                 .password(COctetString::from_str("rEZYMq5j")?)
//!                 .system_type(COctetString::empty())
//!                 .addr_ton(Ton::Unknown)
//!                 .addr_npi(Npi::Unknown)
//!                 .address_range(COctetString::empty())
//!                 .build(),
//!         )
//!         .await?;
//!     
//!     let events = tokio::spawn(async move {
//!         // Listen for events like incoming commands and background errors.
//!         while let Some(event) = events.next().await {
//!             println!("Event: {:?}", event);
//!         }
//!     });
//!     
//!     // Send commands to the server and wait for the responses.
//!     let response = client
//!         .submit_sm(
//!             SubmitSm::builder()
//!                 .service_type(ServiceType::default())
//!                 .source_addr_ton(Ton::Unknown)
//!                 .source_addr_npi(Npi::Unknown)
//!                 .source_addr(COctetString::from_str("12345")?)
//!                 .destination_addr(COctetString::from_str("491701234567")?)
//!                 .esm_class(EsmClass::default())
//!                 .registered_delivery(RegisteredDelivery::request_all())
//!                 .short_message(OctetString::from_str("Hi, I am a short message.")?)
//!                 .build(),
//!         )
//!         .await?;
//!     
//!     println!("SubmitSm response: {:?}", response);
//!     
//!     // Send an unbind command to the server and wait for the response.
//!     client.unbind().await?;
//!     
//!     // Close the connection.
//!     client.close().await?;
//!     
//!     // Wait for the connection to be closed.
//!     client.closed().await;
//!     
//!     // When the connection is closed, the event stream will also be closed.
//!     events.await?;
//!     
//!     Ok(())
//! }
//! ```

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]

mod action;
pub(crate) use action::Action;

mod connection;
pub(crate) use connection::Connection;

mod builder;
pub use builder::ConnectionBuilder;

mod event;
pub use event::Event;

mod request;
pub(crate) use request::{CloseRequest, RegisteredRequest, Request, UnregisteredRequest};

mod timer;
pub(crate) use timer::Timer;

mod client;
pub use client::Client;

pub mod error;

mod command;
pub(crate) use command::CommandExt;

mod futures;
pub(crate) use futures::RequestFutureGuard;

mod response;
pub(crate) use response::PendingResponses;

#[cfg(test)]
mod tests;
