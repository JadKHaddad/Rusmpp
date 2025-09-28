pub mod borrowed;
#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod owned;

use rusmpp_macros::Rusmpp;

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Rusmpp)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub enum DestFlag {
    #[default]
    SmeAddress = 0x01,
    DistributionListName = 0x02,
    Other(u8),
}

impl From<u8> for DestFlag {
    fn from(value: u8) -> Self {
        match value {
            0x01 => DestFlag::SmeAddress,
            0x02 => DestFlag::DistributionListName,
            value => DestFlag::Other(value),
        }
    }
}

impl From<DestFlag> for u8 {
    fn from(value: DestFlag) -> Self {
        match value {
            DestFlag::SmeAddress => 0x01,
            DestFlag::DistributionListName => 0x02,
            DestFlag::Other(value) => value,
        }
    }
}

impl From<DestFlag> for u32 {
    fn from(value: DestFlag) -> Self {
        u8::from(value).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_test_instances::<DestFlag>();
        crate::tests::borrowed::encode_decode_test_instances::<DestFlag>();
    }
}
