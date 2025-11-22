use crate::udhs::{ConcatenatedShortMessage8Bit, ConcatenatedShortMessage16Bit};

/// Concatenated short message.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConcatenatedShortMessage {
    /// 8-bit reference number concatenated short message.
    EightBit(ConcatenatedShortMessage8Bit),
    /// 16-bit reference number concatenated short message.
    SixteenBit(ConcatenatedShortMessage16Bit),
}

impl ConcatenatedShortMessage {
    /// Very similar to [`ConcatenatedShortMessageType::udh_length`], but used elsewhere.
    ///
    /// See [`ConcatenatedShortMessageType::udh_length`].
    pub(crate) const fn udh_length(&self) -> usize {
        match self {
            Self::EightBit(_) => ConcatenatedShortMessage8Bit::UDH_LENGTH,
            Self::SixteenBit(_) => ConcatenatedShortMessage16Bit::UDH_LENGTH,
        }
    }

    /// Converts [`ConcatenatedShortMessage`] to its UDH bytes representation.
    pub(crate) const fn udh_bytes(&self) -> ConcatenatedShortMessageUdhBytes {
        match self {
            Self::EightBit(concatenation) => {
                ConcatenatedShortMessageUdhBytes::EightBit(concatenation.udh_bytes())
            }
            Self::SixteenBit(concatenation) => {
                ConcatenatedShortMessageUdhBytes::SixteenBit(concatenation.udh_bytes())
            }
        }
    }
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

/// Concatenated short message type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ConcatenatedShortMessageType {
    /// 8-bit reference number concatenated short message.
    EightBit { reference: u8 },
    /// 16-bit reference number concatenated short message.
    SixteenBit { reference: u16 },
}

impl ConcatenatedShortMessageType {
    pub(crate) const fn u8(reference: u8) -> Self {
        Self::EightBit { reference }
    }

    pub(crate) const fn u16(reference: u16) -> Self {
        Self::SixteenBit { reference }
    }

    /// Returns the length of the UDH type in bytes.
    // XXX: Codecs rely on this value to be correct. Using bad (very large) values will cause underflow.
    pub(crate) const fn udh_length(self) -> usize {
        match self {
            Self::EightBit { .. } => ConcatenatedShortMessage8Bit::UDH_LENGTH,
            Self::SixteenBit { .. } => ConcatenatedShortMessage16Bit::UDH_LENGTH,
        }
    }

    pub(crate) const fn concatenated_short_message_unchecked(
        self,
        total_parts: u8,
        part_number: u8,
    ) -> ConcatenatedShortMessage {
        match self {
            Self::EightBit { reference } => ConcatenatedShortMessage::EightBit(
                ConcatenatedShortMessage8Bit::new_unchecked(reference, total_parts, part_number),
            ),
            Self::SixteenBit { reference } => ConcatenatedShortMessage::SixteenBit(
                ConcatenatedShortMessage16Bit::new_unchecked(reference, total_parts, part_number),
            ),
        }
    }
}
