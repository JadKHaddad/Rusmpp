use crate::{
    ende::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum UnitOfTime {
    #[default]
    AsFrquentlyAsPossible = 0x00,
    Seconds = 0x08,
    Minutes = 0x09,
    Hours = 0x0A,
    Days = 0x0B,
    Weeks = 0x0C,
    Months = 0x0D,
    Years = 0x0E,
    Other(u8),
}

impl From<u8> for UnitOfTime {
    fn from(value: u8) -> Self {
        match value {
            0x00 => UnitOfTime::AsFrquentlyAsPossible,
            0x08 => UnitOfTime::Seconds,
            0x09 => UnitOfTime::Minutes,
            0x0A => UnitOfTime::Hours,
            0x0B => UnitOfTime::Days,
            0x0C => UnitOfTime::Weeks,
            0x0D => UnitOfTime::Months,
            0x0E => UnitOfTime::Years,
            value => UnitOfTime::Other(value),
        }
    }
}

impl From<UnitOfTime> for u8 {
    fn from(value: UnitOfTime) -> Self {
        match value {
            UnitOfTime::AsFrquentlyAsPossible => 0x00,
            UnitOfTime::Seconds => 0x08,
            UnitOfTime::Minutes => 0x09,
            UnitOfTime::Hours => 0x0A,
            UnitOfTime::Days => 0x0B,
            UnitOfTime::Weeks => 0x0C,
            UnitOfTime::Months => 0x0D,
            UnitOfTime::Years => 0x0E,
            UnitOfTime::Other(value) => value,
        }
    }
}

impl Length for UnitOfTime {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for UnitOfTime {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for UnitOfTime {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BroadcastFrequencyInterval {
    pub unit: UnitOfTime,
    pub value: u16,
}

impl Length for BroadcastFrequencyInterval {
    fn length(&self) -> usize {
        self.unit.length() + self.value.length()
    }
}

impl Encode for BroadcastFrequencyInterval {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.unit.encode_to(writer));
        tri!(self.value.encode_to(writer));

        Ok(())
    }
}

impl Decode for BroadcastFrequencyInterval {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let unit = tri!(UnitOfTime::decode_from(reader));
        let value = tri!(u16::decode_from(reader));

        Ok(Self { unit, value })
    }
}
