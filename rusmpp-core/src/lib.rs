#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

#[cfg(any(test, feature = "arbitrary"))]
extern crate std;

pub mod pdus;

pub mod values;

mod command_id;
pub use command_id::CommandId;

mod command_status;
pub use command_status::CommandStatus;

pub mod command;

pub mod fields;

pub mod decode;
pub mod encode;

pub mod types;

pub mod tlvs;

#[cfg(test)]
pub(crate) mod tests;

pub(crate) mod formatter;

#[cfg(any(feature = "log", feature = "defmt", feature = "tracing"))]
pub(crate) mod logging;
