use core::num::NonZeroUsize;

use crate::values::{ConcatenatedShortMessageType, DataCoding};

pub trait Encoder<T> {
    /// The associated error type for encoding operations.
    type Error;

    /// Encodes the input value into a new `Vec<u8>`.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<u8>)` with the encoded bytes.
    /// - `Err(Self::Error)` if an encoding error occurs.
    fn encode(&self, value: T) -> Result<alloc::vec::Vec<u8>, Self::Error>;

    /// The corresponding data coding for the encoded value.
    fn data_coding(&self) -> DataCoding;

    /// Max bytes for no concatenation, after encoding.
    ///
    /// # Note
    ///
    /// `max_bytes` must not exceed `255`: the maximum length of an SMPP short message.
    /// See [`SubmitSm::short_message`](crate::pdus::owned::SubmitSm::short_message).
    fn max_bytes(&self) -> NonZeroUsize;

    /// Max bytes for concatenation, after encoding.
    ///
    /// # Note
    ///
    /// `max_bytes_with_concatenation` + [`ConcatenatedShortMessageType::udh_length`] must not exceed `255`: the maximum length of an SMPP short message.
    /// See [`SubmitSm::short_message`](crate::pdus::owned::SubmitSm::short_message).
    fn max_bytes_with_concatenation(
        &self,
        concatenation: ConcatenatedShortMessageType,
    ) -> NonZeroUsize;
}
