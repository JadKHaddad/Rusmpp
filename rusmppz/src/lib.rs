#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

//! ## Features
//!
//! - `tracing`: Enables logging using [`tracing`](https://docs.rs/tracing/latest/tracing/).
//! - `pretty-hex-fmt`: Logs byte slices like `[0x00, 0x00, 0x00, 0x6F]` instead of `[00, 00, 00, 6F]`, if `tracing` feature is enabled.
//! - `char-fmt`: Logs byte slices as characters, if `tracing` feature is enabled.

pub mod codec;

pub mod types;

pub mod decode;
pub mod encode;

pub use rusmpp_core::{CommandId, CommandStatus, command::borrowed::Command, pdus::borrowed::Pdu};

pub mod command;

pub mod values;

pub mod tlvs;

pub mod pdus;
