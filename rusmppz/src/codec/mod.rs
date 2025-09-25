//! Codec for encoding and decoding `SMPP` PDUs.

mod command_codec;
pub use command_codec::CommandCodec;

#[cfg(feature = "framez")]
#[cfg_attr(docsrs, doc(cfg(feature = "framez")))]
pub use command_codec::framez;

#[cfg(test)]
mod tests;
