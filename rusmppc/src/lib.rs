#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

mod action;

mod builder;
pub use builder::ConnectionBuilder;

mod client;
pub use client::Client;

mod connection;

pub mod error;

mod event;
pub use event::Event;
