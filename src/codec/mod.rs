//! A codec for encoding and decoding `SMPP` PDUs.

mod command_codec;
pub use command_codec::CommandCodec;

#[cfg(test)]
mod tests;
