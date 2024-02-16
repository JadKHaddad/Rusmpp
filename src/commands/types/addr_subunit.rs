use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum AddrSubunit {
    #[default]
    Unknown = 0x00,
    MSDisplay = 0x01,
    MobileEquipment = 0x02,
    SmartCard = 0x03,
    ExternalUnit = 0x04,
    Other(u8),
}

impl From<u8> for AddrSubunit {
    fn from(value: u8) -> Self {
        match value {
            0x00 => AddrSubunit::Unknown,
            0x01 => AddrSubunit::MSDisplay,
            0x02 => AddrSubunit::MobileEquipment,
            0x03 => AddrSubunit::SmartCard,
            0x04 => AddrSubunit::ExternalUnit,
            value => AddrSubunit::Other(value),
        }
    }
}

impl From<AddrSubunit> for u8 {
    fn from(value: AddrSubunit) -> Self {
        match value {
            AddrSubunit::Unknown => 0x00,
            AddrSubunit::MSDisplay => 0x01,
            AddrSubunit::MobileEquipment => 0x02,
            AddrSubunit::SmartCard => 0x03,
            AddrSubunit::ExternalUnit => 0x04,
            AddrSubunit::Other(value) => value,
        }
    }
}

impl Length for AddrSubunit {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for AddrSubunit {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for AddrSubunit {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
