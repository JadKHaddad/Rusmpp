mod gsm;
mod udh;

pub use udh::Udh;

pub mod borrowed;
#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;
