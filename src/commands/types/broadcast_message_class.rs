use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum BroadcastMessageClass {
    #[default]
    NoClassSpecified = 0x00,
    Class1 = 0x01,
    Class2 = 0x02,
    Class3 = 0x03,
    Other(u8),
}

impl From<u8> for BroadcastMessageClass {
    fn from(value: u8) -> Self {
        match value {
            0x00 => BroadcastMessageClass::NoClassSpecified,
            0x01 => BroadcastMessageClass::Class1,
            0x02 => BroadcastMessageClass::Class2,
            0x03 => BroadcastMessageClass::Class3,
            value => BroadcastMessageClass::Other(value),
        }
    }
}

impl From<BroadcastMessageClass> for u8 {
    fn from(value: BroadcastMessageClass) -> Self {
        match value {
            BroadcastMessageClass::NoClassSpecified => 0x00,
            BroadcastMessageClass::Class1 => 0x01,
            BroadcastMessageClass::Class2 => 0x02,
            BroadcastMessageClass::Class3 => 0x03,
            BroadcastMessageClass::Other(value) => value,
        }
    }
}

impl Length for BroadcastMessageClass {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for BroadcastMessageClass {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for BroadcastMessageClass {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
