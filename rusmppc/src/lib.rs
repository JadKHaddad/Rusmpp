//! # Rusmppc
//!
//! A [`tokio`](https://docs.rs/tokio/latest/tokio/) based [SMPP v5](https://smpp.org/SMPP_v5.pdf) client.
//! use std::{str::FromStr, time::Duration};
//!
//!```rust
//! use std::{str::FromStr, time::Duration};
//!
//! use futures::StreamExt;
//! use rusmpp::{
//!     types::COctetString,
//!     values::{Npi, Ton},
//! };
//! use rusmppc::ConnectionBuilder;
//!
//! let (client, mut events) = ConnectionBuilder::new()
//!     .system_id(COctetString::from_str("NfDfddEKVI0NCxO")?)
//!     .password(COctetString::from_str("rEZYMq5j")?)
//!     .system_type(COctetString::empty())
//!     .addr_ton(Ton::Unknown)
//!     .addr_npi(Npi::Unknown)
//!     .address_range(COctetString::empty())
//!     .transceiver()
//!     .enquire_link_interval(Duration::from_secs(5))
//!     .response_timeout(Duration::from_secs(2))
//!     .connect("127.0.0.1:2775")
//!     .await?;
//!
//! let events_task = tokio::spawn(async move {
//!     // Listen for events like incoming commands and background errors
//!     while let Some(event) = events.next().await {
//!         println!("{#:?}", event);
//!     }
//! });
//!
//! // Unbind and disconnect from the server
//! client.unbind().await?;
//!
//! let _ = client.terminated().await;
//!
//! let _ = events_task.await;
//!
//! Ok(())
//! ```

#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

mod action;

mod bind;
pub use bind::BindMode;

mod builder;
pub use builder::ConnectionBuilder;

mod client;
pub use client::Client;

mod command;
pub(crate) use command::CommandExt;

mod connection;

pub mod error;

mod event;
pub use event::Event;

mod session_state;

mod timer;

#[cfg(test)]
mod tests;
