mod gsm;

pub use gsm::{Gsm7PackedCodec, Gsm7UnpackedCodec};

pub mod errors {
    pub use super::gsm::Gsm7EncodeError;
}

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;
