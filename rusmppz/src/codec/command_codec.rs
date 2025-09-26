/// Codec for encoding and decoding `SMPP` PDUs.
#[derive(Debug)]
#[non_exhaustive]
pub struct CommandCodec<const N: usize> {}

impl<const N: usize> CommandCodec<N> {
    pub const fn new() -> Self {
        Self {}
    }
}

impl<const N: usize> Default for CommandCodec<N> {
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
        logging::{debug, error, trace},
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

    impl<'buf, const N: usize> Encoder<Command<'buf, N>> for CommandCodec<N> {
        type Error = EncodeError;

        fn encode(&mut self, item: Command<'buf, N>, dst: &mut [u8]) -> Result<usize, Self::Error> {
            let command_length = 4 + item.length();

            if dst.len() < command_length {
                return Err(EncodeError::BufferTooSmall);
            }

            dst[..4].copy_from_slice(&(command_length as u32).to_be_bytes());
            let _ = item.encode(&mut dst[4..command_length]);

            debug!(target: "rusmppz::codec::encode", command=?item, "Encoding");
            debug!(target: "rusmppz::codec::encode", encoded=?crate::logging::Formatter(&dst[..command_length]), encoded_length=item.length(), command_length, "Encoded");

            Ok(command_length)
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

    impl<const N: usize> framez::decode::DecodeError for CommandCodec<N> {
        type Error = DecodeError;
    }

    impl<'buf, const N: usize> Decoder<'buf> for CommandCodec<N> {
        type Item = Command<'buf, N>;

        fn decode(
            &mut self,
            src: &'buf mut [u8],
        ) -> Result<Option<(Self::Item, usize)>, Self::Error> {
            const HEADER_LENGTH: usize = 16;

            if src.len() < HEADER_LENGTH {
                trace!(target: "rusmppz::codec::decode", source_length=src.len(), "Not enough bytes to read the header");

                return Ok(None);
            }

            let command_length =
                usize::try_from(u32::from_be_bytes([src[0], src[1], src[2], src[3]]))
                    .inspect_err(|_err| {
                        error!(target: "rusmppz::codec::decode", err=?_err, "Failed to convert command length to usize");
                    })
                    .map_err(DecodeError::InvalidLength)?;

            trace!(target: "rusmppz::codec::decode", command_length);

            if command_length < HEADER_LENGTH {
                error!(target: "rusmppz::codec::decode", command_length, min_command_length=HEADER_LENGTH, "Minimum command length not met");

                return Err(DecodeError::MinLength {
                    actual: command_length,
                    min: HEADER_LENGTH,
                });
            }

            if src.len() < command_length {
                trace!(target: "rusmppz::codec::decode", command_length, "Not enough bytes to read the entire command");

                return Ok(None);
            }

            // command_length is at least 16 bytes
            let pdu_len = command_length - 4;

            debug!(target: "rusmppz::codec::decode", decoding=?crate::logging::Formatter(&src[..command_length]), "Decoding");

            let (command, _size) = match Command::decode(&src[4..command_length], pdu_len) {
                Ok((command, size)) => {
                    debug!(target: "rusmppz::codec::decode", command=?command, command_length, decoded_length=size, "Decoded");

                    (command, size)
                }
                Err(err) => {
                    error!(target: "rusmppz::codec::decode", ?err);

                    return Err(DecodeError::Decode(err));
                }
            };

            Ok(Some((command, command_length)))
        }
    }
}
