#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

#[cfg(any(test, feature = "arbitrary"))]
extern crate std;

#[cfg(any(test, feature = "alloc"))]
extern crate alloc;

pub mod types;

mod macros;

pub(crate) mod utils;

pub mod decode;
pub mod encode;
