use rusmpp_core::types::owned::OctetString;

use crate::{codecs::owned::Encoder, concatenation::owned::concatenation::Concatenation};

/// A trait for concatenating messages into smaller parts.
pub trait Concatenator: Encoder {
    /// The type of errors that can occur during concatenation.
    type Error;

    /// Splits the encoded message into concatenated parts.
    ///
    /// # Arguments
    ///
    /// * `message` - The full message as a string slice.
    /// * `max_message_size` - The maximum size of each message part.
    /// * `part_header_size` - The size of the header for each part.
    fn concatenate(
        &self,
        message: &str,
        max_message_size: usize,
        part_header_size: usize,
    ) -> Result<
        Concatenation<impl Iterator<Item = OctetString<0, 255>> + '_>,
        <Self as Concatenator>::Error,
    >;
}
