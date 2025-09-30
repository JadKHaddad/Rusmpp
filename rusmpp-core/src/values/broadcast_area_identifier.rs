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
pub enum BroadcastAreaFormat {
    #[default]
    AliasName = 0x00,
    EllipsoidArc = 0x01,
    Polygon = 0x02,
    Other(u8),
}

impl From<u8> for BroadcastAreaFormat {
    fn from(value: u8) -> Self {
        match value {
            0x00 => BroadcastAreaFormat::AliasName,
            0x01 => BroadcastAreaFormat::EllipsoidArc,
            0x02 => BroadcastAreaFormat::Polygon,
            value => BroadcastAreaFormat::Other(value),
        }
    }
}

impl From<BroadcastAreaFormat> for u8 {
    fn from(value: BroadcastAreaFormat) -> Self {
        match value {
            BroadcastAreaFormat::AliasName => 0x00,
            BroadcastAreaFormat::EllipsoidArc => 0x01,
            BroadcastAreaFormat::Polygon => 0x02,
            BroadcastAreaFormat::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::owned::encode_decode_test_instances::<BroadcastAreaFormat>();
        crate::tests::borrowed::encode_decode_test_instances::<BroadcastAreaFormat>();
    }
}
