use crate::io::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Ton {
    #[default]
    Unknown = 0b00000000,
    International = 0b00000001,
    National = 0b00000010,
    NetworkSpecific = 0b00000011,
    SubscriberNumber = 0b00000100,
    Alphanumeric = 0b00000101,
    Abbreviated = 0b00000110,
    Other(u8),
}

impl From<u8> for Ton {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => Ton::Unknown,
            0b00000001 => Ton::International,
            0b00000010 => Ton::National,
            0b00000011 => Ton::NetworkSpecific,
            0b00000100 => Ton::SubscriberNumber,
            0b00000101 => Ton::Alphanumeric,
            0b00000110 => Ton::Abbreviated,
            value => Ton::Other(value),
        }
    }
}

impl From<Ton> for u8 {
    fn from(value: Ton) -> Self {
        match value {
            Ton::Unknown => 0b00000000,
            Ton::International => 0b00000001,
            Ton::National => 0b00000010,
            Ton::NetworkSpecific => 0b00000011,
            Ton::SubscriberNumber => 0b00000100,
            Ton::Alphanumeric => 0b00000101,
            Ton::Abbreviated => 0b00000110,
            Ton::Other(value) => value,
        }
    }
}

impl Length for Ton {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for Ton {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for Ton {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
