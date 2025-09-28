#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

#[cfg(any(test, feature = "arbitrary"))]
extern crate std;

pub mod codec;

pub mod types;

pub mod decode;
pub mod encode;

pub use rusmpp_core::{CommandId, CommandStatus, command::borrowed::Command, pdus::borrowed::Pdu};

pub mod command;

pub mod values;

pub mod tlvs;

pub mod pdus;

#[cfg(all(
    feature = "framez",
    any(feature = "log", feature = "defmt", feature = "tracing")
))]
pub(crate) mod logging;
