use crate::{
    ende::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ItsSessionInfo {
    pub session_number: u8,
    pub sequence_number: u8,
}

impl ItsSessionInfo {
    pub fn new(session_number: u8, sequence_number: u8) -> Self {
        Self {
            session_number,
            sequence_number,
        }
    }
}

impl Length for ItsSessionInfo {
    fn length(&self) -> usize {
        self.session_number.length() + self.sequence_number.length()
    }
}

impl Encode for ItsSessionInfo {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.session_number.encode_to(writer));
        tri!(self.sequence_number.encode_to(writer));

        Ok(())
    }
}

impl Decode for ItsSessionInfo {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let session_number = tri!(u8::decode_from(reader));
        let sequence_number = tri!(u8::decode_from(reader));

        Ok(Self {
            session_number,
            sequence_number,
        })
    }
}
