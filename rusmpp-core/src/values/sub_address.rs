use rusmpp_macros::Rusmpp;

pub mod borrowed;
#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum SubaddressTag {
    #[default]
    NsapEven = 0b10000000,
    NsapOdd = 0b10001000,
    UserSpecified = 0b10100000,
    Other(u8),
}

impl From<u8> for SubaddressTag {
    fn from(value: u8) -> Self {
        match value {
            0b10000000 => SubaddressTag::NsapEven,
            0b10001000 => SubaddressTag::NsapOdd,
            0b10100000 => SubaddressTag::UserSpecified,
            value => SubaddressTag::Other(value),
        }
    }
}

impl From<SubaddressTag> for u8 {
    fn from(value: SubaddressTag) -> Self {
        match value {
            SubaddressTag::NsapEven => 0b10000000,
            SubaddressTag::NsapOdd => 0b10001000,
            SubaddressTag::UserSpecified => 0b10100000,
            SubaddressTag::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::borrowed::encode_decode_test_instances::<SubaddressTag>();
        crate::tests::owned::encode_decode_test_instances::<SubaddressTag>();
    }
}
