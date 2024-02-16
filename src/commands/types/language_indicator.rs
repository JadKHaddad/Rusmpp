use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

/// Refer to [CMT-136] for other values
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum LanguageIndicator {
    #[default]
    Unspecified = 0,
    English = 1,
    French = 2,
    Spanish = 3,
    German = 4,
    Portuguese = 5,
    Other(u8),
}

impl From<u8> for LanguageIndicator {
    fn from(value: u8) -> Self {
        match value {
            0 => LanguageIndicator::Unspecified,
            1 => LanguageIndicator::English,
            2 => LanguageIndicator::French,
            3 => LanguageIndicator::Spanish,
            4 => LanguageIndicator::German,
            5 => LanguageIndicator::Portuguese,
            value => LanguageIndicator::Other(value),
        }
    }
}

impl From<LanguageIndicator> for u8 {
    fn from(value: LanguageIndicator) -> Self {
        match value {
            LanguageIndicator::Unspecified => 0,
            LanguageIndicator::English => 1,
            LanguageIndicator::French => 2,
            LanguageIndicator::Spanish => 3,
            LanguageIndicator::German => 4,
            LanguageIndicator::Portuguese => 5,
            LanguageIndicator::Other(value) => value,
        }
    }
}

impl Length for LanguageIndicator {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for LanguageIndicator {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for LanguageIndicator {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
