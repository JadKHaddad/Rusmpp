/// Codec for encoding and decoding `SMPP` PDUs.
///
/// # Usage
/// ```rust
/// use futures::{SinkExt, StreamExt};
/// use rusmpp::{codec::CommandCodec, Command, CommandId, CommandStatus, Pdu};
/// use tokio::io::DuplexStream;
/// use tokio_util::codec::Framed;
///
/// async fn launch_server(server_stream: DuplexStream) -> Result<(), Box<dyn core::error::Error>> {
///     tokio::spawn(async move {
///         let mut framed = Framed::new(server_stream, CommandCodec::new());
///
///         while let Some(Ok(command)) = framed.next().await {
///             if let CommandId::EnquireLink = command.id() {
///                 let response = Command::new(CommandStatus::EsmeRok, command.sequence_number, Pdu::EnquireLinkResp);
///                 framed.send(&response).await.unwrap();
///                 break;
///             }
///         }
///     });
///     Ok(())
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn core::error::Error>> {
///     let (server_stream, client_stream) = tokio::io::duplex(4096);
///     launch_server(server_stream).await?;
///
///     let mut framed = Framed::new(client_stream, CommandCodec::new());
///
///     let enquire_link_command = Command::new(CommandStatus::EsmeRok, 0, Pdu::EnquireLink);
///     framed.send(&enquire_link_command).await?;
///
///     while let Some(Ok(command)) = framed.next().await {
///         if let CommandId::EnquireLinkResp = command.id() {
///             break;
///         }
///     }
///
///     Ok(())
/// }
/// ```
#[derive(Debug)]
pub struct CommandCodec {
    max_length: Option<usize>,
}

impl CommandCodec {
    #[inline]
    pub const fn new() -> Self {
        Self { max_length: None }
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
    //! Tokio's util [`Encoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Encoder.html) and [`Decoder`](https://docs.rs/tokio-util/latest/tokio_util/codec/trait.Decoder.html)
    //! implementations for [`CommandCodec`].

    use tokio_util::{
        bytes::{Buf, BufMut, BytesMut},
        codec::{Decoder, Encoder},
    };

    use crate::{
        decode::DecodeWithLength,
        encode::{Encode, Length},
        Command,
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

            crate::debug!(target: "rusmpp::codec::encode::encoding", command=?command);
            crate::debug!(target: "rusmpp::codec::encode::encoded", encoded=?crate::utils::HexFormatter(&buf), encoded_length=command.length(), command_length);

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
        Length { actual: usize, max: usize },
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
                DecodeError::Length { actual, max } => {
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
                DecodeError::Length { .. } => None,
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
            if src.len() < 4 {
                crate::trace!(target: "rusmpp::codec::decode", source_length=src.len(), "Not enough bytes to read command length");

                return Ok(None);
            }

            let command_length = u32::from_be_bytes([src[0], src[1], src[2], src[3]]) as usize;

            crate::trace!(target: "rusmpp::codec::decode", command_length);

            if let Some(max_command_length) = self.max_length {
                if command_length > max_command_length {
                    crate::error!(target: "rusmpp::codec::decode", command_length, max_command_length, "Maximum command length exceeded");

                    return Err(DecodeError::Length {
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

            let pdu_len = command_length - 4;

            crate::debug!(target: "rusmpp::codec::decode::decoding", decoding=?crate::utils::HexFormatter(&src[..command_length]));

            let (command, _size) = match Command::decode(&src[4..command_length], pdu_len) {
                Ok((command, size)) => {
                    crate::debug!(target: "rusmpp::codec::decode::decoded", command=?command, command_length, decoded_length=size);

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
