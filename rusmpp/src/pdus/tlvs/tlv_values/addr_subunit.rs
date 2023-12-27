use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::RusmppIoU8;

#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    FromPrimitive,
    RusmppIoU8,
)]
pub enum AddrSubunit {
    Unknown = 0x00,
    MSDisplay = 0x01,
    MobileEquipment = 0x02,
    SmartCard = 0x03,
    ExternalUnit = 0x04,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for AddrSubunit {
    fn default() -> Self {
        AddrSubunit::Unknown
    }
}
