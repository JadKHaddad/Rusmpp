use crate::{
    commands::command::Command,
    ende::{
        decode::{DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
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
/// use tokio::net::TcpStream;
/// use tokio_util::codec::{FramedRead, FramedWrite};
/// use rusmpp::{
///    codec::command_codec::CommandCodec,
///    commands::{
///        command::Command,
///        pdu::{Pdu},
///        types::{
///           command_status::CommandStatus, command_id::CommandId,
///          },
///     },
/// };
///
/// #[tokio::main]
/// async fn main() {
///     let stream = TcpStream::connect("34.242.18.250:2775")
///         .await
///         .expect("Failed to connect");
///
///     let (reader, writer) = stream.into_split();
///     let mut framed_read = FramedRead::new(reader, CommandCodec {});
///     let mut framed_write = FramedWrite::new(writer, CommandCodec {});
///
///     let enquire_link_command = Command::builder()
///         .command_status(CommandStatus::EsmeRok)
///         .sequence_number(0)
///         .pdu(Pdu::EnquireLink)
///         .build();
///
///     framed_write
///         .send(enquire_link_command)
///         .await
///         .expect("Failed to send PDU");
///
///     while let Some(Ok(command)) = framed_read.next().await {
///         if let CommandId::EnquireLinkResp = command.command_id() {
///             break;
///         }
///     }
/// }
/// ```
pub struct CommandCodec;

impl Encoder<Command> for CommandCodec {
    type Error = EncodeError;

    fn encode(&mut self, item: Command, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let command_length = 4 + item.length();

        dst.reserve(command_length);

        dst.put_u32(command_length as u32);
        dst.put_slice(&item.encode_into_vec()?);

        Ok(())
    }
}

impl Decoder for CommandCodec {
    type Item = Command;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            // Not enough data to read command_length
            return Ok(None);
        }

        let command_length = u32::from_be_bytes([src[0], src[1], src[2], src[3]]) as usize;

        if src.len() < command_length {
            // Reserve enough space to read the entire command
            src.reserve(command_length - src.len());

            // Not enough data to read the entire command
            return Ok(None);
        }

        let pdu_len = command_length - 4;
        let command = Command::decode_from_slice(&src[4..command_length], pdu_len)?;

        src.advance(command_length);

        Ok(Some(command))
    }
}
