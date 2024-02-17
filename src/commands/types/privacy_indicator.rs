use crate::{
    ende::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    types::u8::EndeU8,
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

impl EndeU8 for PrivacyIndicator {}
