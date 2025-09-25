/// Codec for encoding and decoding `SMPP` PDUs.
#[derive(Debug)]
#[non_exhaustive]
pub struct CommandCodec {}

impl CommandCodec {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for CommandCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "framez")]
#[cfg_attr(docsrs, doc(cfg(feature = "framez")))]
pub mod framez {}
