use rusmpp_core::types::OctetStringError;

use crate::concatenation::owned::Concatenation;

#[derive(Debug, thiserror::Error)]
pub enum MultipartError<E, C> {
    #[error("Encode error: {0}")]
    Encode(E),
    #[error("Concatenation error: {0}")]
    Concatenation(C),
    #[error("Encoder produced invalid short message: {0}")]
    ShortMessage(
        #[from]
        #[source]
        OctetStringError,
    ),
    #[error("The number of parts is less than the minimum required. actual: {actual}, min: {min}")]
    MinPartCount {
        /// The minimum required number of parts.
        min: usize,
        /// The actual number of parts.
        actual: usize,
    },
    #[error("The number of parts exceeds the maximum allowed. actual: {actual}, max: {max}")]
    MaxPartsCount {
        /// The maximum allowed number of parts.
        max: usize,
        /// The actual number of parts.
        actual: usize,
    },
}

impl<E, C> MultipartError<E, C> {
    pub(crate) const fn encode(error: E) -> Self {
        Self::Encode(error)
    }

    pub(crate) const fn concatenation(error: C) -> Self {
        Self::Concatenation(error)
    }

    pub(crate) const fn min_part_count(actual: usize) -> Self {
        Self::MinPartCount {
            min: Concatenation::MIN_PARTS,
            actual,
        }
    }

    pub(crate) const fn max_parts_count(actual: usize) -> Self {
        Self::MaxPartsCount {
            max: Concatenation::MAX_PARTS,
            actual,
        }
    }
}
