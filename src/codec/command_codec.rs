use crate::{
    commands::command::Command,
    ende::{
        decode::{DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
};
use tokio_util::{
    bytes::{Buf, BufMut, BytesMut},
    codec::{Decoder, Encoder},
};

/// A codec for encoding and decoding SMPP PDUs.
///
/// Only available when the `tokio-codec` feature is enabled.
///
/// # Usage
/// ```rust
/// use futures::{SinkExt, StreamExt};
/// use rusmpp::{
///     codec::command_codec::CommandCodec,
///     commands::{
///         command::Command,
///         pdu::Pdu,
///         types::{command_id::CommandId, command_status::CommandStatus},
///     },
/// };
/// use tokio::net::TcpStream;
/// use tokio_util::codec::{FramedRead, FramedWrite};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let stream = TcpStream::connect("34.242.18.250:2775").await?;
///
///     let (reader, writer) = stream.into_split();
///     let mut framed_read = FramedRead::new(reader, CommandCodec {});
///     let mut framed_write = FramedWrite::new(writer, CommandCodec {});
///
///     let enquire_link_command = Command::new(CommandStatus::EsmeRok, 0, Pdu::EnquireLink);
///
///     framed_write.send(&enquire_link_command).await?;
///
///     while let Some(Ok(command)) = framed_read.next().await {
///         if let CommandId::EnquireLinkResp = command.command_id() {
///             break;
///         }
///     }
///
///     Ok(())
/// }
/// ```
pub struct CommandCodec;

impl Encoder<&Command> for CommandCodec {
    type Error = EncodeError;

    fn encode(&mut self, command: &Command, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let command_length = 4 + command.length();
        let encoded = tri!(command.encode_into_vec());

        dst.reserve(command_length);
        dst.put_u32(command_length as u32);
        dst.put_slice(&encoded);

        #[cfg(feature = "tracing")]
        {
            tracing::debug!(target: "rusmpp::codec::encode::encoding", command=?command);
            tracing::debug!(target: "rusmpp::codec::encode::encoded", encoded=?crate::utils::HexFormatter(&encoded));
        }

        Ok(())
    }
}

impl Decoder for CommandCodec {
    type Item = Command;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            #[cfg(feature = "tracing")]
            tracing::trace!(target: "rusmpp::codec::decode", source_length=src.len(), "Not enough data to read command_length");

            return Ok(None);
        }

        let command_length = u32::from_be_bytes([src[0], src[1], src[2], src[3]]) as usize;

        #[cfg(feature = "tracing")]
        tracing::trace!(target: "rusmpp::codec::decode", %command_length);

        if src.len() < command_length {
            // Reserve enough space to read the entire command
            src.reserve(command_length - src.len());

            #[cfg(feature = "tracing")]
            tracing::trace!(target: "rusmpp::codec::decode", %command_length, "Not enough data to read the entire command");

            return Ok(None);
        }

        let pdu_len = command_length - 4;

        #[cfg(feature = "tracing")]
        tracing::debug!(target: "rusmpp::codec::decode::decoding", decoding=?crate::utils::HexFormatter(&src[..command_length]));

        let command = match Command::decode_from_slice(&src[4..command_length], pdu_len) {
            Ok(command) => {
                #[cfg(feature = "tracing")]
                tracing::debug!(target: "rusmpp::codec::decode::decoded", command=?command);

                command
            }
            Err(err) => {
                #[cfg(feature = "tracing")]
                tracing::error!(target: "rusmpp::codec::decode", ?err);

                return Err(err);
            }
        };

        src.advance(command_length);

        Ok(Some(command))
    }
}
