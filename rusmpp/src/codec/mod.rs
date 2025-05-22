//! Codec for encoding and decoding `SMPP` PDUs.

mod command_codec;
pub use command_codec::CommandCodec;

#[cfg(feature = "tokio-codec")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-codec")))]
pub use command_codec::tokio;

#[cfg(test)]
mod tests;
