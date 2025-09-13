crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum DataCoding {
        #[default]
        McSpecific = 0b00000000,
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
}

impl From<u8> for DataCoding {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => DataCoding::McSpecific,
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
            DataCoding::McSpecific => 0b00000000,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<DataCoding>();
    }
}
