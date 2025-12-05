use crate::codecs::gsm::alphabet::Gsm7BitAlphabet;

/// GSM 7-bit unpacked codec.
#[non_exhaustive]
#[derive(Debug)]
pub struct Gsm7BitUnpacked {
    /// The GSM 7-bit alphabet to use for encoding.
    alphabet: Gsm7BitAlphabet,
    /// Whether to allow splitting extended characters across message parts.
    allow_split_extended_character: bool,
}

impl Default for Gsm7BitUnpacked {
    fn default() -> Self {
        Self::new()
    }
}

impl Gsm7BitUnpacked {
    /// Creates a new [`Gsm7BitUnpacked`] with [`Gsm7BitAlphabet::Default`].
    ///
    /// # Defaults
    ///
    /// - `alphabet`: [`Gsm7BitAlphabet::Default`]
    /// - `allow_split_extended_character`: `false`
    pub const fn new() -> Self {
        Self {
            alphabet: Gsm7BitAlphabet::default(),
            allow_split_extended_character: false,
        }
    }

    /// Sets the alphabet for the codec.
    pub const fn with_alphabet(mut self, alphabet: Gsm7BitAlphabet) -> Self {
        self.alphabet = alphabet;
        self
    }

    /// Returns whether splitting extended characters is allowed.
    pub const fn allow_split_extended_character(&self) -> bool {
        self.allow_split_extended_character
    }

    /// Sets whether to allow splitting extended characters across message parts.
    pub const fn with_allow_split_extended_character(mut self, allow: bool) -> Self {
        self.allow_split_extended_character = allow;
        self
    }
}

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
mod impl_owned {
    use alloc::vec::Vec;

    use rusmpp_core::{types::owned::OctetString, values::DataCoding};

    use crate::{
        codecs::{
            gsm::errors::{Gsm7BitConcatenateError, Gsm7BitEncodeError},
            owned::Encoder,
        },
        concatenation::owned::{Concatenation, Concatenator},
    };

    use super::*;

    impl Gsm7BitUnpacked {
        /// Encodes the given message into a vector of bytes.
        pub fn encode_to_vec(&self, message: &str) -> Result<Vec<u8>, Gsm7BitEncodeError> {
            self.alphabet
                .encode_to_vec(message)
                .map_err(Gsm7BitEncodeError::unencodable_character)
        }
    }

    impl Encoder for Gsm7BitUnpacked {
        type Error = Gsm7BitEncodeError;

        fn data_coding(&self) -> DataCoding {
            DataCoding::McSpecific
        }

        fn encode(&self, message: &str) -> Result<Vec<u8>, Self::Error> {
            self.encode_to_vec(message)
        }
    }

    impl Concatenator for Gsm7BitUnpacked {
        type Error = Gsm7BitConcatenateError;

        fn concatenate(
            &self,
            encoded: Vec<u8>,
            max_message_size: u8,
            part_header_size: u8,
        ) -> Result<
            Concatenation<impl Iterator<Item = OctetString<0, 255>> + '_>,
            <Self as Concatenator>::Error,
        > {
            /// Iterator for concatenated message parts.
            ///
            /// # Note
            ///
            /// Must never create invalid parts while iterating. This is done by
            ///
            /// - Early checks for `part_payload_size < 2 && !allow_split_extended_character` in [`Gsm7Bit::concatenate`]:
            ///   this removes the possibility of creating invalid parts while iterating.
            /// - `part_payload_size` is `u8` in [`Gsm7Bit::concatenate`]:
            ///   this ensures that created parts can never exceed `255` bytes.
            struct ConcatenationIter {
                encoded: Vec<u8>,
                allow_split_extended_character: bool,
                part_payload_size: usize,
                pos: usize,
            }

            impl Iterator for ConcatenationIter {
                type Item = Vec<u8>;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.pos >= self.encoded.len() {
                        return None;
                    }

                    let total = self.encoded.len();
                    let mut end = (self.pos + self.part_payload_size).min(total);

                    // avoid splitting extended characters unless `allow_split_extended_character == true`
                    if !self.allow_split_extended_character
                        && end < total
                        && self.encoded[end - 1] == 0x1B
                    {
                        end -= 1;

                        // We made sure that `end == i` would never happen because we checked for part_payload_size < 2 earlier.
                    }

                    let chunk = &self.encoded[self.pos..end];
                    self.pos = end;

                    Some(chunk.to_vec())
                }
            }

            let total = encoded.len();

            if total <= max_message_size as usize {
                return Ok(Concatenation::single(OctetString::new(encoded).expect(
                    "encoded.len() <= max_message_size (u8), which can not be greater than 255",
                )));
            }

            let part_payload_size: u8 = max_message_size.saturating_sub(part_header_size);

            if part_payload_size == 0 {
                return Err(Gsm7BitConcatenateError::PartCapacityExceeded);
            }

            // This early check removes the possibility of creating invalid parts in the iterator.
            // The iterator must never create invalid parts while iterating.
            if part_payload_size < 2 && !self.allow_split_extended_character {
                return Err(Gsm7BitConcatenateError::InvalidBoundary);
            }

            let iter = ConcatenationIter {
                encoded,
                allow_split_extended_character: self.allow_split_extended_character,
                part_payload_size: part_payload_size as usize,
                pos: 0,
            }
            .map(|bytes| {
                OctetString::<0, 255>::new(bytes)
                    .expect("part_payload_size (u8) can not be greater than 255")
            });

            Ok(Concatenation::concatenated(iter))
        }
    }
}
