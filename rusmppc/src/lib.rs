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

mod response;
pub(crate) use response::PendingResponses;

mod session_state;

#[cfg(test)]
mod tests;
