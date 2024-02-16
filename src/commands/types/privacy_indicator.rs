use crate::ende::{
    decode::{Decode, DecodeError},
    encode::{Encode, EncodeError},
    length::Length,
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum PrivacyIndicator {
    #[default]
    NotRestricted = 0,
    Restricted = 1,
    Confidential = 2,
    Secret = 3,
    Other(u8),
}

impl From<u8> for PrivacyIndicator {
    fn from(value: u8) -> Self {
        match value {
            0 => PrivacyIndicator::NotRestricted,
            1 => PrivacyIndicator::Restricted,
            2 => PrivacyIndicator::Confidential,
            3 => PrivacyIndicator::Secret,
            value => PrivacyIndicator::Other(value),
        }
    }
}

impl From<PrivacyIndicator> for u8 {
    fn from(value: PrivacyIndicator) -> Self {
        match value {
            PrivacyIndicator::NotRestricted => 0,
            PrivacyIndicator::Restricted => 1,
            PrivacyIndicator::Confidential => 2,
            PrivacyIndicator::Secret => 3,
            PrivacyIndicator::Other(value) => value,
        }
    }
}

impl Length for PrivacyIndicator {
    fn length(&self) -> usize {
        1
    }
}

impl Encode for PrivacyIndicator {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        u8::from(*self).encode_to(writer)
    }
}

impl Decode for PrivacyIndicator {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let value = Self::from(u8::decode_from(reader)?);

        Ok(value)
    }
}
