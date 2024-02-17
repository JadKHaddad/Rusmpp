use crate::types::u8::EndeU8;

/// Numeric Plan Indicator.
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

impl EndeU8 for Npi {}
