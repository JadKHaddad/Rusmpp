use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum DestAddrNpResolution {
    #[default]
    QueryNotPerformed = 0,
    QueryPerformedNumberNotPorted = 1,
    QueryPerformedNumberPorted = 2,
    Other(u8),
}

impl From<u8> for DestAddrNpResolution {
    fn from(value: u8) -> Self {
        match value {
            0 => DestAddrNpResolution::QueryNotPerformed,
            1 => DestAddrNpResolution::QueryPerformedNumberNotPorted,
            2 => DestAddrNpResolution::QueryPerformedNumberPorted,
            value => DestAddrNpResolution::Other(value),
        }
    }
}

impl From<DestAddrNpResolution> for u8 {
    fn from(value: DestAddrNpResolution) -> Self {
        match value {
            DestAddrNpResolution::QueryNotPerformed => 0,
            DestAddrNpResolution::QueryPerformedNumberNotPorted => 1,
            DestAddrNpResolution::QueryPerformedNumberPorted => 2,
            DestAddrNpResolution::Other(value) => value,
        }
    }
}

impl Length for DestAddrNpResolution {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for DestAddrNpResolution {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for DestAddrNpResolution {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
