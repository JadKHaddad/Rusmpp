#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_debug_implementations)]

#[cfg(any(test, feature = "arbitrary"))]
extern crate std;

pub mod types;

pub mod decode;
pub mod encode;

mod macros;
