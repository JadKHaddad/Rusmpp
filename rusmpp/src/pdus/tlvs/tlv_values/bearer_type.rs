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
pub enum BearerType {
    Unknown = 0x00,
    Sms = 0x01,
    Csd = 0x02,
    PacketData = 0x03,
    Ussd = 0x04,
    Cdpd = 0x05,
    DataTac = 0x06,
    FlexReFlex = 0x07,
    CellBroadcast = 0x08,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for BearerType {
    fn default() -> Self {
        BearerType::Unknown
    }
}
