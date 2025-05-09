use crate::types::octet_string::OctetString;

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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
    /// Identifies one or more target Broadcast Area(s) for which the
    /// status information applies.
    ///
    /// The number of instances of this parameter will be exactly equal
    /// to the number of occurrences of the broadcast_area_identifiers
    /// parameter in the corresponding broadcast_sm.
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    pub struct BroadcastAreaIdentifier {
        pub format: BroadcastAreaFormat,
        @[length = unchecked]
        pub area: OctetString<0, 100>,
    }
}

impl BroadcastAreaIdentifier {
    pub fn new(format: BroadcastAreaFormat, area: OctetString<0, 100>) -> Self {
        Self { format, area }
    }
}
