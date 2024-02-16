use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum DisplayTime {
    Temporary = 0,
    #[default]
    Default = 1,
    Invoke = 2,
    Other(u8),
}

impl From<u8> for DisplayTime {
    fn from(value: u8) -> Self {
        match value {
            0 => DisplayTime::Temporary,
            1 => DisplayTime::Default,
            2 => DisplayTime::Invoke,
            value => DisplayTime::Other(value),
        }
    }
}

impl From<DisplayTime> for u8 {
    fn from(value: DisplayTime) -> Self {
        match value {
            DisplayTime::Temporary => 0,
            DisplayTime::Default => 1,
            DisplayTime::Invoke => 2,
            DisplayTime::Other(value) => value,
        }
    }
}

impl Length for DisplayTime {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for DisplayTime {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for DisplayTime {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
