/// Codec for encoding and decoding `SMPP` PDUs.
#[derive(Debug)]
pub struct CommandCodec {
    max_length: Option<usize>,
}

impl CommandCodec {
    /// Creates a new [`CommandCodec`] with a default maximum length of `8192` bytes.
    #[inline]
    pub const fn new() -> Self {
        Self {
            max_length: Some(8192),
        }
    }

    #[inline]
    pub const fn max_length(&self) -> Option<usize> {
        self.max_length
    }

    #[inline]
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }

    #[inline]
    pub fn without_max_length(mut self) -> Self {
        self.max_length = None;
        self
    }
}

impl Default for CommandCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "tokio-codec")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-codec")))]
pub mod tokio {
    //! Tokio's util [`Encoder`] and [`Decoder`] implementations for [`CommandCodec`].

    use tokio_util::{
        bytes::{Buf, BufMut, BytesMut},
        codec::{Decoder, Encoder},
    };

    use crate::{
        Command,
        decode::DecodeWithLength,
        encode::{Encode, Length},
    };

    use super::CommandCodec;

    #[derive(Debug)]
    #[non_exhaustive]
    pub enum EncodeError {
        Io(std::io::Error),
    }

    impl From<std::io::Error> for EncodeError {
        fn from(e: std::io::Error) -> Self {
            EncodeError::Io(e)
        }
    }

    impl core::fmt::Display for EncodeError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                EncodeError::Io(e) => write!(f, "I/O error: {e}"),
            }
        }
    }

    impl core::error::Error for EncodeError {
        fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
            match self {
                EncodeError::Io(e) => Some(e),
            }
        }

        fn cause(&self) -> Option<&dyn core::error::Error> {
            self.source()
        }
    }

    impl Encoder<&Command> for CommandCodec {
        type Error = EncodeError;

        fn encode(&mut self, command: &Command, dst: &mut BytesMut) -> Result<(), Self::Error> {
            let command_length = 4 + command.length();

            dst.reserve(command_length);
            dst.put_u32(command_length as u32);

            // TODO: Can we encode directly into dst?
            let mut buf = alloc::vec![0; command.length()];
            let _ = command.encode(buf.as_mut_slice());

            dst.put_slice(&buf);

            crate::debug!(target: "rusmpp::codec::encode", command=?command, "Encoding");
            crate::debug!(target: "rusmpp::codec::encode", encoded=?crate::utils::HexFormatter(&buf), encoded_length=command.length(), command_length, "Encoded");

            Ok(())
        }
    }

    impl Encoder<Command> for CommandCodec {
        type Error = EncodeError;

        fn encode(&mut self, command: Command, dst: &mut BytesMut) -> Result<(), Self::Error> {
            self.encode(&command, dst)
        }
    }

    #[derive(Debug)]
    #[non_exhaustive]
    pub enum DecodeError {
        Io(std::io::Error),
        Decode(crate::decode::DecodeError),
        MinLength { actual: usize, min: usize },
        MaxLength { actual: usize, max: usize },
    }

    impl From<std::io::Error> for DecodeError {
        fn from(e: std::io::Error) -> Self {
            DecodeError::Io(e)
        }
    }

    impl core::fmt::Display for DecodeError {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                DecodeError::Io(e) => write!(f, "I/O error: {e}"),
                DecodeError::Decode(e) => write!(f, "Decode error: {e}"),
                DecodeError::MinLength { actual, min } => {
                    write!(
                        f,
                        "Minimum command length not met. actual: {actual}, min: {min}"
                    )
                }
                DecodeError::MaxLength { actual, max } => {
                    write!(
                        f,
                        "Maximum command length exceeded. actual: {actual}, max: {max}"
                    )
                }
            }
        }
    }

    impl core::error::Error for DecodeError {
        fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
            match self {
                DecodeError::Io(e) => Some(e),
                DecodeError::Decode(e) => Some(e),
                DecodeError::MinLength { .. } => None,
                DecodeError::MaxLength { .. } => None,
            }
        }

        fn cause(&self) -> Option<&dyn core::error::Error> {
            self.source()
        }
    }

    impl Decoder for CommandCodec {
        type Item = Command;
        type Error = DecodeError;

        fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
            const HEADER_LENGTH: usize = 16;

            if src.len() < HEADER_LENGTH {
                crate::trace!(target: "rusmpp::codec::decode", source_length=src.len(), "Not enough bytes to read the header");

                return Ok(None);
            }

            let command_length = u32::from_be_bytes([src[0], src[1], src[2], src[3]]) as usize;

            crate::trace!(target: "rusmpp::codec::decode", command_length);

            if command_length < HEADER_LENGTH {
                crate::error!(target: "rusmpp::codec::decode", command_length, min_command_length=HEADER_LENGTH, "Minimum command length not met");

                return Err(DecodeError::MinLength {
                    actual: command_length,
                    min: HEADER_LENGTH,
                });
            }

            if let Some(max_command_length) = self.max_length {
                if command_length > max_command_length {
                    crate::error!(target: "rusmpp::codec::decode", command_length, max_command_length, "Maximum command length exceeded");

                    return Err(DecodeError::MaxLength {
                        actual: command_length,
                        max: max_command_length,
                    });
                }
            }

            if src.len() < command_length {
                // Reserve enough space to read the entire command
                src.reserve(command_length - src.len());

                crate::trace!(target: "rusmpp::codec::decode", command_length, "Not enough bytes to read the entire command");

                return Ok(None);
            }

            // command_length is at least 16 bytes
            let pdu_len = command_length - 4;

            crate::debug!(target: "rusmpp::codec::decode", decoding=?crate::utils::HexFormatter(&src[..command_length]), "Decoding");

            let (command, _size) = match Command::decode(&src[4..command_length], pdu_len) {
                Ok((command, size)) => {
                    crate::debug!(target: "rusmpp::codec::decode", command=?command, command_length, decoded_length=size, "Decoded");

                    (command, size)
                }
                Err(err) => {
                    crate::error!(target: "rusmpp::codec::decode", ?err);

                    return Err(DecodeError::Decode(err));
                }
            };

            src.advance(command_length);

            Ok(Some(command))
        }
    }
}
