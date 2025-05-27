//! # Rusmppc
//!
//! A [`tokio`](https://docs.rs/tokio/latest/tokio/) based [SMPP v5](https://smpp.org/SMPP_v5.pdf) client.

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
