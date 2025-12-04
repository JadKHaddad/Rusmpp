//! Concatenation support.

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;
