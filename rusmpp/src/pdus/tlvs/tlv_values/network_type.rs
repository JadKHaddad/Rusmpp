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
pub enum NetworkType {
    Unknown = 0x00,
    Gsm = 0x01,
    Ansi136 = 0x02,
    Is95 = 0x03,
    Pdc = 0x04,
    Phs = 0x05,
    IDen = 0x06,
    Amps = 0x07,
    PagingNetwork = 0x08,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for NetworkType {
    fn default() -> Self {
        NetworkType::Unknown
    }
}
