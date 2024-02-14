use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Npi {
    #[default]
    Unknown = 0b00000000,
    Isdn = 0b00000001,
    Data = 0b00000011,
    Telex = 0b00000100,
    LandMobile = 0b00000110,
    National = 0b00001000,
    Private = 0b00001001,
    Ermes = 0b00001010,
    Internet = 0b00001110,
    WapClientId = 0b00010010,
    Other(u8),
}

impl From<u8> for Npi {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => Npi::Unknown,
            0b00000001 => Npi::Isdn,
            0b00000011 => Npi::Data,
            0b00000100 => Npi::Telex,
            0b00000110 => Npi::LandMobile,
            0b00001000 => Npi::National,
            0b00001001 => Npi::Private,
            0b00001010 => Npi::Ermes,
            0b00001110 => Npi::Internet,
            0b00010010 => Npi::WapClientId,
            value => Npi::Other(value),
        }
    }
}

impl From<Npi> for u8 {
    fn from(value: Npi) -> Self {
        match value {
            Npi::Unknown => 0b00000000,
            Npi::Isdn => 0b00000001,
            Npi::Data => 0b00000011,
            Npi::Telex => 0b00000100,
            Npi::LandMobile => 0b00000110,
            Npi::National => 0b00001000,
            Npi::Private => 0b00001001,
            Npi::Ermes => 0b00001010,
            Npi::Internet => 0b00001110,
            Npi::WapClientId => 0b00010010,
            Npi::Other(value) => value,
        }
    }
}

impl Length for Npi {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for Npi {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for Npi {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
