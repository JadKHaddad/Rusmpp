use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum DataCoding {
    #[default]
    McSpesific = 0b00000000,
    Ia5 = 0b00000001,
    OctetUnspecified = 0b00000010,
    Latin1 = 0b00000011,
    OctetUnspecified2 = 0b00000100,
    Jis = 0b00000101,
    Cyrillic = 0b00000110,
    LatinHebrew = 0b00000111,
    Ucs2 = 0b00001000,
    PictogramEncoding = 0b00001001,
    Iso2022JpMusicCodes = 0b00001010,
    ExtendedKanjiJis = 0b00001101,
    Ksc5601 = 0b00001110,
    GsmMwiControl = 0b11000000,
    GsmMwiControl2 = 0b11010000,
    GsmMessageClassControl = 0b11100000,
    Other(u8),
}

impl From<u8> for DataCoding {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => DataCoding::McSpesific,
            0b00000001 => DataCoding::Ia5,
            0b00000010 => DataCoding::OctetUnspecified,
            0b00000011 => DataCoding::Latin1,
            0b00000100 => DataCoding::OctetUnspecified2,
            0b00000101 => DataCoding::Jis,
            0b00000110 => DataCoding::Cyrillic,
            0b00000111 => DataCoding::LatinHebrew,
            0b00001000 => DataCoding::Ucs2,
            0b00001001 => DataCoding::PictogramEncoding,
            0b00001010 => DataCoding::Iso2022JpMusicCodes,
            0b00001101 => DataCoding::ExtendedKanjiJis,
            0b00001110 => DataCoding::Ksc5601,
            0b11000000 => DataCoding::GsmMwiControl,
            0b11010000 => DataCoding::GsmMwiControl2,
            0b11100000 => DataCoding::GsmMessageClassControl,
            value => DataCoding::Other(value),
        }
    }
}

impl From<DataCoding> for u8 {
    fn from(value: DataCoding) -> Self {
        match value {
            DataCoding::McSpesific => 0b00000000,
            DataCoding::Ia5 => 0b00000001,
            DataCoding::OctetUnspecified => 0b00000010,
            DataCoding::Latin1 => 0b00000011,
            DataCoding::OctetUnspecified2 => 0b00000100,
            DataCoding::Jis => 0b00000101,
            DataCoding::Cyrillic => 0b00000110,
            DataCoding::LatinHebrew => 0b00000111,
            DataCoding::Ucs2 => 0b00001000,
            DataCoding::PictogramEncoding => 0b00001001,
            DataCoding::Iso2022JpMusicCodes => 0b00001010,
            DataCoding::ExtendedKanjiJis => 0b00001101,
            DataCoding::Ksc5601 => 0b00001110,
            DataCoding::GsmMwiControl => 0b11000000,
            DataCoding::GsmMwiControl2 => 0b11010000,
            DataCoding::GsmMessageClassControl => 0b11100000,
            DataCoding::Other(value) => value,
        }
    }
}

impl Length for DataCoding {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for DataCoding {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for DataCoding {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
