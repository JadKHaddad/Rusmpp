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

#[cfg(any(feature = "framez", feature = "tokio-codec"))]
pub(crate) mod logging;

#[cfg(feature = "framez")]
#[cfg_attr(docsrs, doc(cfg(feature = "framez")))]
pub mod framez;

#[cfg(feature = "tokio-codec")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-codec")))]
pub mod tokio_codec;
