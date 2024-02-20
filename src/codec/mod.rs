//! Tokio codec for encoding and decoding SMPP PDUs.
//! Only available when the `tokio-codec` feature is enabled.

pub mod command_codec;
#[cfg(test)]
mod tests;
