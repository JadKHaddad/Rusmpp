#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

#[cfg(any(test, feature = "alloc", feature = "verbose"))]
extern crate alloc;

#[cfg(any(test, feature = "arbitrary"))]
extern crate std;

pub mod fields;

pub mod decode;
pub mod encode;

pub mod types;

pub(crate) mod utils;
