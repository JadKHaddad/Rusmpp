/// Codec for encoding and decoding `SMPP` PDUs.
#[derive(Debug)]
#[non_exhaustive]
pub struct CommandCodec {}

impl CommandCodec {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for CommandCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "framez")]
#[cfg_attr(docsrs, doc(cfg(feature = "framez")))]
pub mod framez {
    //! Framez [`Encoder`] and [`Decoder`] implementations for [`CommandCodec`].

    use core::num::TryFromIntError;

    use framez::{decode::Decoder, encode::Encoder};

    use crate::{
        Command,
        decode::DecodeWithLength,
        encode::{Encode, Length},
    };

    use super::CommandCodec;

    /// An error that can occur when encoding a [`Command`].
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum EncodeError {
        /// The input buffer is too small to fit the encoded [`Command`].
        BufferTooSmall,
    }

    impl core::fmt::Display for EncodeError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::BufferTooSmall => write!(f, "buffer too small"),
            }
        }
    }

    impl core::error::Error for EncodeError {}

    impl<'buf> Encoder<Command<'buf>> for CommandCodec {
        type Error = EncodeError;

        fn encode(&mut self, item: Command<'buf>, dst: &mut [u8]) -> Result<usize, Self::Error> {
            let size = item.length();

            if dst.len() < size {
                return Err(EncodeError::BufferTooSmall);
            }

            let _ = item.encode(&mut dst[..size]);

            Ok(size)
        }
    }

    /// An error that can occur when decoding a [`Command`].
    #[derive(Debug)]
    #[non_exhaustive]
    pub enum DecodeError {
        /// Decode error.
        Decode(crate::decode::DecodeError),
        /// Minimum command length not met.
        MinLength { actual: usize, min: usize },
        /// Integral type conversion failed.
        InvalidLength(TryFromIntError),
    }

    impl core::fmt::Display for DecodeError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                DecodeError::Decode(e) => write!(f, "Decode error: {e}"),
                DecodeError::MinLength { actual, min } => {
                    write!(
                        f,
                        "Minimum command length not met. actual: {actual}, min: {min}"
                    )
                }

                DecodeError::InvalidLength(e) => {
                    write!(f, "Integral type conversion failed: {e}")
                }
            }
        }
    }

    impl core::error::Error for DecodeError {}

    impl framez::decode::DecodeError for CommandCodec {
        type Error = DecodeError;
    }

    impl<'buf> Decoder<'buf> for CommandCodec {
        type Item = Command<'buf>;

        fn decode(
            &mut self,
            src: &'buf mut [u8],
        ) -> Result<Option<(Self::Item, usize)>, Self::Error> {
            const HEADER_LENGTH: usize = 16;

            if src.len() < HEADER_LENGTH {
                // Not enough bytes to read the header

                return Ok(None);
            }

            let command_length =
                usize::try_from(u32::from_be_bytes([src[0], src[1], src[2], src[3]]))
                    .map_err(DecodeError::InvalidLength)?;

            if command_length < HEADER_LENGTH {
                // Minimum command length not met

                return Err(DecodeError::MinLength {
                    actual: command_length,
                    min: HEADER_LENGTH,
                });
            }

            if src.len() < command_length {
                // Not enough bytes to read the entire command

                return Ok(None);
            }

            // command_length is at least 16 bytes
            let pdu_len = command_length - 4;

            let (command, _size) = match Command::decode(&src[4..command_length], pdu_len) {
                Ok((command, size)) => (command, size),
                Err(err) => {
                    return Err(DecodeError::Decode(err));
                }
            };

            Ok(Some((command, command_length)))
        }
    }
}
