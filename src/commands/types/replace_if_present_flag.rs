use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum ReplaceIfPresentFlag {
    #[default]
    DontReplace = 0,
    Replace = 1,
    Other(u8),
}

impl From<u8> for ReplaceIfPresentFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => ReplaceIfPresentFlag::DontReplace,
            1 => ReplaceIfPresentFlag::Replace,
            value => ReplaceIfPresentFlag::Other(value),
        }
    }
}

impl From<ReplaceIfPresentFlag> for u8 {
    fn from(value: ReplaceIfPresentFlag) -> Self {
        match value {
            ReplaceIfPresentFlag::DontReplace => 0,
            ReplaceIfPresentFlag::Replace => 1,
            ReplaceIfPresentFlag::Other(value) => value,
        }
    }
}

impl Length for ReplaceIfPresentFlag {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for ReplaceIfPresentFlag {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for ReplaceIfPresentFlag {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
