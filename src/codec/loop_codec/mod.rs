use bytes::{Buf, BytesMut};

use crate::{
    ende::decode::{DecodeError, DecodeWithLength},
    Command,
};

#[derive(Default)]
pub struct CommandLoopCodec {
    src: BytesMut,
}

impl CommandLoopCodec {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn try_decode<R: std::io::Read>(
        &mut self,
        buffer: &mut [u8],
        mut reader: R,
    ) -> Result<Option<Command>, DecodeError> {
        match reader.read(buffer) {
            Ok(0) => Err(DecodeError::IoError(std::io::Error::new(
                std::io::ErrorKind::UnexpectedEof,
                "Unexpected EOF",
            ))),
            Err(err) => Err(DecodeError::IoError(err)),
            Ok(n) => {
                self.src.extend_from_slice(&buffer[..n]);

                #[cfg(feature = "tracing")]
                tracing::trace!(target: "rusmpp::codec::decode", read=n, source_length= self.src.len());

                if self.src.len() < 4 {
                    #[cfg(feature = "tracing")]
                    tracing::trace!(target: "rusmpp::codec::decode", source_length= self.src.len(), "Not enough data to read command_length");

                    return Ok(None);
                }

                let command_length =
                    u32::from_be_bytes([self.src[0], self.src[1], self.src[2], self.src[3]])
                        as usize;

                if self.src.len() < command_length {
                    self.src.reserve(command_length - self.src.len());

                    #[cfg(feature = "tracing")]
                    tracing::trace!(target: "rusmpp::codec::decode", %command_length, source_length= self.src.len(), "Not enough data to read the entire command");

                    return Ok(None);
                }

                let pdu_len = command_length - 4;

                #[cfg(feature = "tracing")]
                tracing::debug!(target: "rusmpp::codec::decode::decoding", decoding=?crate::utils::HexFormatter(&self.src[..command_length]));

                let command = match Command::decode_from_slice(
                    &self.src[4..command_length],
                    pdu_len,
                ) {
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

                self.src.advance(command_length);

                Ok(Some(command))
            }
        }
    }
}

#[cfg(test)]
mod tests;
