mod gsm;

pub use gsm::Gsm7UnpackedCodec;

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;
