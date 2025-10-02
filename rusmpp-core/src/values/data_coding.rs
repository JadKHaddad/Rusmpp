use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_test_instances::<DataCoding>();
        crate::tests::borrowed::encode_decode_test_instances::<DataCoding>();
    }
}
