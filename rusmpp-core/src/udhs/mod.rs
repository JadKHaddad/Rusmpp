//! User Data Header (UDH).

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;

mod id;
pub use id::UdhId;

mod concatenated_short_message_16_bit;
pub use concatenated_short_message_16_bit::ConcatenatedShortMessage16Bit;

mod concatenated_short_message_8_bit;
pub use concatenated_short_message_8_bit::ConcatenatedShortMessage8Bit;

pub mod parts {
    pub use super::concatenated_short_message_8_bit::ConcatenatedShortMessage8BitParts;
    pub use super::concatenated_short_message_16_bit::ConcatenatedShortMessage16BitParts;
}

pub mod errors;
