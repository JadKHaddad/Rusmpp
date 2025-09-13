use crate::types::AnyOctetString;

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

crate::create! {
    /// The broadcast_area_identifier defines the Broadcast Area in terms of a geographical descriptor.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub struct BroadcastAreaIdentifier {
        pub format: BroadcastAreaFormat,
        @[length = unchecked]
        pub area: AnyOctetString,
    }
}

impl BroadcastAreaIdentifier {
    pub fn new(format: BroadcastAreaFormat, area: AnyOctetString) -> Self {
        Self { format, area }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<BroadcastAreaFormat>();
        crate::tests::encode_decode_with_length_test_instances::<BroadcastAreaIdentifier>();
    }
}
