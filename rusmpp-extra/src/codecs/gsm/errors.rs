use crate::{codecs::errors::UnencodableCharacterError, concatenation::owned::Concatenation};

/// Errors that can occur during GSM 7-bit encoding.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Gsm7BitEncodeError {
    /// Input contains un-encodable character.
    #[error(transparent)]
    UnencodableCharacter(#[from] UnencodableCharacterError),
}

impl Gsm7BitEncodeError {
    pub(crate) const fn unencodable_character(character: char) -> Self {
        Self::UnencodableCharacter(UnencodableCharacterError::new(character))
    }
}

/// Errors that can occur during GSM 7-bit concatenation.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Gsm7BitConcatenateError {
    /// Encoding error.
    #[error("Encoding error: {0}")]
    Encode(
        #[from]
        #[source]
        Gsm7BitEncodeError,
    ),
    /// Part cannot fit even a single septet.
    ///
    /// This error is returned when `max_message_size - part_header_size == 0`.
    #[error(
        "Cannot fit even a single septet into a part with the given header and size constraints"
    )]
    PartCapacityExceeded,
    /// A part would end with an escape (0x1B) septet, which is not allowed unless allow_split_extended_character=true.
    ///
    /// This error is returned when `max_message_size - part_header_size < 2 && allow_split_extended_character == false`.
    #[error(
        "A part would end with an escape (0x1B) septet, which is not allowed unless allow_split_extended_character=true"
    )]
    InvalidBoundary,
    #[error("The number of parts exceeds the maximum allowed. actual: {actual} > max: {max}")]
    /// The number of parts exceeds the maximum allowed.
    PartsCountExceeded {
        /// The maximum allowed number of parts.
        max: usize,
        /// The actual number of parts.
        actual: usize,
    },
}

impl Gsm7BitConcatenateError {
    pub(crate) const fn parts_count_exceeded(actual: usize) -> Self {
        Self::PartsCountExceeded {
            max: Concatenation::MAX_PARTS,
            actual,
        }
    }
}
