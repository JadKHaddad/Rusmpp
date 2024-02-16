use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum BroadcastAreaSuccess {
    #[default]
    InformationNotAvailable,
    ZeroToHundred(u8),
    Other(u8),
}

impl From<BroadcastAreaSuccess> for u8 {
    fn from(value: BroadcastAreaSuccess) -> Self {
        match value {
            BroadcastAreaSuccess::InformationNotAvailable => 255,
            BroadcastAreaSuccess::ZeroToHundred(value) => value,
            BroadcastAreaSuccess::Other(value) => value,
        }
    }
}

impl From<u8> for BroadcastAreaSuccess {
    fn from(value: u8) -> Self {
        match value {
            0..=100 => Self::ZeroToHundred(value),
            255 => Self::InformationNotAvailable,
            _ => Self::Other(value),
        }
    }
}

impl Length for BroadcastAreaSuccess {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for BroadcastAreaSuccess {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for BroadcastAreaSuccess {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
