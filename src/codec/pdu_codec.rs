use crate::{
    io::{
        decode::{DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    pdus::pdu::Pdu,
};
use tokio_util::{
    bytes::{Buf, BufMut, BytesMut},
    codec::{Decoder, Encoder},
};

/// A codec for encoding and decoding SMPP PDUs.
pub struct PduCodec;

impl Encoder<Pdu> for PduCodec {
    type Error = EncodeError;

    fn encode(&mut self, item: Pdu, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let command_length = 4 + item.length();

        let mut buf = Vec::with_capacity(command_length);
        item.encode_to(&mut buf)?;

        dst.reserve(command_length);
        dst.put_u32(command_length as u32);
        dst.put_slice(&buf);

        Ok(())
    }
}

impl Decoder for PduCodec {
    type Item = Pdu;
    type Error = DecodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            // Not enough data to read encoding and length marker.
            return Ok(None);
        }

        let command_length = u32::from_be_bytes([src[0], src[1], src[2], src[3]]) as usize;

        if src.len() < command_length {
            // Reserve enough space to read the entire PDU.
            src.reserve(command_length - src.len());

            // Not enough data to read the entire PDU.
            return Ok(None);
        }

        let mut slice = &src[4..command_length];
        let pdu_len = command_length - 4;

        let pdu = Pdu::decode_from(&mut slice, pdu_len)?;

        src.advance(command_length);

        Ok(Some(pdu))
    }
}
