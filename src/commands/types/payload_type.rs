use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum PayloadType {
    #[default]
    Default = 0,
    WcmpMessage = 1,
    Other(u8),
}

impl From<u8> for PayloadType {
    fn from(value: u8) -> Self {
        match value {
            0 => PayloadType::Default,
            1 => PayloadType::WcmpMessage,
            value => PayloadType::Other(value),
        }
    }
}

impl From<PayloadType> for u8 {
    fn from(value: PayloadType) -> Self {
        match value {
            PayloadType::Default => 0,
            PayloadType::WcmpMessage => 1,
            PayloadType::Other(value) => value,
        }
    }
}

impl Length for PayloadType {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for PayloadType {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for PayloadType {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
