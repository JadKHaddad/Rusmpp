use crate::{
    io::traits::{decode::DecodeError, encode::EncodeError},
    pdus::pdu::{PduIn, PduOut},
};
use tokio_util::{
    bytes::BytesMut,
    codec::{Decoder, Encoder},
};

pub struct PduCodec;

impl Encoder<PduOut> for PduCodec {
    type Error = DecodeError;

    fn encode(&mut self, item: PduOut, dst: &mut BytesMut) -> Result<(), Self::Error> {
        todo!()
    }
}

impl Decoder for PduCodec {
    type Item = PduIn;
    type Error = EncodeError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        todo!()
    }
}
