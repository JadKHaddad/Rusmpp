use crate::{codecs::owned::Encoder, concatenation::owned::concatenation::Concatenation};

/// A trait for concatenating messages into smaller parts.
pub trait Concatenator: Encoder {
    /// The type of errors that can occur during concatenation.
    type Error;

    /// Splits the encoded message into concatenated parts.
    ///
    /// # Arguments
    ///
    /// * `encoded` - The full encoded message as a byte vector.
    /// * `max_message_size` - The maximum size of each message part.
    /// * `part_header_size` - The size of the header for each part.
    ///
    /// # Note
    ///
    /// The returned `Vec<u8>` in the `Concatenation` must *`NOT`* exceed `max_message_size - part_header_size` in length.
    ///
    /// `max_message_size` and `part_header_size` are defined as `u8` to never exceed 255, which is the maximum size of an SMS message.
    fn concatenate(
        &self,
        encoded: alloc::vec::Vec<u8>,
        max_message_size: u8,
        part_header_size: u8,
    ) -> Result<
        Concatenation<impl Iterator<Item = alloc::vec::Vec<u8>> + '_>,
        <Self as Concatenator>::Error,
    >;
}
