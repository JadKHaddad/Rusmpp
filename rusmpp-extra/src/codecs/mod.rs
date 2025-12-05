//! Encoding and decoding support.

pub mod errors;
pub mod gsm;

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;
