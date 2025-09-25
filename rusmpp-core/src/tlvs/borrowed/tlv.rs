use rusmpp_macros::Rusmpp;

use crate::{
    encode::Length,
    tlvs::{borrowed::TlvValue, tag::TlvTag},
};

/// See module level documentation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct Tlv<'a> {
    tag: TlvTag,
    value_length: u16,
    #[rusmpp(key = tag, length = value_length)]
    value: Option<TlvValue<'a>>,
}

impl<'a> Tlv<'a> {
    pub fn new(value: impl Into<TlvValue<'a>>) -> Self {
        let value = value.into();
        let tag = value.tag();
        let value_length = value.length() as u16;

        Self {
            tag,
            value_length,
            value: Some(value),
        }
    }

    pub const fn tag(&self) -> TlvTag {
        self.tag
    }

    pub const fn value_length(&self) -> u16 {
        self.value_length
    }

    pub const fn value(&'_ self) -> Option<&'_ TlvValue<'_>> {
        self.value.as_ref()
    }
}

impl<'a> From<TlvValue<'a>> for Tlv<'a> {
    fn from(value: TlvValue<'a>) -> Self {
        Self::new(value)
    }
}
