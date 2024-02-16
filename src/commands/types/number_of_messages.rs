use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NumberOfMessages {
    Allowed(u8),
    Other(u8),
}

impl From<u8> for NumberOfMessages {
    fn from(value: u8) -> Self {
        match value {
            0..=99 => NumberOfMessages::Allowed(value),
            _ => NumberOfMessages::Other(value),
        }
    }
}

impl From<NumberOfMessages> for u8 {
    fn from(value: NumberOfMessages) -> Self {
        match value {
            NumberOfMessages::Allowed(value) => value,
            NumberOfMessages::Other(value) => value,
        }
    }
}

impl Length for NumberOfMessages {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for NumberOfMessages {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for NumberOfMessages {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
