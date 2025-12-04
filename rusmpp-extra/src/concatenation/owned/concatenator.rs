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
    /// * `encoded` - The full encoded message as a byte vector.
    /// * `max_message_size` - The maximum size of each message part. Max is `255` so no invalid `OctetStrings` are created.
    /// * `part_header_size` - The size of the header for each part.
    fn concatenate(
        &self,
        encoded: alloc::vec::Vec<u8>,
        max_message_size: u8,
        part_header_size: u8,
    ) -> Result<
        Concatenation<impl Iterator<Item = OctetString<0, 255>> + '_>,
        <Self as Concatenator>::Error,
    >;
}
