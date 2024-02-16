use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum DpfResult {
    #[default]
    NotSet = 0,
    Set = 1,
    Other(u8),
}

impl From<u8> for DpfResult {
    fn from(value: u8) -> Self {
        match value {
            0 => DpfResult::NotSet,
            1 => DpfResult::Set,
            value => DpfResult::Other(value),
        }
    }
}

impl From<DpfResult> for u8 {
    fn from(value: DpfResult) -> Self {
        match value {
            DpfResult::NotSet => 0,
            DpfResult::Set => 1,
            DpfResult::Other(value) => value,
        }
    }
}

impl Length for DpfResult {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for DpfResult {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for DpfResult {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
