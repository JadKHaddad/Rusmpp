use crate::{
    ende::decode::{DecodeError, DecodeWithLength},
    impl_length_encode, tri,
    types::{octet_string::OctetString, u8::EndeU8},
};

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

impl EndeU8 for BroadcastAreaFormat {}

impl_length_encode! {
    /// Identifies one or more target Broadcast Area(s) for which the
    /// status information applies.
    ///
    /// The number of instances of this parameter will be exactly equal
    /// to the number of occurrences of the broadcast_area_identifiers
    /// parameter in the corresponding broadcast_sm.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    pub struct BroadcastAreaIdentifier {
        pub format: BroadcastAreaFormat,
        pub area: OctetString<0, 100>,
    }
}

impl BroadcastAreaIdentifier {
    pub fn new(format: BroadcastAreaFormat, area: OctetString<0, 100>) -> Self {
        Self { format, area }
    }
}

impl DecodeWithLength for BroadcastAreaIdentifier {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let format = tri!(BroadcastAreaFormat::decode_from(reader));

        let area_length = length.saturating_sub(format.length());

        let area = tri!(OctetString::decode_from(reader, area_length));

        Ok(Self { format, area })
    }
}
