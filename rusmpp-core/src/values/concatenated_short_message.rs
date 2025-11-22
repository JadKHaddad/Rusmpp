use crate::udhs::{ConcatenatedShortMessage8Bit, ConcatenatedShortMessage16Bit};

/// Concatenated short message.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConcatenatedShortMessage {
    /// 8-bit reference number concatenated short message.
    EightBit(ConcatenatedShortMessage8Bit),
    /// 16-bit reference number concatenated short message.
    SixteenBit(ConcatenatedShortMessage16Bit),
}

/// Bytes representation of [`ConcatenatedShortMessage`] as full UDH.
#[derive(Debug)]
pub(crate) enum ConcatenatedShortMessageUdhBytes {
    /// 8-bit UDH bytes
    EightBit([u8; 6]),
    /// 16-bit UDH bytes
    SixteenBit([u8; 7]),
}

impl ConcatenatedShortMessageUdhBytes {
    /// Returns the bytes as a slice.
    pub(crate) const fn as_bytes(&self) -> &[u8] {
        match self {
            Self::EightBit(bytes) => bytes,
            Self::SixteenBit(bytes) => bytes,
        }
    }
}
